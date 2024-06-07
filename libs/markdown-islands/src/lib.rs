use leptos::{component, create_node_ref, island, create_signal, view, Children, IntoView};

mod unstable;

mod feedback {
    use leptos::{component, view, IntoView, ReadSignal, Show, SignalGet};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum PopupState {
        Opened,
        Closing,
        #[default]
        Closed,
    }

    #[component]
    pub fn Feedback(opened: ReadSignal<PopupState>, text: &'static str) -> impl IntoView {
        view! {
            <>
                <Show when=move || opened.get() != PopupState::Closed>
                    <div class="feedback" class:closing={move || opened.get() == PopupState::Closing}>
                        {text}
                    </div>
                </Show>
            </>
        }
    }
}

#[island]
pub fn CodeBlock(language: Option<String>, children: Children) -> impl IntoView {
    use feedback::Feedback;
    use leptos::{set_timeout, SignalSet};

    let code_el = create_node_ref::<leptos::html::Code>();
    let (opeend, set_opened) = create_signal(feedback::PopupState::default());

    let on_copy = move |_| {
        let text = code_el.get_untracked().unwrap().inner_text();
        unstable::copy_to_clipboard(&text);

        set_opened.set(feedback::PopupState::Opened);
        set_timeout(
            move || {
                set_opened.set(feedback::PopupState::Closing);
                set_timeout(
                    move || {
                        set_opened.set(feedback::PopupState::Closed);
                    },
                    std::time::Duration::from_millis(100),
                );
            },
            std::time::Duration::from_millis(500),
        );
    };

    view! {
        <div class="code-block">
            <div class="code-header">
                <span>{language}</span>
                <div class="copy">
                    <Feedback opened={opeend} text="Copied!" />

                    <button class="copy-btn" on:click=on_copy>
                        <iconify-icon icon="tdesign:copy" />
                    </button>
                </div>
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
    const PROTOCOL_SEPARATOR: &str = "://";

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
