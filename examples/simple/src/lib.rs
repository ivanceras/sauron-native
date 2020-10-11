#![deny(warnings)]
#[cfg(feature = "with-web")]
use sauron_native::backend::HtmlApp;
#[cfg(feature = "with-web")]
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-web")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    use sauron_native::Backend;
    console_log::init_with_level(log::Level::Debug).ok();
    log::trace!("Initial state: {}", initial_state);
    HtmlApp::init(app::App::new(1));
}
