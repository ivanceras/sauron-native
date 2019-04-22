use app::App;
use sauron::body;
use sauron_ui::{Component, Program};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod app;
use app::Msg;

#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    sauron::log!("Initial state: {}", initial_state);
    Program::new(App::new(1));
}
