use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::fmt;

use crate::Callback;

#[derive(PartialEq)]
pub enum VNode{
    Element(VElement),
    Text(VText),
}

pub struct VElement{
    pub tag: String,
    pub attrs: HashMap<String, Value>,
    pub events: Events<Value>,
    pub children: Vec<VNode>,
}

#[derive(Debug)]
pub enum Value{
    String(String),
    Vec(Vec<Value>),
    U8(u8),
}


#[derive(PartialEq)]
pub struct VText {
    pub text: String,
}

pub struct Events<IN>(pub HashMap<String, Callback<IN>>);

impl PartialEq for VElement{

    fn eq(&self, rhs: &Self) -> bool {
        self.tag == rhs.tag
            || self.events == rhs.events
            || self.children == rhs.children
    }
}

impl <IN>PartialEq for Events<IN>{

    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}


impl fmt::Debug for VNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VNode::Element(e) => write!(f, "Node::{:?}", e),
            VNode::Text(t) => write!(f, "Node::{:?}", t),
        }
    }
}

impl fmt::Debug for VElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element(<{}>, attrs: {:?}, children: {:?})",
            self.tag, self.attrs, self.children,
        )
    }
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text({})", self.text)
    }
}



impl fmt::Display for Value{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Value::String(v) => write!(f, "{}",v),
            Value::Vec(v) => write!(f, "{:?}", v),
            Value::U8(v) => write!(f, "{}", v),
        }
    }
}

impl From<u8> for Value {

    fn from(v: u8) -> Self {
        Value::U8(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl fmt::Display for VElement {
    // Turn a VElement and all of it's children (recursively) into an HTML string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::util::any_to_string;

        write!(f, "<{}", self.tag).unwrap();


        for (attr, value) in self.attrs.iter() {
            write!(f, r#" {}="{}""#, attr, value)?;
        }

        write!(f, ">")?;

        for child in self.children.iter() {
            write!(f, "{}", child.to_string())?;
        }

        /*
        if !html_validation::is_self_closing(&self.tag) {
            write!(f, "</{}>", self.tag)?;
        }
        */

        Ok(())
    }
}

// Turn a VText into an HTML string
impl fmt::Display for VText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

// Turn a VNode into an HTML string (delegate impl to variants)
impl fmt::Display for VNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VNode::Element(element) => write!(f, "{}", element),
            VNode::Text(text) => write!(f, "{}", text),
        }
    }
}
