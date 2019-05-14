use sauron::html::attributes::*;
use sauron_native::{
    backend::html::widget_tree_to_html_node, event::on, widget::*, Component, Node, Program,
};
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
            [],
            [
                hbox(
                    [
                        attr("class", "column1"),
                        on("click", |_| Msg::Click),
                        connect("click", |_| Msg::Click),
                    ],
                    [
                        button([], "column1 element1"),
                        button([], "column1 element2"),
                        button([], "column1 element3"),
                        button([], "column1 element4"),
                        button([], "column1 element5"),
                        button([], "column1 element6"),
                    ],
                ),
                hbox(
                    [attr("class", "column2")],
                    [button([], "column2"), button([], "c2 element2")],
                ),
                button(
                    [on("click", |_| {
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
