#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

pub mod dom;
#[macro_use]
pub mod html;
pub mod svg;

mod util;

pub use dom::DomUpdater;
pub use util::*;
pub use vdom::builder::Attribute;
pub use vdom::Event;

pub type Node = vdom::Node<&'static str>;
pub type Element = vdom::Element<&'static str>;
pub type Patch<'a> = vdom::Patch<'a, &'static str>;
pub use vdom::Text;

pub trait Component: Widget {
    fn subscribe(&mut self, callback: Box<Fn()>);
}

pub trait Widget: View {
    fn update(&mut self);
}

pub trait View {
    fn view(&self) -> Node;
}
