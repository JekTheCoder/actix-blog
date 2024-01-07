use pulldown_cmark::{html::push_html, CowStr, Event, HeadingLevel, LinkType, Parser, Tag};

use crate::shared::vec_set::VecSet;

use super::{images::Filename, value_objects::preview::Preview};

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
        if Filename::new(url).is_err() {
            return;
        }

        images.insert(url.to_string());
        injector.inject(url);
    }
}

pub fn parse(markdown: &str, injector: &impl ImageUrlInjector) -> Result<BlogParse, Error> {
    let mut parser = Parser::new(markdown);
    let mut title_elements = vec![];

    let Some(first) = parser.next() else {
        return Err(Error::InvalidTitle);
    };

    if !matches!(first, Event::Start(Tag::Heading(HeadingLevel::H1, ..))) {
        return Err(Error::InvalidTitle);
    }

    title_elements.push(first);

    // It should always emit a end of title
    for event in parser.by_ref() {
        match &event {
            Event::End(Tag::Heading(HeadingLevel::H1, ..)) => {
                title_elements.push(event);
                break;
            }
            Event::Text(_) => {
                title_elements.push(event);
            }
            _ => {
                return Err(Error::InvalidTitle);
            }
        };
    }

    let title = title_elements
        .iter()
        .cloned()
        .fold(String::new(), |mut title, event| {
            if let Event::Text(text) = event {
                title.push_str(&text);
            }

            title
        });

    if title.is_empty() {
        return Err(Error::InvalidTitle);
    }

    let mut content = String::new();
    let mut images = VecSet::default();

    push_html(&mut content, title_elements.into_iter());

    let parser = parser.map(|mut item| {
        mutate_item(&mut item, &mut images, injector);
        item
    });

    push_html(&mut content, parser);

    Ok(BlogParse {
        title,
        content,
        images,
    })
}

pub fn parse_preview(markdown: &str) -> Option<Box<Preview>> {
    let (preview_start, _) = lines_indices::LinesIndices::new(markdown)
        .find(|&(_, line)| Parser::new(line).take(40).all(|event| is_readable(&event)))?;

    let mut preview = String::new();
    let mut preview_iter = Parser::new(&markdown[preview_start..]).take(40);

    let first = preview_iter.by_ref().next();

    let rest = preview_iter.by_ref().take_while(|event| {
        matches!(
            event,
            Event::Text(_)
                | Event::Start(Tag::Strong | Tag::Emphasis | Tag::Link(_, _, _))
                | Event::End(Tag::Strong | Tag::Emphasis | Tag::Paragraph | Tag::Link(_, _, _))
        )
    });

    push_html(&mut preview, first.into_iter().chain(rest));

    Some(unsafe { Preview::from_boxed_unchecked(preview.into_boxed_str()) })
}

fn is_readable(event: &Event<'_>) -> bool {
    macro_rules! readable_tags {
        () => {
            Tag::Strong | Tag::Emphasis | Tag::Paragraph | Tag::Link(_, _, _)
        };
    }
    matches!(
        event,
        Event::Text(_) | Event::Start(readable_tags!()) | Event::End(readable_tags!())
    )
}

mod lines_indices {
    pub struct LinesIndices<'a> {
        str: &'a str,
        last_index: usize,
        char_indices: std::str::CharIndices<'a>,
    }

    impl<'a> LinesIndices<'a> {
        pub fn new(str: &'a str) -> Self {
            Self {
                str,
                last_index: 0,
                char_indices: str.char_indices(),
            }
        }
    }

    impl<'a> Iterator for LinesIndices<'a> {
        type Item = (usize, &'a str);

        fn next(&mut self) -> Option<Self::Item> {
            let (i, _) = self.char_indices.by_ref().find(|item| item.1 == '\n')?;
            let current_index = self.last_index;
            self.last_index = i + 1;

            Some((current_index, &self.str[current_index..i]))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn yields_indices() {
            let mut lines = LinesIndices::new("hello\nworld\n");
            assert_eq!(lines.next(), Some((0, "hello")));
            assert_eq!(lines.next(), Some((6, "world")));
        }
    }
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

        let parsed = parse(markdown, &NoopInjector {});

        assert_eq!(parsed.unwrap_err(), Error::InvalidTitle);
    }

    #[test]
    fn validates_title_cointains_only_text() {
        let markdown = "# Hello  ![world](image.png) peace";
        let parsed = parse(markdown, &NoopInjector {});

        assert_eq!(parsed.unwrap_err(), Error::InvalidTitle);
    }

    #[test]
    fn collects_images() {
        let markdown = r#"# Hello guorld 
![image](image.png)
Hello
![bruda](./bruda.png)"#;

        let BlogParse { images, .. } = parse(markdown, &NoopInjector {}).unwrap();

        assert_eq!(images.into_inner(), vec!["image.png".to_string()]);
    }

    #[test]
    fn collects_preview() {
        let markdown = r#"# hello world

how are you, my friends?

![bruda](./bruda.png)"#;

        let preview = parse_preview(markdown);
        assert_eq!(preview.unwrap().as_ref(), "<p>how are you, my friends?</p>\n");
    }

    #[test]
    fn should_get_preview() {
        let markdown = r#"# Hello my brodas

This is an interesting preview

![image](wosi.png)

## Hello my brodas

This is more content"#;

        let preview = parse_preview(markdown);
        assert_eq!(preview.unwrap().as_ref(), "<p>This is an interesting preview</p>\n");
    }

    #[test]
    fn content_includes_title() {
        let markdown = r#"# Hello my brodas

This is an interesting preview"#;

        let BlogParse { content, .. } = parse(markdown, &NoopInjector {}).unwrap();

        assert_eq!(
            content,
            "<h1>Hello my brodas</h1>\n<p>This is an interesting preview</p>\n"
        );
    }

    #[test]
    fn can_get_a_clean_title() {
        let markdown = "# Hello my brodas";
        let BlogParse { title, .. } = parse(markdown, &NoopInjector {}).unwrap();

        assert_eq!(title, "Hello my brodas");
    }
}
