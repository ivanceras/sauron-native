//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]
#![feature(arbitrary_self_types)]

pub mod event {
    pub use sauron_vdom::{builder::on, event::InputEvent, Event};
}

pub mod backend;
mod component;
mod program;
pub mod util;
pub mod widget;

pub use backend::Backend;
pub use component::Component;
pub use program::Program;
pub use sauron_vdom::{builder, Callback, Event, Value};
pub use widget::Widget;

pub type Node<MSG> = sauron_vdom::Node<Widget, &'static str, Event, MSG>;
pub type Element<MSG> = sauron_vdom::Element<Widget, &'static str, Event, MSG>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, Widget, &'static str, Event, MSG>;
pub type Attribute<MSG> = sauron_vdom::Attribute<&'static str, Event, MSG>;
