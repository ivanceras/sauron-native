use crate::{Component, Node, Widget};
use std::{fmt::Debug, rc::Rc};

#[cfg(feature = "with-html")]
pub mod html;
#[cfg(feature = "with-html")]
pub use html::HtmlBackend;
#[cfg(feature = "with-tui")]
pub mod text_ui;

#[cfg(feature = "with-gtk")]
pub mod gtk_ui;

#[cfg(feature = "with-nwg")]
pub mod nwg_ui;

pub trait Backend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) -> Rc<Self>;

    fn start_render(self: &Rc<Self>) {
        // html backend don't use render loop
        //
        // this is useful for tui backend
    }
}
