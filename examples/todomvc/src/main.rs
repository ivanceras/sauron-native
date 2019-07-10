#[cfg(feature = "with-gtk")]
use sauron_native::backend::gtk_ui::GtkBackend;
#[cfg(feature = "with-tui")]
use sauron_native::backend::text_ui::TuiBackend;
use sauron_native::Program;
use std::rc::Rc;

pub mod app;
use app::{Model, Msg};

fn main() {
    #[cfg(feature = "with-tui")]
    let program: Rc<Program<Model, Msg, TuiBackend<Model, Msg>>> = Program::new(Model::new());

    #[cfg(feature = "with-gtk")]
    let program: Rc<Program<Model, Msg, GtkBackend<Model, Msg>>> = Program::new(Model::new());
}
