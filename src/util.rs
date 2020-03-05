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
