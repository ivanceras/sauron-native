use crate::{Backend, Component, Widget};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, CssProvider, Entry, Orientation,
    StyleContext, TextBuffer, TextBufferExt, TextTagTable, TextView, WidgetExt, Window,
    WindowPosition, WindowType,
};
use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use crate::{Attribute, Node};
use gtk::{IsA, Label, Paned};
use sauron_vdom::{event::MouseEvent, AttribValue, Dispatch};
use std::cell::RefCell;

/// removing a child widget by using
/// https://docs.rs/gtk/0.7.0/gtk/trait.ContainerExt.html#tymethod.remove
///

pub struct GtkBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_vdom: Node<MSG>,
    //TODO: need a reference to the gkt window
    //for updating its child widgets
    _phantom_msg: PhantomData<MSG>,
}
impl<APP, MSG> GtkBackend<APP, MSG>
where
    MSG: 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) -> Self {
        let current_vdom = app.view();
        GtkBackend {
            app: Rc::new(RefCell::new(app)),
            current_vdom,
            _phantom_msg: PhantomData,
        }
    }

    fn dispatch(self: &Rc<Self>, msg: MSG)
    where
        MSG: Debug,
    {
        //self.app.dispatch(msg);
        println!("dispatching : {:?}", msg);
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        let diff = sauron_vdom::diff(&self.current_vdom, &new_view);
        println!("diff: {:#?}", diff);
        // TODO do the widget update here
    }

    fn create_app(self: &Rc<Self>)
    where
        APP: Component<MSG> + 'static,
        MSG: Clone + Debug + 'static,
    {
        let uiapp = Application::new("ivanceras.github.io.gtk", ApplicationFlags::FLAGS_NONE)
            .expect("Failed to start app");

        let view: crate::Node<MSG> = self.app.borrow().view();
        let gtk_widget: GtkWidget = self.convert_widget_node_tree_to_gtk_widget(view);
        let self_clone = Rc::clone(&self);
        uiapp.connect_activate(move |uiapp| {
            let win = ApplicationWindow::new(uiapp);
            let rc_win = Rc::new(win);
            rc_win.set_default_size(800, 1000);
            rc_win.set_icon_name(Some("applications-graphics"));
            rc_win.set_title("Gtk backend");
            self_clone.draw_gtk_widget(&rc_win, &gtk_widget);
            rc_win.show_all();
        });
        uiapp.run(&[]);
    }

    fn draw_gtk_widget(self: &Rc<Self>, rc_win: &Rc<ApplicationWindow>, gtk_widget: &GtkWidget) {
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

    fn convert_widget_node_tree_to_gtk_widget(
        self: &Rc<Self>,
        widget_node: crate::Node<MSG>,
    ) -> GtkWidget
    where
        MSG: Debug,
    {
        match widget_node {
            crate::Node::Element(element) => {
                let mut gtk_widget = self.widget_to_gtk_widget(element.tag, element.attrs);
                let mut children = vec![];
                for child in element.children {
                    let gtk_child = self.convert_widget_node_tree_to_gtk_widget(child);
                    children.push(gtk_child);
                }
                gtk_widget.add_children(children);
                gtk_widget
            }
            crate::Node::Text(txt) => Button::new_with_label(&txt.text).into(),
        }
    }

    fn widget_to_gtk_widget(
        self: &Rc<Self>,
        widget: Widget,
        attrs: Vec<Attribute<MSG>>,
    ) -> GtkWidget
    where
        MSG: Debug + 'static,
    {
        match widget {
            Widget::Vbox => gtk::Box::new(Orientation::Vertical, 0).into(),
            Widget::Hbox => gtk::Box::new(Orientation::Horizontal, 0).into(),
            Widget::Button(txt) => {
                let btn = Button::new_with_label(&txt);
                for attr in attrs {
                    match attr.value {
                        AttribValue::Value(_) => {}
                        AttribValue::Callback(cb) => {
                            match attr.name {
                                "click" => {
                                    let self_clone = Rc::clone(self);
                                    btn.connect_clicked(move |_| {
                                        let mouse_event = MouseEvent::default();
                                        let msg = cb.emit(mouse_event);
                                        println!("got msg: {:?}", msg);
                                        //TODO: dispatch the program here
                                        // program.dispatch(msg);
                                        self_clone.dispatch(msg);
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }
                btn.into()
            }
            Widget::Text(txt) => textview(&txt),
            Widget::Block(txt) => textview(&txt),
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
        self.create_app();
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
