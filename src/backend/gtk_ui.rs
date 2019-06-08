use crate::{Backend, Component, Widget};
use std::marker::PhantomData;
use std::{fmt::Debug, rc::Rc};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, CssProvider, Entry, Orientation,
    StyleContext, TextBufferExt, WidgetExt, Window, WindowPosition, WindowType,
};

use gtk::{
   Paned,IsA,
};

pub struct GtkBackend<APP,MSG> {
    app: APP,
    _phantom_msg: PhantomData<MSG>,
}
impl <APP,MSG>GtkBackend<APP,MSG>{
    fn new(app: APP) -> Self {
        GtkBackend { app, _phantom_msg: PhantomData }
    }
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP,MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        Rc::new(GtkBackend::new(app))
    }

    fn start_render(self: &Rc<Self>) {
        println!("start render is called!");
        create_app();
    }
}

enum GtkWidget{
    Paned(Paned),
    Button(Button),
}
impl From<Button> for GtkWidget{
    fn from(btn: Button) -> Self {
        GtkWidget::Button(btn)
    }
}

impl From<Paned> for GtkWidget{
    fn from(paned: Paned) -> Self {
        GtkWidget::Paned(paned)
    }
}

fn widget_to_gtk_widget(widget: Widget) -> GtkWidget{
    match widget {
        Widget::Vbox => Paned::new(Orientation::Vertical).into(),
        Widget::Hbox => Paned::new(Orientation::Horizontal).into(),
        Widget::Button(txt) => Button::new_with_label(&txt).into(),
        _ => Button::new_with_label("not yet").into(),
    }
}

fn create_app(){

    let uiapp = Application::new("ivanceras.github.io.gtk", ApplicationFlags::FLAGS_NONE)
        .expect("Failed to start app");

    uiapp.connect_activate(move |uiapp| {
        let win = ApplicationWindow::new(uiapp);
        let rc_win = Rc::new(win);
        rc_win.set_default_size(800, 1000);
        rc_win.set_icon_name(Some("applications-graphics"));
        rc_win.set_title("Gtk backend");

        rc_win.show_all();

    });
    uiapp.run(&[]);
}
