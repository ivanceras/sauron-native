use sauron_native::{backend::text_ui::TuiBackend, Program};
use std::rc::Rc;

pub mod app;
use app::{App, Msg};

fn main() {
    println!("tui mode");
    let program: Rc<Program<App, Msg, TuiBackend<App, Msg>>> = Program::new(App::new(1));
    program.dispatch(Msg::Click);
}
