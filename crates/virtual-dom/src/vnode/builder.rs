use crate::Callback;
use crate::{Element, Node, Text, Value};
use std::convert::AsRef;

pub struct Attribute<'a> {
    name: &'a str,
    value: AttribValue,
}

pub enum AttribValue {
    Value(Value),
    Callback(Callback<Value>),
}

impl<V: Into<Value>> From<V> for AttribValue {
    fn from(v: V) -> Self {
        AttribValue::Value(v.into())
    }
}

impl From<Callback<Value>> for AttribValue {
    fn from(c: Callback<Value>) -> Self {
        AttribValue::Callback(c)
    }
}

impl Element {
    pub fn set_attribute<V>(mut self, name: &str, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.attrs.insert(name.into(), value.into());
        self
    }

    pub fn add_attributes<'a, A>(mut self, attrs: A) -> Self
    where
        A: AsRef<[Attribute<'a>]>,
    {
        for a in attrs.as_ref() {
            match a.value {
                AttribValue::Value(ref v) => {
                    self.attrs.insert(a.name.to_string(), v.clone());
                }
                AttribValue::Callback(ref v) => {
                    self.events.insert(a.name.to_string(), v.clone());
                }
            }
        }
        self
    }

    pub fn add_children<C>(mut self, children: C) -> Self
    where
        C: AsRef<[Node]>,
    {
        for c in children.as_ref() {
            self.children.push(c.clone());
        }
        self
    }

    pub fn add_event_listener(mut self, event: &str, cb: Callback<Value>) -> Self {
        self.events.insert(event.to_string(), cb);
        self
    }
}

#[inline]
pub fn element<'a, A, C>(tag: &str, attrs: A, children: C) -> Node
where
    C: AsRef<[Node]>,
    A: AsRef<[Attribute<'a>]>,
{
    Node::Element(
        Element::new(tag)
            .add_children(children)
            .add_attributes(attrs),
    )
}

#[inline]
pub fn text<V>(v: V) -> Node
where
    V: Into<String>,
{
    Node::Text(Text { text: v.into() })
}

#[inline]
pub fn attr<'a, V>(name: &'a str, v: V) -> Attribute<'a>
where
    V: Into<Value>,
{
    Attribute {
        name: name,
        value: v.into().into(),
    }
}

#[inline]
pub fn on<'a, C>(name: &'a str, c: C) -> Attribute<'a>
where
    C: Into<Callback<Value>>,
{
    Attribute {
        name: name,
        value: c.into().into(),
    }
}
