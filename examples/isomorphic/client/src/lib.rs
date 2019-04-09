//#![deny(warnings)]
#![deny(clippy::all)]
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::*;
use console_error_panic_hook;
use js_sys::{Array, Date};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Mutex;
use vdom::builder::*;
use vdom::Event;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::console;
use web_sys::{Document, Element, Window};

use app::App;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use lazy_static::lazy_static;

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}



// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}


#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        console::log_1(&format!("What to do with this initial state: {}", initial_state).into());

        let root_node = document()
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();

        let app = App::new(1);

        let dom_updater = DomUpdater::new_replace_mount(app.view(), root_node);
        Client { app, dom_updater }
    }

    pub fn render(&mut self) {
        console::log_1(&"in render function".into());
        let vdom = self.app.view();
        self.dom_updater.update(vdom);
    }
}


