//! Provides functions for attributes of sauron native widgets
//!
pub use super::event::Event;
use crate::Attribute;
use mt_dom::attr;
use std::fmt;
use stretch::style::Style;
pub use util::{find_callback, find_value, get_style};
pub use value::Value;

pub mod util;
mod value;

/// declare an attribute to be used as a function call
macro_rules! declare_attr {
    (
        $(
            $(#[$attr:meta])*
            $fname:ident => $att_key:tt;
        )*
    ) => {

        $(
            /// creates an attribute with the function name as the attribute key
            $(#[$attr])*
            pub fn $fname<V,MSG>(v: V) -> Attribute<MSG>
                where V:Into<Value>,
            {
                attr(AttribKey::$att_key, v.into())
            }
        )*
    }
}

/// These are attribute keys used in sauron-native, which will be translated to their
/// corresponding backends
#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum AttribKey {
    /// String, used in text_input
    Value,
    /// String, used in button, label, checkbox, radio
    Label,
    /// bool, used in checkbox, radio
    Checked,
    /// Alignment Enum, used in hbox and vbox
    Alignment,
    /// If the key differ in the diff, all of the subtree will be discarded
    Key,
    /// widget id
    Id,
    /// whether or not the control is editable, used in text_view
    Editable,
    /// data, used in image blobs and svg
    Data,
    /// svg image data attribute used in button
    SvgImage,
    /// The style attribute
    Style,
    /// whether or not a widget is scrollable, such as image, text_area
    Scrollable,
    /// the calculated layout of this widget
    Layout,
    /// explicit width specified to the widget
    Width,
    /// explicit height specified to the widget
    Height,
    /// Position type of widgets whether absolute or relative
    PositionType,
    /// whether an item is resizable or not, used in Hpaned
    Resizable,
    /// whether to use a monospace font or not
    Monospace,
    /// whether the UI is selectable or not
    Selectable,
    /// whether to assume the content to be pre-formatted or not
    Preformatted,
    /// widgets can have a name and can be styled
    Name,
    /// Uri used in link buttons
    Uri,
    /// Placeholder is used in text input, search input, and text_area
    Placeholder,

    /// Events
    ClickEvent,
    /// Mouse down event
    MouseDown,
    /// Mouse up event
    MouseUp,
    /// Mouse move event
    MouseMove,
    /// Input event
    InputEvent,
    /// keyboard events
    KeyEvent,
    /// doubleclick event
    DoubleClickEvent,
    /// on blur event
    BlurEvent,
    /// activate event, used in on_enter
    Activate,
    /// For
    For,
}

declare_attr! {
    /// value attribute, used in text_input, textarea
    value => Value;
    /// data attribute, used in image, svg
    data => Data;
    /// label attribute, used in button, checkbox and radio
    label => Label;
    /// height attribute, used in most widgets
    /// svg_image attribute, used in buttons
    svg_image => SvgImage;
    /// editable attribute
    editable => Editable;
    /// scrollable attribute
    scrollable => Scrollable;
    /// specified width
    width => Width;
    /// specified height
    height => Height;
    /// resizable item
    resizable => Resizable;
    /// monospace
    monospace => Monospace;
    /// selectable
    selectable => Selectable;
    /// preformatted
    preformatted => Preformatted;
    /// name
    name => Name;
    /// uri
    uri => Uri;
    /// checked
    checked => Checked;
    /// placeholder
    placeholder => Placeholder;
    /// key
    key => Key;
    /// id
    id => Id;
    /// for
    for_ => For;
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// create a style attribute from the style in stretch crate
pub(crate) fn style<MSG>(style: Style) -> Attribute<MSG> {
    attr(AttribKey::Style, Value::Style(style))
}
