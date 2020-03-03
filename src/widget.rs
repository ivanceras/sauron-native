use crate::{Attribute, Node};
use sauron_vdom::builder::element;
use std::fmt::Debug;

/// TODO: Each widget variant will need to have more details
///  such as attributes, that will be converted to their
///  corresponding target widget of each platform
///
/// Widget definitions
/// This will have a counterparts for each of the supported
/// different platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    Vbox,
    Hbox,
    Button,
    Text(String),
    TextInput(String),
    Checkbox(String, bool),
    Radio(String, bool),
    Image(Vec<u8>),
}

pub fn widget<MSG>(
    widget: Widget,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element(widget, attrs, children)
}

pub fn column<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Vbox, attrs, children)
}

pub fn row<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Hbox, attrs, children)
}

pub fn button<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Button, attrs, vec![])
}

pub fn text<MSG>(txt: &str) -> Node<MSG> {
    widget(Widget::Text(txt.to_string()), vec![], vec![])
}

pub fn text_input<MSG>(attrs: Vec<Attribute<MSG>>, txt: &str) -> Node<MSG> {
    widget(Widget::TextInput(txt.to_string()), attrs, vec![])
}

pub fn checkbox<MSG>(label: &str, checked: bool) -> Node<MSG> {
    widget(Widget::Checkbox(label.to_string(), checked), vec![], vec![])
}

pub fn radio<MSG>(label: &str, checked: bool) -> Node<MSG> {
    widget(Widget::Radio(label.to_string(), checked), vec![], vec![])
}

pub fn image<MSG>(image: Vec<u8>) -> Node<MSG> {
    widget(Widget::Image(image), vec![], vec![])
}
