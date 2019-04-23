use crate::{Component, Node, Widget};
use std::{fmt::Debug, rc::Rc};

#[cfg(feature = "with-html")]
pub mod html;
#[cfg(feature = "with-html")]
pub use html::HtmlBackend;
#[cfg(feature = "with-tui")]
pub mod text_ui;

pub trait Backend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) -> Rc<Self>;

    /// backend will render the view
    fn render(self: &Rc<Self>, msg: MSG);
}
