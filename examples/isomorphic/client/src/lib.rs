//#![deny(warnings)]
use console_error_panic_hook;
use html_backend::html::attributes::*;
use html_backend::html::events::*;
use html_backend::html::*;
use html_backend::*;
use vdom::builder::*;
use vdom::Event;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    dom_updater: DomUpdater,
    vdom: vdom::Node,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        console::log_1(&format!("What to do with this initial state: {}", initial_state).into());

        let html = div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text("Hello world!")]),
                div(
                    [],
                    [button(
                        [onclick(|v: Event| {
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
                                oninput(|v: Event| {
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
                                oninput(|v: Event| {
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
                                onchange(|v: Event| {
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
