//! widget module provides a unified abstract representation of the widget
//! which has a mapping to the actual widget in the supported backend
//!
use crate::{Attribute, Node};
pub use attribute::event;
use mt_dom::element;
use std::fmt::Debug;

pub mod attribute;

/// TODO: Each widget variant will need to have more details
///  such as attributes, that will be converted to their
///  corresponding target widget of each platform
///
/// Widget definitions
/// This will have a counterparts for each of the supported
/// different platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    /// vertical flexbox
    Vbox,
    /// horizontal flexbox
    Hbox,
    /// vertical resizable flexbox
    Vpane,
    /// horizontal resizable flexbox
    Hpane,
    /// a button widget
    Button,
    /// a text label
    Label,
    /// text paragraph
    Paragraph,
    /// text input
    TextInput,
    /// checkbox
    Checkbox,
    /// radio control
    Radio,
    /// image widget
    Image,
    /// svg widget
    Svg,
    /// textarea widget
    TextArea,
    /// an overlay widget
    Overlay,
    /// groupbox
    GroupBox,
}

/// a helper function to create widget elements
pub fn widget<MSG>(
    widget: Widget,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG> {
    element(widget, attrs, children)
}

/// a vertically oriented flexbox
pub fn column<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Vbox, attrs, children)
}

/// create a horizontally oriented flexbox
pub fn row<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Hbox, attrs, children)
}

/// create a vertically oriented resizable flexbox
pub fn vpane<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Vpane, attrs, children)
}

/// create a horizontally oriented resizable flexbox
pub fn hpane<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Hpane, attrs, children)
}

/// overlay can be on top of other widgets
pub fn overlay<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::Overlay, attrs, children)
}

/// group widges together will a visible label and border enclosure
pub fn groupbox<MSG>(attrs: Vec<Attribute<MSG>>, children: Vec<Node<MSG>>) -> Node<MSG> {
    widget(Widget::GroupBox, attrs, children)
}

/// create a button
pub fn button<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Button, attrs, vec![])
}

/// create a text paragraph
pub fn paragraph<MSG>(txt: &str) -> Node<MSG> {
    widget(
        Widget::Paragraph,
        vec![attribute::value(txt.to_string())],
        vec![],
    )
}

/// create a text input
pub fn text_input<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::TextInput, attrs, vec![])
}

/// create a checkbox control
pub fn checkbox<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Checkbox, attrs, vec![])
}

/// create a radio control
pub fn radio<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Radio, attrs, vec![])
}

/// create an image control
pub fn image<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Image, attrs, vec![])
}

/// create an image control from svg
pub fn svg<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Svg, attrs, vec![])
}

/// create a text area
pub fn textarea<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::TextArea, attrs, vec![])
}

/// create a text label
pub fn text_label<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG> {
    widget(Widget::Label, attrs, vec![])
}
