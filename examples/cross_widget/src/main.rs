use sauron_ui::{backend::text_ui::TuiBackend, Program};
use std::rc::Rc;

pub mod app;
use app::{App, Msg};

fn main() {
    println!("tui mode");
    let program: Rc<Program<App, Msg, TuiBackend>> = Program::new(App::new(1));
}
