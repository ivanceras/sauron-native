use std::fmt;
use stretch::result::Layout;
use stretch::style::Style;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Str(&'static str),
    Bool(bool),
    Bytes(Vec<u8>),
    Style(Style),
    Layout(Layout),
    F64(f64),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            _ => false,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(&v),
            Value::Str(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_style(&self) -> Option<&Style> {
        match self {
            Value::Style(style) => Some(&style),
            _ => None,
        }
    }

    pub fn as_layout(&self) -> Option<&Layout> {
        match self {
            Value::Layout(layout) => Some(&layout),
            _ => None,
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::F64(v.into())
    }
}

impl From<&'static str> for Value {
    fn from(s: &'static str) -> Self {
        Value::Str(s)
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}

impl From<Layout> for Value {
    fn from(v: Layout) -> Self {
        Value::Layout(v)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::F64(v) => write!(f, "{}", v),
            Value::Style(s) => todo!(),
            Value::Layout(v) => todo!(),
            Value::Bytes(v) => todo!(),
        }
    }
}
