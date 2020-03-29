use crate::{AttribKey, Attribute, Node};
use control::{Button, Checkbox, TextInput};
use sauron_vdom::{builder::element, Callback, Event};
use std::fmt::Debug;

pub mod attribute;
mod control;

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
    Paragraph,
    TextInput,
    Checkbox,
    Radio,
    Image,
    Svg,
    TextArea,
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

pub fn paragraph<MSG>(txt: &str) -> Node<MSG> {
    widget(Widget::Paragraph, vec![attribute::value(txt.to_string())], vec![])
}

pub fn text_input<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::TextInput, attrs, vec![])
}

pub fn checkbox<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Checkbox, attrs, vec![])
}

pub fn radio<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Radio, attrs, vec![])
}

pub fn image<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Image, attrs, vec![])
}

pub fn svg<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Svg, attrs, vec![])
}

pub fn textarea<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::TextArea, attrs, vec![])
}
