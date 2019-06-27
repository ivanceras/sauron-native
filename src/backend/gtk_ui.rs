use crate::{Backend, Component, Widget};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, CssProvider, Entry, Orientation,
    StyleContext, TextBuffer, TextBufferExt, TextTagTable, TextView, WidgetExt, Window,
    WindowPosition, WindowType,
};
use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use gtk::{IsA, Label, Paned};

/// removing a child widget by using
/// https://docs.rs/gtk/0.7.0/gtk/trait.ContainerExt.html#tymethod.remove
///

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
    GBox(gtk::Box),
    Button(Button),
    Text(TextView),
}
impl GtkWidget {
    fn add_children(&self, children: Vec<GtkWidget>) {
        match self {
            GtkWidget::GBox(gbox) => {
                for child in children {
                    match child {
                        GtkWidget::Button(btn) => {
                            gbox.add(&btn);
                        }
                        GtkWidget::GBox(cbox) => {
                            gbox.add(&cbox);
                        }
                        GtkWidget::Text(text_view) => {
                            gbox.add(&text_view);
                        }
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

impl From<gtk::Box> for GtkWidget {
    fn from(gbox: gtk::Box) -> Self {
        GtkWidget::GBox(gbox)
    }
}

fn textview(txt: &str) -> GtkWidget {
    let buffer = TextBuffer::new(None::<&TextTagTable>);
    let text_view = TextView::new_with_buffer(&buffer);
    buffer.set_text(txt);
    GtkWidget::Text(text_view)
}

fn widget_to_gtk_widget(widget: Widget) -> GtkWidget {
    match widget {
        Widget::Vbox => gtk::Box::new(Orientation::Vertical, 0).into(),
        Widget::Hbox => gtk::Box::new(Orientation::Horizontal, 0).into(),
        Widget::Button(txt) => Button::new_with_label(&txt).into(),
        Widget::Text(txt) => textview(&txt),
        Widget::Block(txt) => textview(&txt),
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
        GtkWidget::GBox(gbox) => {
            rc_win.add(gbox);
        }
        GtkWidget::Button(btn) => {
            rc_win.add(btn);
        }
        GtkWidget::Text(text_view) => {
            rc_win.add(text_view);
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
