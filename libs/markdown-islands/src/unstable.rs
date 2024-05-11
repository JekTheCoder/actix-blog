use leptos::window;

pub fn copy_to_clipboard(text: &str) {
    #[cfg(feature = "hydrate")]
    {
        let Some(clipboard) = window().navigator().clipboard() else {
            leptos::logging::warn!("No clipboard available");
            return;
        };

        let _ = clipboard.write_text(&text);
    }
}
