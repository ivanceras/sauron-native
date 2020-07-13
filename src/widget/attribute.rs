//! Provides functions for attributes of sauron native widgets
//!
use crate::{
    builder::attr,
    event::{on, InputEvent, MouseEvent},
    Attribute, Event, Value,
};
use std::fmt;
pub use util::{find_callback, find_value};

pub mod util;

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
                attr(AttribKey::$att_key, v)
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
    /// whether or not the control is editable, used in text_view
    Editable,
    /// data, used in image blobs and svg
    Data,
    /// height of widget
    Height,
    /// width of widgets
    Width,
    /// svg image data attribute used in button
    SvgImage,
    /// The style attribute
    Style,
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
    /// whether or not a widget is scrollable, such as image, text_area
    Scrollable,
    /// position of the Hpane, Vpane
    Position,
}

declare_attr! {
    /// value attribute, used in text_input, textarea
    value => Value;
    /// data attribute, used in image, svg
    data => Data;
    /// label attribute, used in button, checkbox and radio
    label => Label;
    /// height attribute, used in most widgets
    height => Height;
    /// width attribute, used in most widgets
    width => Width;
    /// svg_image attribute, used in buttons
    svg_image => SvgImage;
    /// editable attribute
    editable => Editable;
    /// scrollable attribute
    scrollable => Scrollable;
    position => Position;
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// create an attribute which attach a callback to the on_click event
pub fn on_click<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::ClickEvent, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousedown event
pub fn on_mousedown<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseDown, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mouseup event
pub fn on_mouseup<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseUp, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousemove event
pub fn on_mousemove<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseMove, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousemove event
pub fn on_input<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(InputEvent) -> MSG + 'static,
{
    on(AttribKey::InputEvent, move |ev: Event| match ev {
        Event::InputEvent(input) => func(input),
        _ => unreachable!(),
    })
}
