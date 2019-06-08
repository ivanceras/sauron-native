use app::{App, Msg};
#[cfg(feature = "with-html")]
use sauron::body;
#[cfg(feature = "with-html")]
use sauron_native::{backend::HtmlBackend, Component, Program};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    sauron::log!("Initial state: {}", initial_state);
    let program: Rc<Program<App, Msg, HtmlBackend<App, Msg>>> = Program::new(App::new(1));
}
