//#![deny(warnings)]
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::DomUpdater;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Client {
    _dom_updater: DomUpdater,
}

/// Build using
/// ```sh
/// $ wasm-pack build --target no-modules
/// ```
///
impl Client {

    pub fn new() -> Client {
        let html = div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [input(
                [
                    class("client"),
                    r#type("button"),
                    value("Click me!"),
                    onclick(|_| {
                        browser::log("i've been clicked");
                    }),
                ],
                [],
            )],
        );
        browser::log("hello from here!");
        let body = browser::body();
        let _dom_updater = DomUpdater::new_append_to_mount(html, &body);
        Client { _dom_updater }
    }
}


#[wasm_bindgen]
pub fn initialize() -> Client {
    let client = Client::new();
    client
}
