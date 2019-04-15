use crate::store::Msg;
use crate::store::Store;
use sauron_ui::event::on;
use sauron_ui::widget::*;
use sauron_ui::Node;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(count: u32) -> App {
        App {
            store: Rc::new(RefCell::new(Store::new(count))),
        }
    }

    pub fn update(&mut self) {}

    pub fn view(&self) -> Node {
        let click_count = self.store.borrow().click_count();
        let store = self.store.clone();
        let vnode = row(
            [],
            [
                column([], []),
                button(
                    [on("click", move |_| {
                        sauron_ui::browser::log("hello");
                        store.borrow_mut().msg(&Msg::Click)
                    })],
                    &format!("Hello: {}", click_count),
                ),
            ],
        );
        Node(vnode)
    }

    pub fn subscribe(&mut self, callback: Box<Fn()>) {
        self.store.borrow_mut().subscribe(callback);
    }
}
