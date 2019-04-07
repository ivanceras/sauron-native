#![deny(warnings)]
mod callback;
mod diff;
mod patch;
mod vnode;

pub use vnode::builder;

pub use callback::Callback;
pub use diff::diff;
pub use patch::Patch;
pub use vnode::{Element, Node, Text, Value};
