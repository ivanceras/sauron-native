#[cfg(feature = "with-gtk")]
use sauron_native::backend::gtk_ui::GtkBackend;
#[cfg(feature = "with-nwg")]
use sauron_native::backend::nwg_ui::NwgBackend;
#[cfg(feature = "with-tui")]
use sauron_native::backend::text_ui::TuiBackend;
#[cfg(feature = "with-titik")]
use sauron_native::backend::titik_ui::TitikBackend;
use sauron_native::Backend;
use std::rc::Rc;

pub mod app;
use app::{App, Msg};

fn main() {
    pretty_env_logger::init();
    #[cfg(feature = "with-titik")]
    TitikBackend::init(App::new(1));

    #[cfg(feature = "with-gtk")]
    GtkBackend::init(App::new(1));

    #[cfg(feature = "with-nwg")]
    NwgBackend::init(App::new(1));
}
