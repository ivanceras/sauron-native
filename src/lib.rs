//! sauron native is a multi-faceted library which supports
//! multiple UI backends including native, html and tui
//!
//#![deny(warnings, missing_docs)]
#![deny(clippy::all)]

pub use backend::Backend;
pub use component::Component;
pub use mt_dom;
pub use sauron_widget::{
    attribute,
    attribute::{AttribKey, Value},
    widget, Attribute, Element, Event, Node, Patch, Widget,
};
pub use stretch;

pub mod backend;
mod component;
pub(crate) mod image_util;
