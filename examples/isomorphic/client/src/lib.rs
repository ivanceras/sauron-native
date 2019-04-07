use console_error_panic_hook;
use std::rc::Rc;
use ui_backend::html::attributes::*;
use ui_backend::html::events::*;
use ui_backend::html::*;
use ui_backend::*;
use vdom::builder::*;
use vdom::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::Element;

#[wasm_bindgen]
pub struct Client {
    dom_updater: DomUpdater,
    vdom: vdom::Node,
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
    pub fn new(_initial_state: &str) -> Client {
        console_error_panic_hook::set_once();

        let html = div(
            [
                class("some-class"),
                id("some-id"),
                onclick(|_| {
                    println!("clicked");
                }),
                attr("data-id", 1),
                on("mouseover", |_| {
                    println!("i've been clicked");
                }),
            ],
            [
                text("Hello world!"),
                input([r#type("button"), class("client"), value("Click me!")], []),
                button([], [text("Other button")]),
            ],
        );

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();
        let dom_updater = DomUpdater::new_replace_mount(html.clone(), root_node);

        Client {
            dom_updater,
            vdom: html,
        }
    }

    pub fn render(&mut self) {
        self.dom_updater.update(self.vdom.clone());
    }
}
