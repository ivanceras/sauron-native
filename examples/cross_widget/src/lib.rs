use app::App;
use caesar::browser::*;
use wasm_bindgen::prelude::*;

mod app;
mod store;

#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;
    pub static global_js: GlobalJS;
    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

#[wasm_bindgen]
impl Client {
    pub fn new(initial_state: &str) -> Client {
        let root_node = document().get_element_by_id("web-app").unwrap();

        let app = App::new(1);

        let widget_dom = app.view();
        let browser_dom: caesar::browser::Node = widget_dom.into();
        let dom_updater = DomUpdater::new_replace_mount(browser_dom, root_node);
        let mut client = Client { app, dom_updater };
        client.init_subscrption();
        client
    }

    /// set up the app.store
    /// whenever there is a changes to the store
    /// the app.update function will be called
    pub fn init_subscrption(&mut self) {
        self.app.subscribe(Box::new(|| {
            global_js.update();
        }));
    }

    pub fn render(&mut self) {
        caesar::browser::log("in render function");
        self.app.update();
        let vdom = self.app.view();
        self.dom_updater.update(vdom.into());
    }
}

#[wasm_bindgen]
pub fn initialize() -> Client {
    Client::new("initial state")
}
