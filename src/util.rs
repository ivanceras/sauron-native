use crate::{event::on, Attribute, Callback, Event, Value};

pub fn value<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    attr("value", v)
}

pub fn connect<C, MSG>(event: &'static str, c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
    MSG: Clone,
{
    on(event, c)
}

pub fn attr<V, MSG>(name: &'static str, v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    crate::builder::attr(name, v)
}
