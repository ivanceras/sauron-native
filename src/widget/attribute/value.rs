use std::fmt;
use stretch::style::Style;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Str(&'static str),
    Bool(bool),
    Bytes(Vec<u8>),
    Style(Style),
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
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&'static str> for Value {
    fn from(s: &'static str) -> Self {
        Value::Str(s)
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
            Value::Bytes(v) => todo!(),
        }
    }
}
