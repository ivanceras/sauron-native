#![deny(warnings)]
#![deny(clippy::all)]
pub mod dom;
#[macro_use]
pub mod html;
pub mod svg;

mod util;

pub use dom::DomUpdater;
pub use util::*;
