use leptos::{create_node_ref, island, view, Children, IntoView};

mod unstable;

#[island]
pub fn CodeBlock(language: Option<String>, children: Children) -> impl IntoView {
    let code_el = create_node_ref::<leptos::html::Code>();

    let on_copy = move |_| {
        let text = code_el.get_untracked().unwrap().inner_text();
        unstable::copy_to_clipboard(&text);
    };

    view! {
        <pre>
            <div class="code-header">
                <span>{language}</span>
                <button class="copy-btn" on:click=on_copy>
                    <iconify-icon icon="bxs:copy" />
                </button>
            </div>
            <code ref={code_el}>{children()}</code>
        </pre>
    }
}
