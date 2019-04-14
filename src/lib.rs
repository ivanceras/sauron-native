#![deny(warnings)]
#![deny(clippy::all)]

pub mod backend;
pub mod layout;
pub mod widget;

pub use widget::Widget;


pub type Node<Widget> = vdom::Node<Widget>;
