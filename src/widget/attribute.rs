use crate::builder::attr;
use crate::{event::on, Attribute, Callback, Event, Value};
use std::fmt;

/// TODO: replace the &'static str attribute key as an enum
/// enumerating all the properties of our widget abstraction
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
    ClickEvent,
    InputEvent,
    Key,
    /// data, used in image blobs and svg
    Data,
    Height,
    Width,
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn find_value<MSG>(key: AttribKey, attrs: &Vec<Attribute<MSG>>) -> Option<&sauron_vdom::Value>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| att.name == key)
        .map(|att| att.get_value())
        .flatten()
}

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

macro_rules! declare_event_attr {
    (
        $(
            $(#[$attr:meta])*
            $fname:ident => $att_key:tt;
        )*
    ) => {

        $(
            $(#[$attr])*
            pub fn $fname<V,MSG>(v: V) -> Attribute<MSG>
                where V:Into<Callback<Event,MSG>>,
            {
                on(AttribKey::$att_key, v)
            }
        )*
    }
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
}

declare_event_attr! {
    on_input => InputEvent;
    on_click => ClickEvent;
}
