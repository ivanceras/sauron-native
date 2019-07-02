#[cfg(feature = "with-html")]
use sauron::html::attributes::*;
#[cfg(feature = "with-html")]
use sauron_native::backend::html::widget_tree_to_html_node;
use sauron_native::{event::on, widget::*, Component, Node, Program};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use sauron_native::{util::*, Attribute, Callback, Event, Value};

pub struct App {
    click_count: u32,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
}

impl App {
    pub fn new(count: u32) -> App {
        App { click_count: count }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        println!("updating in App");
        match msg {
            Msg::Click => self.click_count += 1,
        }
    }

    fn view(&self) -> Node<Msg> {
        vbox(
            vec![],
            vec![
                hbox(
                    vec![
                        attr("class", "column1"),
                        on("click", |_| Msg::Click),
                        connect("click", |_| Msg::Click),
                    ],
                    vec![
                        button(vec![value("column1 element1")]),
                        button(vec![value("column1 element2")]),
                        button(vec![value("column1 element3")]),
                        button(vec![value("column1 element4")]),
                        button(vec![value("column1 element5")]),
                        button(vec![value("column1 element6")]),
                    ],
                ),
                hbox(
                    vec![attr("class", "column2")],
                    vec![
                        button(vec![value("column2")]),
                        button(vec![value("c2 element2")]),
                    ],
                ),
                button(vec![
                    on("click", |_| {
                        sauron::log("Button is clicked!");
                        Msg::Click
                    }),
                    value(format!("Hello: {}", self.click_count)),
                ]),
                block("I'm a block kid!"),
                text(
                    "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     ",
                ),
            ],
        )
    }
}
