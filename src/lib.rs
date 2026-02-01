pub mod app;

#[cfg(feature = "ssr")]
pub mod app_data;

#[cfg(feature = "ssr")]
pub mod server_main;

pub mod achievements;
mod requests;
mod unclaimed_achievements;
mod update_achievement;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
