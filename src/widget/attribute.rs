use crate::{event::on, AttribKey, Attribute, Callback, Event, Value};

pub fn value<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    attr(AttribKey::Value, v)
}

pub fn label<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    attr(AttribKey::Label, v)
}

pub fn on_input<C, MSG>(c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
{
    on(AttribKey::InputEvent, c)
}

pub fn on_click<C, MSG>(c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
{
    on(AttribKey::ClickEvent, c)
}

pub fn attr<V, MSG>(name: AttribKey, v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    crate::builder::attr(name, v)
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
