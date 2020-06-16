use log::*;
use sauron_native::{
    event::{on, InputEvent},
    widget::{attribute::*, *},
    Attribute, Callback, Component, Event, Node, Program, Value,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct App {
    click_count: u32,
    text: String,
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
    pub fn new() -> App {
        App {
            click_count: 4,
            #[cfg(feature = "with-titik")]
            text: String::from("Press CTRL-q / CTRL-c to exit"),
            #[cfg(not(feature = "with-titik"))]
            text: String::from("Some text"),
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
            Msg::Decrement => {
                if self.click_count > 0 {
                    self.click_count -= 1;
                }
            }
            Msg::ChangeText(txt) => {
                self.text = txt;
            }
            Msg::ParagraphChanged(txt) => {
                self.paragraph_text = txt;
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        column(
            vec![],
            vec![
                button(vec![on_click(|_| Msg::Decrement), label(&self.text)]),
                button(vec![
                    on_click(|_| Msg::Click),
                    label(format!("Hello: {}", self.click_count)),
                ]),
                checkbox(vec![label("Checkbox1"), value(true)]),
                checkbox(vec![label("Checkbox2"), value(false)]),
                checkbox(vec![label("Checkbox3"), value(false)]),
                radio(vec![label("Radio1"), value(true)]),
                radio(vec![label("Radio2"), value(false)]),
                row(vec![], {
                    (0..self.click_count)
                        .map(|x| button(vec![label("Hello".to_string())]))
                        .collect()
                }),
                textarea(vec![
                    value(
                        "This is a paragraph\n\
                                This is a paragraph line 1\n\
                                This is a paragraph line 2\n\
                                This is a paragraph line 3\n\
                                This is a paragraph line 4\n\
                                This is a paragraph line 5\n\
                                This is a paragraph line 6\n\
                        ",
                    ),
                    on_input(|input| Msg::ParagraphChanged(input.value)),
                    height(7.0),
                ]),
                textarea(vec![value("This is another text area")]),
            ],
        )
    }
}
