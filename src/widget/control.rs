use sauron_vdom::{Callback, Event};

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub(crate) label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextInput {
    pub(crate) value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
    pub(crate) label: String,
    pub(crate) value: bool,
}
