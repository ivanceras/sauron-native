use crate::{event::on, Attribute, Callback, Event, Value};

pub fn value<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    attr("value", v)
}

pub fn oninput<C, MSG>(c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
    MSG: Clone,
{
    on("input", c)
}

pub fn onclick<C, MSG>(c: C) -> Attribute<MSG>
where
    C: Into<Callback<Event, MSG>>,
    MSG: Clone,
{
    on("click", c)
}

pub fn attr<V, MSG>(name: &'static str, v: V) -> Attribute<MSG>
where
    V: Into<Value>,
    MSG: Clone,
{
    crate::builder::attr(name, v)
}
