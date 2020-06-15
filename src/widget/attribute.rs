use crate::builder::attr;
use crate::event::{InputEvent, MouseEvent};
use crate::{event::on, Attribute, Callback, Event, Value};
use std::fmt;

/// declare an attribute to be used as a function call
macro_rules! declare_attr {
    (
        $(
            $(#[$attr:meta])*
            $fname:ident => $att_key:tt;
        )*
    ) => {

        $(
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
    Key,
    /// whether or not the control is editable, used in text_view
    Editable,
    /// data, used in image blobs and svg
    Data,
    Height,
    Width,
    /// svg image data attribute used in button
    SvgImage,

    /// Events
    ClickEvent,
    MouseDown,
    MouseUp,
    MouseMove,
    InputEvent,
    /// whether or not a widget is scrollable, such as image, text_area
    Scrollable,
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
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// find the value of the attribute key from a Vec of attributes
pub fn find_value<MSG>(key: AttribKey, attrs: &[Attribute<MSG>]) -> Option<&sauron_vdom::Value>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| att.name == key)
        .map(|att| att.get_value())
        .flatten()
}

/// find the callback of the attribute key from a Vec of attributes
pub fn find_callback<MSG>(
    key: AttribKey,
    attrs: &Vec<Attribute<MSG>>,
) -> Option<&Callback<Event, MSG>>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| att.name == key)
        .map(|att| att.get_callback())
        .flatten()
}

pub fn on_click<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::ClickEvent, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

pub fn on_mousedown<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseDown, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

pub fn on_mouseup<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseUp, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

pub fn on_mousemove<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseMove, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

pub fn on_input<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(InputEvent) -> MSG + 'static,
{
    on(AttribKey::InputEvent, move |ev: Event| match ev {
        Event::InputEvent(input) => func(input),
        _ => unreachable!(),
    })
}
