//! sauron native is a multi-faceted library which supports
//! multiple UI backends including native, html and tui
//!
#![deny(warnings, missing_docs)]
#![deny(clippy::all)]

pub mod event {
    //! reexport of sauron_vdom event as sauron_native event
    pub use sauron_vdom::{
        builder::on,
        event::{InputEvent, MouseEvent},
        Event,
    };
}
use widget::attribute::AttribKey;

pub mod backend;
mod component;
mod program;
mod util;
pub mod widget;

pub use backend::Backend;
pub use component::Component;
pub use program::Program;
pub use sauron_vdom::{builder, Callback, Event, Value};
pub use widget::Widget;

/// A node tree
pub type Node<MSG> = sauron_vdom::Node<Widget, AttribKey, Event, MSG>;
/// Element contains Widget Enum with attributes keyed by AttribKey enum
pub type Element<MSG> = sauron_vdom::Element<Widget, AttribKey, Event, MSG>;
/// Patch with key set to AttibKey enum and tag to Widget enum
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, Widget, AttribKey, Event, MSG>;
/// attribute type with key to AttribKey enum and tag set to Widget enum
pub type Attribute<MSG> = sauron_vdom::Attribute<AttribKey, Event, MSG>;
