pub mod app;
pub mod app_data;
#[cfg(feature = "ssr")]
pub mod server_main;
pub mod task_format;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
