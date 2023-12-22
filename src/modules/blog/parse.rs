use pulldown_cmark::{html::push_html, CowStr, Event, HeadingLevel, LinkType, Parser, Tag};

use crate::utils::vec_set::VecSet;

use super::images::Filename;

#[derive(Debug)]
pub struct BlogParse {
    pub title: String,
    pub content: String,
    pub images: VecSet<String>,
}

/// Modifies the url of an image
pub trait ImageUrlInjector {
    fn inject(&self, url: &mut CowStr<'_>);
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Invalid title")]
    InvalidTitle,
}

fn mutate_item(
    item: &mut Event<'_>,
    images: &mut VecSet<String>,
    injector: &impl ImageUrlInjector,
) {
    if let Event::Start(Tag::Image(LinkType::Inline, url, _)) = item {
        if Filename::new(&url).is_err() {
            return;
        }

        images.insert(url.to_string());
        injector.inject(url);
    }
}

pub fn parse(markdown: &str, injector: impl ImageUrlInjector) -> Result<BlogParse, Error> {
    let mut parser = Parser::new(markdown);
    let mut title = String::new();

    let first = parser.next();

    if !matches!(
        first,
        Some(Event::Start(Tag::Heading(HeadingLevel::H1, ..)))
    ) {
        return Err(Error::InvalidTitle);
    }

    push_html(&mut title, first.into_iter());

    // It should always emit a end of title
    for event in parser.by_ref() {
        match &event {
            Event::End(Tag::Heading(HeadingLevel::H1, ..)) => {
                push_html(&mut title, Some(event).into_iter());
                break;
            }
            Event::Text(_) => {
                push_html(&mut title, Some(event).into_iter());
            }
            _ => {
                return Err(Error::InvalidTitle);
            }
        };
    }

    if title.is_empty() {
        return Err(Error::InvalidTitle);
    }

    let mut content = String::new();

    let mut images = VecSet::default();
    let parser = parser.map(|mut item| {
        mutate_item(&mut item, &mut images, &injector);
        item
    });

    push_html(&mut content, parser);

    Ok(BlogParse {
        title,
        content,
        images,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    struct NoopInjector;
    impl ImageUrlInjector for NoopInjector {
        fn inject(&self, _url: &mut CowStr<'_>) {}
    }

    #[test]
    fn validates_title_is_first() {
        let markdown = r#"This is not a title
# Hello guorld
"#;

        let parsed = parse(markdown, NoopInjector {});

        assert_eq!(parsed.unwrap_err(), Error::InvalidTitle);
    }

    #[test]
    fn validates_title_cointains_only_text() {
        let markdown = "# Hello  ![world](image.png) peace";
        let parsed = parse(markdown, NoopInjector {});

        assert_eq!(parsed.unwrap_err(), Error::InvalidTitle);
    }

    #[test]
    fn collects_images() {
        let markdown = r#"# Hello guorld 
![image](image.png)
Hello
![bruda](./bruda.png)"#;

        let BlogParse { images, .. } = parse(markdown, NoopInjector {}).unwrap();

        assert_eq!(images.into_inner(), vec!["image.png".to_string()]);
    }
}
