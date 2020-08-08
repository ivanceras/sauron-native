//! widget module provides a unified abstract representation of the widget
//! which has a mapping to the actual widget in the supported backend
//!
use crate::widget::attribute::style;
use crate::widget::attribute::util::find_value;
use crate::{AttribKey, Attribute, Node, Value};
pub use attribute::event;
pub use builder::*;
use mt_dom::{attr, element};
use std::fmt::Debug;
use stretch::geometry::Size;
use stretch::style::Dimension;
use stretch::style::FlexDirection;
use stretch::style::PositionType;
use stretch::style::Style;

pub mod attribute;
mod builder;
pub(crate) mod layout;

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
    /// TODO: rename to cardbox
    Overlay,
    /// groupbox
    GroupBox,
    /// headerbar, such as in gtk where it can contains
    /// menu buttons
    HeaderBar,
    /// menu bar
    MenuBar,
    /// menu
    Menu,
    /// menu item
    MenuItem,
    /// Search entry
    SearchInput,
}

impl Widget {
    /// whether or not the widget can container children or not
    pub fn is_container(&self) -> bool {
        match self {
            Widget::Vbox
            | Widget::Hbox
            | Widget::Vpane
            | Widget::Hpane
            | Widget::Overlay
            | Widget::GroupBox
            | Widget::HeaderBar
            | Widget::MenuBar
            | Widget::Menu
            | Widget::MenuItem => true,

            Widget::SearchInput
            | Widget::Button
            | Widget::Label
            | Widget::Paragraph
            | Widget::TextInput
            | Widget::Checkbox
            | Widget::Radio
            | Widget::Image
            | Widget::Svg
            | Widget::TextArea => false,
        }
    }
}
