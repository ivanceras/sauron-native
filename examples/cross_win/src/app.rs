use log::*;
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
    Decrement,
}

impl App {
    pub fn new(count: u32) -> App {
        App {
            click_count: count,
            #[cfg(feature = "with-tui")]
            text: String::from("CTRL-C to exit"),
            #[cfg(not(feature = "with-tui"))]
            text: String::from("Some text"),
            events: vec![],
            debug: vec![],
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::Decrement => self.click_count -= 1,
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
        column(
            vec![],
            vec![
                button(vec![
                    onclick(|_| Msg::Click),
                    value(format!("Hello: {}", self.click_count)),
                ]),
                checkbox("Checkbox1", true),
                checkbox("Checkbox2", true),
                checkbox("Checkbox3", true),
                radio("Radio1", true),
                radio("Radio2", false),
                text_input(
                    vec![oninput(|event: Event| match event {
                        Event::InputEvent(input) => Msg::ChangeText(input.value),
                        _ => {
                            trace!("This is unexpected: {:#?}", event);
                            panic!();
                        }
                    })],
                    &self.events.join("\n"),
                ),
                image(include_bytes!("../horse.jpg").to_vec()),
                text(
                    "Hello, will this be a paragraph
The quick brown fox jumps over the lazy
dog. Lorem ipsum",
                ),
            ],
        )
    }
}
