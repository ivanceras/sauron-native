use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::Callback;

pub mod builder;
mod value;

pub use value::Value;

/// When building your views you'll typically use the `html!` macro to generate
/// `VirtualNode`'s.
///
/// `html! { <div> <span></span> </div> }` really generates a `VirtualNode` with
/// one child (span).
///
/// Later, on the client side, you'll use the `diff` and `patch` modules to
/// update the real DOM with your latest tree of virtual nodes (virtual dom).
///
/// Or on the server side you'll just call `.to_string()` on your root virtual node
/// in order to recursively render the node and all of its children.
///
/// TODO: Make all of these fields private and create accessor methods
/// TODO: Create a builder to create instances of VirtualNode::Element with
/// attrs and children without having to explicitly create a VElement
#[derive(Debug, PartialEq, Clone)]
pub enum VNode {
    Element(VElement),
    Text(VText),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct VElement {
    pub tag: String,
    pub attrs: BTreeMap<String, Value>,
    pub events: BTreeMap<String, Callback<Value>>,
    pub children: Vec<VNode>,
    pub namespace: Option<String>,
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct VText {
    pub text: String,
}

impl VElement {
    /// Create a VElement using the supplied tag name
    pub fn new(tag: &str) -> Self {
        VElement {
            tag: tag.to_string(),
            attrs: BTreeMap::new(),
            events: BTreeMap::new(),
            children: vec![],
            namespace: None,
        }
    }
}

impl fmt::Display for VElement {
    // Turn a VElement and all of it's children (recursively) into an HTML string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.tag).unwrap();

        for (attr, value) in self.attrs.iter() {
            write!(f, r#" {}="{}""#, attr, value)?;
        }

        write!(f, ">")?;

        for child in self.children.iter() {
            write!(f, "{}", child.to_string())?;
        }

        write!(f, "</{}>", self.tag)?;

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

impl From<VElement> for VNode {
    fn from(v: VElement) -> Self {
        VNode::Element(v)
    }
}
