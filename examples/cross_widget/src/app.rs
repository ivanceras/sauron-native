#[cfg(feature = "with-html")]
use sauron::html::attributes::*;
#[cfg(feature = "with-html")]
use sauron_native::backend::html::widget_tree_to_html_node;
use sauron_native::{event::on, widget::*, Component, Node, Program};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use sauron_native::{Attribute, Callback, Event, Value};

pub fn connect<C, MSG>(event: &'static str, c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
    MSG: Clone,
{
    on(event, c)
}

pub fn attr<V, MSG>(name: &'static str, v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    sauron_native::builder::attr(name, v)
}

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
                        button(vec![], "column1 element1"),
                        button(vec![], "column1 element2"),
                        button(vec![], "column1 element3"),
                        button(vec![], "column1 element4"),
                        button(vec![], "column1 element5"),
                        button(vec![], "column1 element6"),
                    ],
                ),
                hbox(
                    vec![attr("class", "column2")],
                    vec![button(vec![], "column2"), button(vec![], "c2 element2")],
                ),
                button(
                    vec![on("click", |_| {
                        sauron::log("Button is clicked!");
                        Msg::Click
                    })],
                    &format!("Hello: {}", self.click_count),
                ),
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
