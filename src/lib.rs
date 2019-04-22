//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]
#![feature(arbitrary_self_types)]

use sauron::Event;
use sauron_vdom::Callback;

pub mod event {
    pub use sauron_vdom::{builder::on, Event, InputEvent, KeyEvent, MouseButton, MouseEvent};
}

pub mod backend;
mod component;
mod program;
pub mod widget;

pub use component::Component;
pub use program::Program;
pub use widget::Widget;

pub type Node<MSG> = sauron_vdom::Node<Widget, Callback<Event, MSG>>;
pub type Element<MSG> = sauron_vdom::Element<Widget, Callback<Event, MSG>>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, Widget, Callback<Event, MSG>>;
pub type Attribute<'a, MSG> = sauron_vdom::builder::Attribute<'a, Callback<Event, MSG>>;
