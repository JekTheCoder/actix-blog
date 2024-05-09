pub struct MarkdownParser {
    runtime: leptos::RuntimeId,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            runtime: leptos::create_runtime(),
        }
    }

    pub fn push_parse<'a>(
        &mut self,
        buffer: &mut String,
        parser: impl Iterator<Item = Event<'a>>,
    ) {
        extend_parse(buffer, parser)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MarkdownParser {
    fn drop(&mut self) {
        self.runtime.dispose();
    }
}

use std::collections::VecDeque;

use leptos::IntoView;
use pulldown_cmark::Event;

use markdown_islands::CodeBlock;

pub fn extend_parse<'a>(buffer: &mut String, parser: impl Iterator<Item = Event<'a>>) {
    let mut element_events = VecDeque::new();
    let mut in_element = false;

    let elements_iter = parser.flat_map(|e| match e {
        Event::Start(tag) => {
            element_events.push_front(Event::Start(tag));
            in_element = true;

            MDEvents::none()
        }
        Event::End(tag) => {
            element_events.push_front(Event::End(tag.clone()));
            in_element = false;

            match tag {
                pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(
                    language,
                )) => {
                    let children = element_events
                        .iter()
                        .filter_map(|e| match e {
                            Event::Text(text) => Some(match text {
                                pulldown_cmark::CowStr::Borrowed(text) => *text,
                                pulldown_cmark::CowStr::Boxed(text) => text.as_ref(),
                                pulldown_cmark::CowStr::Inlined(text) => text.as_ref(),
                            }),
                            _ => None,
                        })
                        .collect::<String>();

                    let children = vec![children
                        .trim()
                        .trim_end_matches("```")
                        .to_owned()
                        .into_view()];
                    let children = Box::new(move || leptos::Fragment::new(children));

                    element_events.clear();
                    let event = pulldown_cmark::Event::Html(
                        CodeBlock(markdown_islands::CodeBlockProps {
                            children,
                            language: Some(match language {
                                pulldown_cmark::CowStr::Boxed(language) => language.to_string(),
                                pulldown_cmark::CowStr::Inlined(language) => {
                                    language.to_string()
                                }
                                pulldown_cmark::CowStr::Borrowed(language) => {
                                    language.to_owned()
                                }
                            }),
                        })
                        .into_view()
                        .render_to_string()
                        .to_string()
                        .into(),
                    );

                    MDEvents::one(event)
                }
                _ => {
                    let events = MDEvents::many(element_events.clone());
                    element_events.clear();

                    events
                }
            }
        }
        _ => {
            if in_element {
                element_events.push_front(e);
                MDEvents::none()
            } else {
                MDEvents::one(e)
            }
        }
    });

    pulldown_cmark::html::push_html(buffer, elements_iter);
}

enum MDEvents<'a> {
    Event(Option<Event<'a>>),
    Events(VecDeque<Event<'a>>),
}

impl<'a> MDEvents<'a> {
    fn none() -> Self {
        Self::Event(None)
    }

    fn one(e: Event<'a>) -> Self {
        Self::Event(Some(e))
    }

    fn many(e: VecDeque<Event<'a>>) -> Self {
        Self::Events(e)
    }
}

struct MDEventsIter<'a>(MDEvents<'a>);

impl<'a> Iterator for MDEventsIter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            MDEvents::Event(e) => e.take(),
            MDEvents::Events(e) => e.pop_back(),
        }
    }
}

impl<'a> IntoIterator for MDEvents<'a> {
    type Item = Event<'a>;
    type IntoIter = MDEventsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MDEventsIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(content: &str) -> String {
        let mut buffer = String::default();
        MarkdownParser::new().push_parse(&mut buffer, pulldown_cmark::Parser::new(content));

        buffer
    }

    #[test]
    pub fn does_not_panic() {
        let _ = parse(
            r#"```javascript
this is CodeBlock```"#,
        );
    }

    #[test]
    pub fn it_parses() {
        let res = parse(
            r#"```javascript
this is CodeBlock```"#,
        );

        println!("{}", res);
    }
}

