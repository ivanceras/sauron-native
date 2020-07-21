use app::{App, Msg};
use log::*;
#[cfg(feature = "with-html")]
use sauron_native::{backend::HtmlBackend, Component, Program};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("must init");
    trace!("Initial state: {}", initial_state);
    let program: Rc<Program<App, Msg, HtmlBackend<App, Msg>>> = Program::new(App::new());
}
