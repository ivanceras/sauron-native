#[cfg(feature = "with-html")]
use sauron_native::backend::html::widget_tree_to_html_node;
use sauron_native::{event::on, widget::*, Component, Node, Program};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use sauron_native::{event::InputEvent, util::*, Attribute, Callback, Event, Value};

pub struct App {
    click_count: u32,
    text: String,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    ChangeText(String),
}

impl App {
    pub fn new(count: u32) -> App {
        App {
            click_count: count,
            text: String::new(),
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        println!("updating in App");
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::ChangeText(txt) => {
                println!("text changed to {}", txt);
                self.text = txt;
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        vbox(
            vec![],
            vec![
                hbox(
                    vec![attr("class", "column1"), onclick(|_| Msg::Click)],
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
                        button(vec![value(&self.text)]),
                    ],
                ),
                button(vec![
                    onclick(|_| {
                        sauron::log("Button is clicked!");
                        Msg::Click
                    }),
                    value(format!("Hello: {}", self.click_count)),
                ]),
                block("I'm a block kid!"),
                textbox(
                    vec![oninput(|event: Event| match event {
                        Event::InputEvent(input) => Msg::ChangeText(input.value),
                        _ => {
                            sauron::log!("This is unexpected: {:#?}", event);
                            panic!();
                        }
                    })],
                    "a textbox",
                ),
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
