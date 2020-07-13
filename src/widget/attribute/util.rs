//! utility functions for manipulating attributes
//!
use crate::{widget::attribute::AttribKey, Attribute, Callback, Event};

/// find the value of the attribute key from a Vec of attributes
pub fn find_value<MSG>(key: AttribKey, attrs: &[Attribute<MSG>]) -> Option<&sauron_vdom::Value>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| *att.name() == key)
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
        .find(|att| *att.name() == key)
        .map(|att| att.get_callback())
        .flatten()
}

/// find the Scrollable attribute boolean value, default is false
pub fn is_scrollable<MSG: 'static>(attrs: &[Attribute<MSG>]) -> bool {
    find_value(AttribKey::Scrollable, attrs)
        .map(|v| v.as_bool())
        .flatten()
        .unwrap_or(false)
}
