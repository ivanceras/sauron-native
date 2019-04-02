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

#[derive(PartialEq)]
pub struct VElement{
    pub tag: String,
    pub attrs: HashMap<String, Value>,
    pub events: Events<Value>,
    pub children: Vec<VNode>,
}

#[derive(Debug,PartialEq)]
pub enum Value{
    String(String),
    Vec(Vec<Value>),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
}


#[derive(PartialEq)]
pub struct VText {
    pub text: String,
}

#[derive(PartialEq)]
pub struct Events<IN>(pub HashMap<String, Callback<IN>>);




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

impl Value{

    fn as_str(&self) -> Option<&str> {
        match self{
            Value::String(ref v) => Some(&v),
            _ => None
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self{
            Value::String(v) => None, 
            Value::Vec(v) => None,
            Value::U8(v) =>  Some(*v as f64),
            Value::U16(v) => Some(*v as f64),
            Value::U32(v) => Some(*v as f64),
            Value::U64(v) => Some(*v as f64),
            Value::U128(v) => Some(*v as f64),
            Value::Usize(v) => Some(*v as f64),
            Value::I8(v) => Some(*v as f64),
            Value::I16(v) => Some(*v as f64),
            Value::I32(v) => Some(*v as f64),
            Value::I64(v) => Some(*v as f64),
            Value::I128(v) => Some(*v as f64),
            Value::Isize(v) => Some(*v as f64),
            Value::F32(v) => Some(*v as f64),
            Value::F64(v) => Some(*v )
        }
    }
}

impl fmt::Display for Value{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Value::String(v) => write!(f, "{}",v),
            Value::Vec(v) => write!(f, "{:?}", v),
            Value::U8(v) => write!(f, "{}", v),
            Value::U16(v) => write!(f, "{}", v),
            Value::U32(v) => write!(f, "{}", v),
            Value::U64(v) => write!(f, "{}", v),
            Value::U128(v) => write!(f, "{}", v),
            Value::Usize(v) => write!(f, "{}", v),
            Value::I8(v) => write!(f, "{}", v),
            Value::I16(v) => write!(f, "{}", v),
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::I128(v) => write!(f, "{}", v),
            Value::Isize(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
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
