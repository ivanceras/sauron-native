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
    platform: Option<Platform>,
    distribution: Vec<DistributionMedium>,
}

#[derive(Debug)]
pub enum Platform {
    Linux,
    Mac,
    Windows,
    Android,
    Ios,
}

impl Platform {
    fn all() -> Vec<Self> {
        vec![
            Platform::Linux,
            Platform::Mac,
            Platform::Windows,
            Platform::Android,
            Platform::Ios,
        ]
    }
}

#[derive(Debug)]
pub enum DistributionMedium {
    NativeGui,
    TextUi,
    HtmlUi,
}

impl DistributionMedium {
    fn all() -> Vec<Self> {
        vec![
            DistributionMedium::NativeGui,
            DistributionMedium::TextUi,
            DistributionMedium::HtmlUi,
        ]
    }
}

#[derive(Debug)]
pub enum Msg {
    Click,
    Decrement,
    PlatformSelect(Platform),
    DistributionSelect(DistributionMedium),
    Nothing,
}

impl App {
    pub fn new() -> App {
        App {
            click_count: 4,
            platform: None,
            distribution: vec![],
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
            Msg::PlatformSelect(selection) => {
                self.platform = Some(selection);
            }
            Msg::DistributionSelect(medium) => {}
            Msg::Nothing => {}
        }
    }

    fn view(&self) -> Node<Msg> {
        column(
            vec![],
            vec![
                button(vec![label("Target platform:")]),
                column(
                    vec![],
                    Platform::all()
                        .iter()
                        .map(|name| radio(vec![label(format!("{:?}", name))]))
                        .collect(),
                ),
                button(vec![label("Distribute as:")]),
                column(
                    vec![],
                    DistributionMedium::all()
                        .iter()
                        .map(|name| radio(vec![label(format!("{:?}", name))]))
                        .collect(),
                ),
                button(vec![
                    on_click(|_| Msg::Click),
                    label(format!("More buttons: current({})", self.click_count)),
                ]),
                button(vec![on_click(|_| Msg::Decrement), label("Less buttons")]),
                row(vec![], {
                    (0..self.click_count)
                        .map(|x| button(vec![label("Hello".to_string())]))
                        .collect()
                }),
                textarea(vec![value(
                    "This is a paragraph\n\
                                This is a paragraph line 1\n\
                                This is a paragraph line 2\n\
                                This is a paragraph line 3\n\
                                This is a paragraph line 4\n\
                                This is a paragraph line 5\n\
                                This is a paragraph line 6\n\
                        ",
                )]),
            ],
        )
    }
}
