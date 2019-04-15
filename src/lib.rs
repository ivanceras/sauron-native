//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

pub mod event {
    pub use vdom::builder::on;
    pub use vdom::Event;
    pub use vdom::InputEvent;
    pub use vdom::KeyEvent;
    pub use vdom::MouseButton;
    pub use vdom::MouseEvent;
}

pub mod browser {
    pub use browser::DomUpdater;
    pub use browser::Node;
    pub use browser::*;
}

pub mod backend;
pub mod widget;

pub type WidgetNode = vdom::Node<widget::Widget>;

/// This needs to be wrap to be able to implement a
/// Into<browser::Node> for this Node
#[derive(Debug)]
pub struct Node(pub WidgetNode);
