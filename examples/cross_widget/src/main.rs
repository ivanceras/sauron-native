#[cfg(feature = "with-gtk")]
use sauron_native::backend::gtk_ui::GtkBackend;
#[cfg(feature = "with-tui")]
use sauron_native::backend::text_ui::TuiBackend;
use sauron_native::Program;
use std::rc::Rc;

pub mod app;
use app::{App, Msg};

#[cfg(feature = "with-tui")]
fn main() {
    println!("tui mode");
    let program: Rc<Program<App, Msg, TuiBackend<App, Msg>>> = Program::new(App::new(1));
    program.dispatch(Msg::Click);
}

#[cfg(feature = "with-gtk")]
fn main() {
    println!("gkt backend");
    let program: Rc<Program<App, Msg, GtkBackend<App, Msg>>> = Program::new(App::new(1));
    program.dispatch(Msg::Click);
}
