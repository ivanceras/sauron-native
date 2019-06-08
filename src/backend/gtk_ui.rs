use crate::{Backend, Component, Widget};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, CssProvider, Entry, Orientation,
    StyleContext, TextBufferExt, WidgetExt, Window, WindowPosition, WindowType,
};
use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use gtk::{IsA, Paned};

pub struct GtkBackend<APP, MSG> {
    app: APP,
    _phantom_msg: PhantomData<MSG>,
}
impl<APP, MSG> GtkBackend<APP, MSG> {
    fn new(app: APP) -> Self {
        GtkBackend {
            app,
            _phantom_msg: PhantomData,
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        Rc::new(GtkBackend::new(app))
    }

    fn start_render(self: &Rc<Self>) {
        println!("start render is called!");
        create_app(&self.app);
    }
}

enum GtkWidget {
    Paned(Paned),
    Button(Button),
}
impl GtkWidget {
    fn add_children(&self, children: Vec<GtkWidget>) {
        match self {
            GtkWidget::Paned(paned) => {
                for child in children {
                    match child {
                        GtkWidget::Button(btn) => {
                            paned.add(&btn);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
impl From<Button> for GtkWidget {
    fn from(btn: Button) -> Self {
        GtkWidget::Button(btn)
    }
}

impl From<Paned> for GtkWidget {
    fn from(paned: Paned) -> Self {
        GtkWidget::Paned(paned)
    }
}

fn widget_to_gtk_widget(widget: Widget) -> GtkWidget {
    match widget {
        Widget::Vbox => Paned::new(Orientation::Vertical).into(),
        Widget::Hbox => Paned::new(Orientation::Horizontal).into(),
        Widget::Button(txt) => Button::new_with_label(&txt).into(),
        _ => Button::new_with_label("not yet").into(),
    }
}
fn convert_widget_node_tree_to_gtk_widget<MSG>(widget_node: crate::Node<MSG>) -> GtkWidget {
    match widget_node {
        crate::Node::Element(element) => {
            let mut gtk_widget = widget_to_gtk_widget(element.tag);
            let mut children = vec![];
            for child in element.children {
                let gtk_child = convert_widget_node_tree_to_gtk_widget(child);
                children.push(gtk_child);
            }
            gtk_widget.add_children(children);
            gtk_widget
        }
        crate::Node::Text(txt) => Button::new_with_label(&txt.text).into(),
    }
}

fn draw_gtk_widget(rc_win: &Rc<ApplicationWindow>, gtk_widget: &GtkWidget) {
    match gtk_widget {
        GtkWidget::Paned(paned) => {
            rc_win.add(paned);
        }
        GtkWidget::Button(btn) => {
            rc_win.add(btn);
        }
    }
}

fn create_app<APP, MSG>(app: &APP)
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    let uiapp = Application::new("ivanceras.github.io.gtk", ApplicationFlags::FLAGS_NONE)
        .expect("Failed to start app");

    let view: crate::Node<MSG> = app.view();
    let gtk_widget: GtkWidget = convert_widget_node_tree_to_gtk_widget(view);
    uiapp.connect_activate(move |uiapp| {
        let win = ApplicationWindow::new(uiapp);
        let rc_win = Rc::new(win);
        rc_win.set_default_size(800, 1000);
        rc_win.set_icon_name(Some("applications-graphics"));
        rc_win.set_title("Gtk backend");
        draw_gtk_widget(&rc_win, &gtk_widget);
        rc_win.show_all();
    });
    uiapp.run(&[]);
}
