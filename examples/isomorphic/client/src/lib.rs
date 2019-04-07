//#![deny(warnings)]
use console_error_panic_hook;
use ui_backend::html::attributes::*;
use ui_backend::html::events::*;
use ui_backend::html::*;
use ui_backend::*;
use vdom::builder::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::console;

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
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text("Hello world!")]),
                div(
                    [],
                    [button(
                        [onclick(|v| {
                            console::log_1(
                                &format!("I've been clicked and the value is: {:#?}", v).into(),
                            );
                        })],
                        [text("Click me!")],
                    )],
                ),
                div(
                    [],
                    [
                        text("Using oninput"),
                        input(
                            [
                                r#type("text"),
                                oninput(|v| {
                                    console::log_1(&format!("input has input: {:#?}", v).into());
                                }),
                                placeholder("Type here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("using oninput on a textarea"),
                        textarea(
                            [
                                oninput(|v| {
                                    console::log_1(
                                        &format!("textarea has changed: {:#?}", v).into(),
                                    );
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("Using onchange"),
                        input(
                            [
                                r#type("text"),
                                onchange(|v| {
                                    console::log_1(&format!("input has changed: {:#?}", v).into());
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
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
        console::log_1(&"updating render function".into());
        let vdom = self.vdom.clone();
        self.dom_updater.update(vdom);
    }
}
