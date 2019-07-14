use app::{Model, Msg};
#[cfg(feature = "with-html")]
use sauron::body;
#[cfg(feature = "with-html")]
use sauron_native::{backend::HtmlBackend, Component, Program};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod app;

#[cfg(feature = "with-html")]
#[wasm_bindgen]
pub fn main() {
    let program: Rc<Program<Model, Msg, HtmlBackend<Model, Msg>>> = Program::new(Model::new());
}
