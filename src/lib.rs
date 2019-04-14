//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

pub mod backend;
pub mod layout;
pub mod widget;

pub type WidgetNode = vdom::Node<widget::Widget>;

/// This needs to be wrap to be able to implement a
/// Into<browser::Node> for this Node
#[derive(Debug)]
pub struct Node(pub WidgetNode);
