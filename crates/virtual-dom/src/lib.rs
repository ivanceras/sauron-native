#![deny(warnings)]
mod callback;
pub mod diff;
mod patch;
mod vnode;

pub use vnode::builder;

pub use callback::Callback;
pub use patch::Patch;
pub use vnode::{Element, Node, Text, Value};
pub use diff::diff;
