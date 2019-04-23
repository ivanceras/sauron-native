use app::{App, Msg};
use sauron::body;
use sauron_ui::{backend::HtmlBackend, Component, Program};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod app;

#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    sauron::log!("Initial state: {}", initial_state);
    let program: Rc<Program<App, Msg, HtmlBackend<App, Msg>>> = Program::new(App::new(1));
}
