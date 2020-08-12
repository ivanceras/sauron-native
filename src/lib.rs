//! sauron native is a multi-faceted library which supports
//! multiple UI backends including native, html and tui
//!
//#![deny(warnings, missing_docs)]
#![deny(clippy::all)]

pub use backend::Backend;
pub use component::Component;
pub use mt_dom;
pub use stretch;
pub use widget::{
    attribute::{AttribKey, Value},
    Widget,
};

pub mod backend;
mod component;
pub(crate) mod image_util;
pub mod widget;

/// It is an abstracted event to allow conversion
/// to multiple platforms such as gtk, html, titik
pub type Event = widget::attribute::Event;

/// we are not using namespace
pub type Namespace = ();

/// A node tree
pub type Node<MSG> =
    mt_dom::Node<Namespace, Widget, AttribKey, Value, Event, MSG>;
/// Element contains Widget Enum with attributes keyed by AttribKey enum
pub type Element<MSG> =
    mt_dom::Element<Namespace, Widget, AttribKey, Value, Event, MSG>;
/// Patch with key set to AttibKey enum and tag to Widget enum
pub type Patch<'a, MSG> =
    mt_dom::Patch<'a, Namespace, Widget, AttribKey, Value, Event, MSG>;
/// attribute type with key to AttribKey enum and tag set to Widget enum
pub type Attribute<MSG> =
    mt_dom::Attribute<Namespace, AttribKey, Value, Event, MSG>;

/// Callback with Event defined
pub type Callback<MSG> = mt_dom::Callback<Event, MSG>;
