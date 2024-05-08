use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn hydrate() {
    #[allow(unused)]
    use leptos::*;
    #[allow(unused)]
    use markdown_islands::*;

    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
