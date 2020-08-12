#![deny(warnings)]
#[cfg(feature = "with-html")]
use sauron_native::backend::HtmlApp;
#[cfg(feature = "with-html")]
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    console_log::init_with_level(Level::Debug);
    trace!("Initial state: {}", initial_state);
    HtmlApp::init(app::App::new(1));
}
