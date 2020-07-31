//!  sauron native supports multiple back-end
//!
use crate::Component;

#[cfg(feature = "with-html")]
pub mod html;
#[cfg(feature = "with-html")]
pub use html::HtmlApp;

#[cfg(feature = "with-titik")]
pub mod titik_ui;
#[cfg(feature = "with-titik")]
pub use titik_ui::TitikBackend;

#[cfg(feature = "with-gtk")]
pub mod gtk_ui;
#[cfg(feature = "with-gtk")]
pub use gtk_ui::GtkBackend;

#[cfg(feature = "with-nwg")]
pub mod nwg_ui;
#[cfg(feature = "with-nwg")]
pub use nwg_ui::NwgBackend;

/// All backend implementation must implement this trait
pub trait Backend<APP, MSG>
where
    MSG: 'static,
    APP: Component<MSG> + 'static,
{
    /// initialize the backend
    fn init(app: APP);
}

/// This trait is used in the DomUpdater to call the dispatch
/// method when an event occured
///
/// The Program will implement Dispatch instead of sending it to the
/// DomUpdater, this will simplify the amount of generics being defined.
pub trait Dispatch<MSG> {
    /// dispatch the msg which will subsequently change the application state
    fn dispatch(&self, msg: MSG);
}
