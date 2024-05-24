mod lines_indices;

use pulldown_cmark::{html::push_html, CowStr, Event, HeadingLevel, LinkType, Parser, Tag};

use crate::{component_parse::MarkdownParser, vec_set::VecSet};

#[derive(Debug)]
pub struct BlogParse {
    pub title: String,
    pub content: String,
    pub images: VecSet<String>,
}

/// Modifies the url of an image
pub trait ImageUrlInjector {
    fn inject(&self, url: &mut CowStr<'_>);
    fn is_valid(&self, url: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidTitle,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTitle => write!(f, "Invalid title"),
        }
    }
}

impl std::error::Error for Error {}

fn mutate_item(
    item: &mut Event<'_>,
    images: &mut VecSet<String>,
    injector: &impl ImageUrlInjector,
) {
    if let Event::Start(Tag::Image(LinkType::Inline, url, _)) = item {
        if !injector.is_valid(url) {
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

    let mut md_parser = MarkdownParser::new();

    let mut content = String::new();
    let mut images = VecSet::default();

    md_parser.push_parse(&mut content, title_elements.into_iter());

    let parser = parser.map(|mut item| {
        mutate_item(&mut item, &mut images, injector);
        item
    });

    md_parser.push_parse(&mut content, parser);

    Ok(BlogParse {
        title,
        content,
        images,
    })
}

pub struct PreviewParse {
    pub preview: String,
    pub description: String,
}

pub fn parse_preview(markdown: &str) -> Option<PreviewParse> {
    let (preview_start, _) = lines_indices::LinesIndices::new(markdown)
        .find(|&(_, line)| Parser::new(line).take(40).all(|event| is_readable(&event)))?;

    let mut preview_iter = Parser::new(&markdown[preview_start..]).take(40);

    let first = preview_iter.by_ref().next();

    let rest = preview_iter.by_ref().take_while(|event| {
        matches!(
            event,
            Event::Text(_)
                | Event::Code(_)
                | Event::Start(Tag::Strong | Tag::Emphasis | Tag::Link(_, _, _))
                | Event::End(Tag::Strong | Tag::Emphasis | Tag::Paragraph | Tag::Link(_, _, _))
        )
    });

    let events = first.into_iter().chain(rest).collect::<Vec<_>>();

    let mut preview = String::new();
    let description = events
        .iter()
        .filter_map(|event| match event {
            Event::Text(text) => Some(text),
            Event::Code(text) => Some(text),
            Event::SoftBreak => Some(&pulldown_cmark::CowStr::Borrowed("  ")),
            Event::FootnoteReference(text) => Some(text),
            _ => None,
        })
        .map(|text| text.as_ref())
        .collect::<String>();

    push_html(&mut preview, events.into_iter());

    Some(PreviewParse {
        preview,
        description,
    })
}

fn is_readable(event: &Event<'_>) -> bool {
    macro_rules! readable_tags {
        () => {
            Tag::Strong | Tag::Emphasis | Tag::Paragraph | Tag::Link(_, _, _)
        };
    }
    matches!(
        event,
        Event::Text(_)
            | Event::Code(_)
            | Event::Start(readable_tags!())
            | Event::End(readable_tags!())
    )
}

#[cfg(test)]
mod test {
    use super::*;

    struct NoopInjector;
    impl ImageUrlInjector for NoopInjector {
        fn is_valid(&self, _: &str) -> bool {
            true
        }
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
        assert_eq!(
            preview.unwrap().preview,
            "<p>how are you, my friends?</p>\n"
        );
    }

    #[test]
    fn should_get_preview() {
        let markdown = r#"# Hello my brodas

This is an interesting preview

![image](wosi.png)

## Hello my brodas

This is more content"#;

        let preview = parse_preview(markdown);
        assert_eq!(
            preview.unwrap().preview,
            "<p>This is an interesting preview</p>\n"
        );
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

    #[test]
    fn compile_plain_description() {
        let markdown = r#"Hello, *world*!"#;
        let super::PreviewParse { description, .. } = super::parse_preview(markdown).unwrap();
        assert_eq!(description, "Hello, world!");
    }
}
