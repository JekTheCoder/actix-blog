pub fn copy_to_clipboard(text: &str) {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;

        let Some(clipboard) = window().navigator().clipboard() else {
            leptos::logging::warn!("No clipboard available");
            return;
        };

        let _ = clipboard.write_text(text);
    }

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = text;
    }
}
