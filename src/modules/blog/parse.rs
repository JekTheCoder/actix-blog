mod inclusive_take_until;
mod peek_take_until;

use inclusive_take_until::InclusiveTakeUntil;
use peek_take_until::PeekTakeUntil;
use pulldown_cmark::{html::push_html, Event, HeadingLevel, Parser, Tag};

pub struct BlogParse {
    pub title: String,
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty title")]
    EmptyTitle,
}

pub fn parse(markdown: &str) -> Result<BlogParse, Error> {
    let mut title = String::new();
    let mut content = String::new();

    let mut parser = Parser::new(markdown);

    {
        let mut peekable = parser.by_ref().peekable();

        let until_title = InclusiveTakeUntil::new(&mut peekable, |item: &_| {
            matches!(item, Event::Start(Tag::Heading(HeadingLevel::H1, ..)))
        });

        push_html(&mut content, until_title);

        let until_title_end = PeekTakeUntil::new(&mut peekable, |item: &_| {
            matches!(item, Event::End(Tag::Heading(HeadingLevel::H1, ..)))
        });

        push_html(&mut title, until_title_end);
    }

    content.push_str(&title);
    push_html(&mut content, parser);

    if title.is_empty() {
        return Err(Error::EmptyTitle);
    }

    Ok(BlogParse { title, content })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_pre_title() {
        let markdown = r#"PreTitle!!
# Hello world"#;

        let mut content = String::new();
        let mut parser = Parser::new(markdown);

        let mut peekable = parser.by_ref().peekable();
        {
            let content: &mut String = &mut content;
            let parser = &mut peekable;
            let until_title = PeekTakeUntil::new(parser, |item: &_| {
                matches!(item, Event::Start(Tag::Heading(HeadingLevel::H1, ..)))
            });

            push_html(content, until_title);
        };

        assert_eq!(content, "<p>PreTitle!!</p>\n");
    }

    #[test]
    fn can_parse_title() {
        let markdown = r#"PreTitle!!
# Hello world"#;

        let mut pre_title = String::new();
        let mut parser = Parser::new(markdown);

        let mut peekable = parser.by_ref().peekable();
        {
            let content = &mut pre_title;
            let parser = &mut peekable;
            let until_title = PeekTakeUntil::new(parser, |item: &_| {
                matches!(item, Event::Start(Tag::Heading(HeadingLevel::H1, ..)))
            });

            push_html(content, until_title);
        };

        let mut content = String::new();
        push_html(&mut content, peekable);

        assert_eq!(content, "<h1>Hello world</h1>\n");
    }

    #[test]
    fn parses_until_title() {
        let markdown = r#"# Hello world
Foo bar"#;

        let mut title = String::new();
        let mut parser = Parser::new(markdown);

        {
            let content = &mut title;
            let parser = &mut parser;
            let until_title_end = InclusiveTakeUntil::new(parser, |item: &_| {
                matches!(item, Event::End(Tag::Heading(HeadingLevel::H1, ..)))
            });

            push_html(content, until_title_end);
        };

        let mut content = String::new();
        push_html(&mut content, parser);

        assert_eq!(title, "<h1>Hello world</h1>");
        assert_eq!(content, "\n<p>Foo bar</p>\n");
    }
}
