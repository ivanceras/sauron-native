//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

pub mod event {
    pub use sauron_vdom::builder::on;
    pub use sauron_vdom::Event;
    pub use sauron_vdom::InputEvent;
    pub use sauron_vdom::KeyEvent;
    pub use sauron_vdom::MouseButton;
    pub use sauron_vdom::MouseEvent;
}

pub mod browser {
    pub use sauron::DomUpdater;
    pub use sauron::Node;
    pub use sauron::*;
}

pub mod backend;
pub mod widget;

pub type WidgetNode = sauron_vdom::Node<widget::Widget>;

/// This needs to be wrap to be able to implement a
/// Into<sauron::Node> for this Node
#[derive(Debug)]
pub struct Node(pub WidgetNode);
