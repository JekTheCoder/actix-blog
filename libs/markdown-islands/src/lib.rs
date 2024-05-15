use leptos::{component, create_node_ref, island, svg::view, view, Children, IntoView};

mod unstable;

#[island]
pub fn CodeBlock(language: Option<String>, children: Children) -> impl IntoView {
    let code_el = create_node_ref::<leptos::html::Code>();

    let on_copy = move |_| {
        let text = code_el.get_untracked().unwrap().inner_text();
        unstable::copy_to_clipboard(&text);
    };

    view! {
        <div class="code-block">
            <div class="code-header">
                <span>{language}</span>
                <button class="copy-btn" on:click=on_copy>
                    <iconify-icon icon="bxs:copy" />
                </button>
            </div>
            <pre class="code">
                <code ref={code_el}>{children()}</code>
            </pre>
        </div>
    }
}

#[component]
pub fn InlineLink(href: String, title: String, children: Children) -> impl IntoView {
    if is_url(&href) {
        view! {
            <a href={href} title={title} target="_blank" rel="noopener noreferrer">{children()}</a>
        }
    } else {
        view! {
            <a href={href} title={title}>{children()}</a>
        }
    }
}

fn is_url(text: &str) -> bool {
    const PROTOCOL_SEPARATOR: &'static str = "://";

    text.find(PROTOCOL_SEPARATOR)
        .is_some_and(|i| i > 0 && (i + PROTOCOL_SEPARATOR.len()) < text.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_url() {
        assert!(is_url("https://google.com"));
    }

    #[test]
    fn check_not_url() {
        assert!(!is_url("google.com"));
    }

    #[test]
    fn check_has_trailing() {
        assert!(!is_url("https://"));
    }

    #[test]
    fn check_has_one_trailing() {
        assert!(is_url("https://a"));
    }

    #[test]
    fn check_has_protocol() {
        assert!(!is_url("://google.com"));
    }
}
