use leptos::{island, view, Children, IntoView};

#[island]
pub fn CodeBlock(language: Option<String>, children: Children) -> impl IntoView {
    view! {
        <pre><code>{children()}</code>
        <!-- "generated!" -->
        </pre>
    }
}
