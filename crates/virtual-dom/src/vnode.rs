use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::fmt;

use crate::Callback;

pub enum VNode<'a> {
    Element(VElement<'a>),
    Text(VText),
}

pub struct VElement<'a> {
    pub tag: String,
    pub attrs: HashMap<String, &'a dyn Any>,
    pub events: Events<&'a dyn Any>,
    pub children: Vec<VNode<'a>>,
}

pub struct VText {
    pub text: String,
}

pub struct Events<IN>(pub HashMap<String, Callback<IN>>);


impl <'a>fmt::Debug for VNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VNode::Element(e) => write!(f, "Node::{:?}", e),
            VNode::Text(t) => write!(f, "Node::{:?}", t),
        }
    }
}

impl <'a>fmt::Debug for VElement<'a> {
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


impl <'a>fmt::Display for VElement<'a> {
    // Turn a VElement and all of it's children (recursively) into an HTML string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::util::any_to_string;

        write!(f, "<{}", self.tag).unwrap();


        for (attr, value) in self.attrs.iter() {
            write!(f, r#" {}="{}""#, attr, any_to_string(*value))?;
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
impl <'a>fmt::Display for VNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VNode::Element(element) => write!(f, "{}", element),
            VNode::Text(text) => write!(f, "{}", text),
        }
    }
}
