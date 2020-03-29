//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(arbitrary_self_types)]
#![feature(bindings_after_at)]

pub mod event {
    pub use sauron_vdom::{builder::on, event::InputEvent, Event};
}
use std::fmt;
use widget::attribute::AttribKey;

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


pub type Node<MSG> = sauron_vdom::Node<Widget, AttribKey, Event, MSG>;
pub type Element<MSG> = sauron_vdom::Element<Widget, AttribKey, Event, MSG>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, Widget, AttribKey, Event, MSG>;
pub type Attribute<MSG> = sauron_vdom::Attribute<AttribKey, Event, MSG>;
