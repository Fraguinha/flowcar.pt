pub mod app;
pub mod auth;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod server;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
use leptos::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
