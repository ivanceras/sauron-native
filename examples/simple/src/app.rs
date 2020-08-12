use log::*;
use sauron_native::{
    widget::{attribute::*, event::*, *},
    Attribute, Callback, Component, Event, Node, Value,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct App {
    click_count: u32,
    text: String,
    events: Vec<String>,
    debug: Vec<String>,
    paragraph_text: String,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    ChangeText(String),
    Decrement,
    ParagraphChanged(String),
}

impl App {
    pub fn new(count: u32) -> App {
        App {
            click_count: count,
            text: String::from("Some text"),
            events: vec![],
            debug: vec![],
            paragraph_text: String::from("paragraph text"),
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => {
                self.click_count += 1;
                eprintln!("button is clicked");
            }
            Msg::Decrement => self.click_count -= 1,
            Msg::ChangeText(txt) => {
                self.text = txt;
            }
            Msg::ParagraphChanged(txt) => {
                self.text = txt.to_string();
                self.paragraph_text = txt;
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        column(
            vec![],
            vec![
                menu_bar(
                    vec![],
                    vec![
                        menu_item(
                            vec![],
                            vec![
                                text_label(vec![value("File")]),
                                menu(
                                    vec![],
                                    vec![
                                        menu_item(
                                            vec![],
                                            vec![text_label(vec![value(
                                                "Open",
                                            )])],
                                        ),
                                        menu_item(
                                            vec![],
                                            vec![text_label(vec![value(
                                                "Close",
                                            )])],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        menu_item(
                            vec![],
                            vec![text_label(vec![value("About")])],
                        ),
                        menu_item(
                            vec![],
                            vec![text_label(vec![value("Quit")])],
                        ),
                    ],
                ),
                header_bar(
                    vec![],
                    vec![
                        button(vec![label("Header button1")]),
                        button(vec![label("Header button2")]),
                    ],
                ),
                column(
                    vec![],
                    vec![
                        textarea(vec![
                            value(self.text.clone()),
                            on_input(|input| {
                                Msg::ParagraphChanged(
                                    input
                                        .value
                                        .as_str()
                                        .expect("must be a string")
                                        .to_owned(),
                                )
                            }),
                        ]),
                        button(vec![label("btn1")]),
                        button(vec![label("btn2")]),
                        textarea(vec![value(self.paragraph_text.clone())]),
                    ],
                ),
            ],
        )
    }
}
