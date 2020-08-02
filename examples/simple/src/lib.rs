use app::{App, Msg};
use log::*;
#[cfg(feature = "with-html")]
use sauron_native::backend::HtmlApp;
use sauron_native::Backend;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    console_log::init_with_level(Level::Debug);
    trace!("Initial state: {}", initial_state);
    HtmlApp::init(App::new(1));
}
