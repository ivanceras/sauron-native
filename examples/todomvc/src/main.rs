//#![deny(warnings)]
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

fn main() {
    pretty_env_logger::init();
    #[cfg(feature = "with-titik")]
    TitikBackend::init(app::Model::new());

    #[cfg(feature = "with-gtk")]
    GtkBackend::init(app::Model::new());

    #[cfg(feature = "with-nwg")]
    NwgBackend::init(app::Model::new());
}
