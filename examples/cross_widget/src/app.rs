use sauron::html::attributes::*;
use sauron_ui::{
    backend::html::widget_tree_to_html_node, event::on, widget::*, Component, Node, Program,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

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
        row(
            [],
            [
                column(
                    [class("column1")],
                    [
                        button([], "column1 element1"),
                        button([], "column1 element2"),
                        button([], "column1 element3"),
                        button([], "column1 element4"),
                        button([], "column1 element5"),
                        button([], "column1 element6"),
                    ],
                ),
                column(
                    [class("column2")],
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
                text("Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     "),
            ],
        )
    }
}
