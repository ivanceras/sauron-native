#[cfg(feature = "with-gtk")]
use sauron_native::backend::gtk_ui::GtkBackend;
#[cfg(feature = "with-nwg")]
use sauron_native::backend::nwg_ui::NwgBackend;
#[cfg(feature = "with-tui")]
use sauron_native::backend::text_ui::TuiBackend;
#[cfg(feature = "with-titik")]
use sauron_native::backend::titik_ui::TitikBackend;
use sauron_native::Program;
use std::rc::Rc;

pub mod app;
use app::{App, Msg};

fn main() {
    pretty_env_logger::init();
    #[cfg(feature = "with-tui")]
    let program: Rc<Program<App, Msg, TuiBackend<App, Msg>>> = Program::new(App::new(1));
    #[cfg(feature = "with-titik")]
    let program: Rc<Program<App, Msg, TitikBackend<App, Msg>>> = Program::new(App::new(1));

    #[cfg(feature = "with-gtk")]
    {
        let program: Rc<Program<App, Msg, GtkBackend<App, Msg>>> = Program::new(App::new(1));
    }

    #[cfg(feature = "with-nwg")]
    {
        let program: Rc<Program<App, Msg, NwgBackend<App, Msg>>> = Program::new(App::new(1));
    }
}
