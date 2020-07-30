//! utility functions for manipulating attributes
//!
use crate::{widget::attribute::AttribKey, Attribute, Callback, Element, Node, Value};
use mt_dom::AttValue;
use stretch::result::Layout;
use stretch::style::Style;

/// find the value of the attribute key from a Vec of attributes
pub fn find_value<MSG>(key: AttribKey, attrs: &[Attribute<MSG>]) -> Option<&Value>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| *att.name() == key)
        .map(|att| att.get_plain().first().map(|v| *v))
        .flatten()
}

/// find the callback of the attribute key from a Vec of attributes
pub fn find_callback<MSG>(key: AttribKey, attrs: &[Attribute<MSG>]) -> Option<Vec<&Callback<MSG>>>
where
    MSG: 'static,
{
    attrs
        .iter()
        .find(|att| *att.name() == key)
        .map(|att| att.get_callback())
}

/// find the Scrollable attribute boolean value, default is false
pub fn is_scrollable<MSG: 'static>(attrs: &[Attribute<MSG>]) -> bool {
    find_value(AttribKey::Scrollable, attrs)
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

/// return the first style attribute of this node
pub fn get_style<MSG>(node: &Node<MSG>) -> Option<&Style> {
    node.get_attribute_value(&AttribKey::Style)
        .map(|values| values.first().map(|value| value.as_style()))
        .flatten()
        .flatten()
}

pub fn get_layout<MSG>(element: &Element<MSG>) -> Option<&Layout> {
    element
        .get_attribute_value(&AttribKey::Layout)
        .map(|values| values.first().map(|value| value.as_layout()))
        .flatten()
        .flatten()
}
