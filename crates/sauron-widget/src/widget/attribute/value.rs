use expanse::result::Layout;
use expanse::style::PositionType;
use expanse::style::Style;
use std::fmt;

/// The possible values of widget attributes
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    /// generic strings, used in labels, btn labels
    String(String),
    /// a static version of the string, for efficient storage
    Str(&'static str),
    /// boolean values such as checked,
    Bool(bool),
    /// Bytes value, used in controls such as image
    Bytes(Vec<u8>),
    /// The stretch style used in calculating the layout of a widget
    Style(Style),
    /// the calculated layout of a widget
    Layout(Layout),
    /// Position type of a widget
    PositionType(PositionType),
    /// float values
    F64(f64),
}

impl Value {
    /// return the boolean value if it is a Bool variant
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            _ => false,
        }
    }

    /// return the bytes value if it is a Bytes variant
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(v) => Some(v),
            _ => None,
        }
    }

    /// return the f64 value if it is an F64 variant
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(v) => Some(*v),
            _ => None,
        }
    }

    /// return the &str value if it is a String or Str variant
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(&v),
            Value::Str(v) => Some(v),
            _ => None,
        }
    }

    /// return the style if it is a Style variant
    pub fn as_style(&self) -> Option<&Style> {
        match self {
            Value::Style(style) => Some(&style),
            _ => None,
        }
    }

    /// return the layout if it is a layout variant
    pub fn as_layout(&self) -> Option<&Layout> {
        match self {
            Value::Layout(layout) => Some(&layout),
            _ => None,
        }
    }

    /// return the position type if it is a PositionType variant
    pub fn as_position_type(&self) -> Option<PositionType> {
        match self {
            Value::PositionType(position) => Some(*position),
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
        Value::F64(v as f64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value::F64(v as f64)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::F64(v as f64)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::F64(v as f64)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::F64(v as f64)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::F64(v as f64)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::F64(v as f64)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::F64(v as f64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::F64(v as f64)
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

impl From<PositionType> for Value {
    fn from(v: PositionType) -> Self {
        Value::PositionType(v)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::F64(v) => write!(f, "{}", v),
            _ => todo!(),
        }
    }
}
