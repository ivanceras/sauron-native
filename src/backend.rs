use crate::{Component, Node, Widget};
use std::{fmt::Debug, rc::Rc};

#[cfg(feature = "with-html")]
pub mod html;
#[cfg(feature = "with-html")]
pub use html::HtmlBackend;

#[cfg(feature = "with-titik")]
pub mod titik_ui;

#[cfg(feature = "with-gtk")]
pub mod gtk_ui;

#[cfg(feature = "with-nwg")]
pub mod nwg_ui;

pub trait Backend<APP, MSG>
where
    MSG: 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) -> Self;

    fn start_render(&self) {
        // html backend don't use render loop
        //
        // this is useful for tui backend
    }
}

/// This trait is used in the DomUpdater to call the dispatch
/// method when an event occured
///
/// The Program will implement Dispatch instead of sending it to the
/// DomUpdater, this will simplify the amount of generics being defined.
pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG);
}
