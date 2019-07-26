use sauron_native::{
    event::{on, InputEvent},
    util::*,
    widget::*,
    Attribute, Callback, Component, Event, Node, Program, Value,
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
            text: String::from("CTRL-C to exit"),
            events: vec![],
            debug: vec![],
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::ChangeText(txt) => {
                self.text = txt;
            }
        }
    }

    fn on_event(&mut self, event: Event) {
        //self.events.push(format!("{:?}", event));
    }

    fn debug(&mut self, s: String) {
        //self.debug.push(s);
    }

    fn view(&self) -> Node<Msg> {
        vbox(
            vec![],
            vec![
                vbox(
                    vec![attr("class", "column2")],
                    vec![
                        text(&self.debug.join("\n")),
                        button(vec![value(&self.text)]),
                    ],
                ),
                button(vec![
                    onclick(|_| Msg::Click),
                    value(format!("Hello: {}", self.click_count)),
                ]),
                textbox(
                    vec![oninput(|event: Event| match event {
                        Event::InputEvent(input) => Msg::ChangeText(input.value),
                        _ => {
                            sauron::log!("This is unexpected: {:#?}", event);
                            panic!();
                        }
                    })],
                    &self.events.join("\n"),
                ),
                text(
                    "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                     ",
                ),
            ],
        )
    }
}
