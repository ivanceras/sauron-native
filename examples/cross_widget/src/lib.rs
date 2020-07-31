use app::{App, Msg};
use log::*;
use sauron_native::backend::Backend;
#[cfg(feature = "with-html")]
use sauron_native::{backend::HtmlApp, Component};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("must init");
    trace!("Initial state: {}", initial_state);
    let app = App::new();
    HtmlApp::init(app);
}
