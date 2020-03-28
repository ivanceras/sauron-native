//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(arbitrary_self_types)]
#![feature(bindings_after_at)]

pub mod event {
    pub use sauron_vdom::{builder::on, event::InputEvent, Event};
}
use std::fmt;

pub mod backend;
mod component;
mod program;
mod util;
pub mod widget;

pub use backend::Backend;
pub use component::Component;
pub use program::Program;
pub use sauron_vdom::{builder, Callback, Event, Value};
pub use widget::Widget;

/// TODO: replace the &'static str attribute key as an enum
/// enumerating all the properties of our widget abstraction
#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum AttribKey {
    /// String, used in text_input
    Value,
    /// String, used in button, label, checkbox, radio
    Label,
    /// bool, used in checkbox, radio
    Checked,
    /// Alignment Enum, used in hbox and vbox
    Alignment,
    ClickEvent,
    InputEvent,
    Key,
}

impl fmt::Display for AttribKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Node<MSG> = sauron_vdom::Node<Widget, AttribKey, Event, MSG>;
pub type Element<MSG> = sauron_vdom::Element<Widget, AttribKey, Event, MSG>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, Widget, AttribKey, Event, MSG>;
pub type Attribute<MSG> = sauron_vdom::Attribute<AttribKey, Event, MSG>;
