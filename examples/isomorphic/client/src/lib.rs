#![deny(warnings)]
#![deny(clippy::all)]
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::*;
use console_error_panic_hook;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
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
    app: App,
    pub dom_updater: DomUpdater,
}

pub struct State {
    click_count: Rc<Cell<u32>>,
    listeners: Vec<Box<(Fn() -> () + 'static)>>,
}

pub struct App {
    pub store: Rc<RefCell<State>>,
}

#[derive(Debug)]
pub enum Msg {
    Click,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        console::log_1(&format!("What to do with this initial state: {}", initial_state).into());

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();

        let app = App::new(1);
        let dom_updater = DomUpdater::new_replace_mount(app.view(), root_node);

        let mut client = Client { app, dom_updater };
        client.update();
        client
    }

    fn update(&mut self) {
        self.app.store.borrow_mut().subscribe(Box::new(|| {
            web_sys::console::log_1(&"Updating state from client".into());
            //TODO: make it possible to call here
            //self.dom_updater.update(self.app.view());
        }));
    }
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            click_count: Rc::new(Cell::new(count)),
            listeners: vec![],
        }
    }

    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
    }

    pub fn msg(&mut self, msg: &Msg) {
        console::log_1(&format!("Msg got here: {:?}", msg).into());
        match msg {
            Msg::Click => self.increment_click(),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn click_count(&self) -> u32 {
        self.click_count.get()
    }

    fn increment_click(&mut self) {
        self.click_count.set(self.click_count.get() + 1);
    }
}

impl App {
    fn new(count: u32) -> App {
        let mut state = State::new(count);
        state.subscribe(Box::new(|| {
            web_sys::console::log_1(&"Updating state".into());
        }));
        let store = Rc::new(RefCell::new(state));

        App { store }
    }

    fn view(&self) -> vdom::Node {
        let store_clone = Rc::clone(&self.store);
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text("Hello world!")]),
                div(
                    [],
                    [button(
                        [onclick(move |v: Event| {
                            console::log_1(
                                &format!("I've been clicked and the value is: {:#?}", v).into(),
                            );
                            store_clone.borrow_mut().msg(&Msg::Click);
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
        )
    }
}
