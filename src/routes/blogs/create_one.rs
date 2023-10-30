use actix_web::{post, web::Data, Responder};

use crate::{
    modules::{admin::AdminId, blog, db::Pool},
    shared::extractors::valid_json::ValidJson,
    sqlx::void_insert_response,
};

use blog_parser::{parse, BlogParse};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub content: String,
}

#[post("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    req: ValidJson<Request>,
    AdminId { id }: AdminId,
) -> impl Responder {
    let Request { content } = req.as_ref();

    let BlogParse {
        title,
        content: html_content,
    } = parse(content).expect("Foo");

    let result = blog::create(pool.get_ref(), id, &title, content, &html_content).await;
    void_insert_response(result)
}

mod blog_parser {
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

        let mut parser = Parser::new(&markdown);

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

    mod peek_take_until {
        use std::iter::Peekable;

        pub struct PeekTakeUntil<'a, I, F> {
            iter: &'a mut I,
            take_until: F,
        }

        impl<'a, I, F> PeekTakeUntil<'a, I, F> {
            pub fn new(iter: &'a mut I, take_until: F) -> Self {
                Self { iter, take_until }
            }
        }

        impl<'a, I, F> Iterator for PeekTakeUntil<'a, Peekable<I>, F>
        where
            I: Iterator,
            F: Fn(&I::Item) -> bool,
        {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                let item = self.iter.peek()?;
                let has_ended = (self.take_until)(item);

                if has_ended {
                    None
                } else {
                    self.iter.next()
                }
            }
        }
    }

    mod inclusive_take_until {
        pub struct InclusiveTakeUntil<'a, I, F> {
            iter: &'a mut I,
            take_until: F,
            has_ended: bool,
        }

        impl<'a, I, F> InclusiveTakeUntil<'a, I, F> {
            pub fn new(iter: &'a mut I, take_until: F) -> Self {
                Self {
                    iter,
                    take_until,
                    has_ended: false,
                }
            }
        }

        impl<'a, I, F> Iterator for InclusiveTakeUntil<'a, I, F>
        where
            I: Iterator,
            F: Fn(&I::Item) -> bool,
        {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                if self.has_ended {
                    return None;
                }

                let item = self.iter.next()?;
                self.has_ended = (self.take_until)(&item);

                Some(item)
            }
        }
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
                let parser= &mut parser;
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
}
