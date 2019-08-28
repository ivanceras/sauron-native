use crate::{Backend, Component, Widget};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, Container, CssProvider, Entry, EntryBuffer,
    Orientation, StyleContext, TextBuffer, TextBufferExt, TextTagTable, TextView, WidgetExt,
    Window, WindowPosition, WindowType,
};
use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use crate::{Attribute, Node, Patch};
use gtk::{IsA, Label, Paned};
use sauron_vdom::{
    event::{InputEvent, MouseEvent},
    AttribValue, Dispatch,
};
use std::cell::RefCell;

mod apply_patches;

pub struct GtkBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_vdom: Rc<RefCell<Node<MSG>>>,
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
            current_vdom: Rc::new(RefCell::new(current_vdom)),
            _phantom_msg: PhantomData,
        }
    }

    fn dispatch(self: &Rc<Self>, root_node: &Rc<ApplicationWindow>, msg: MSG)
    where
        MSG: Debug,
    {
        println!("dispatching : {:?}", msg);
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        {
        let current_vdom = self.current_vdom.borrow();
        let diff = sauron_vdom::diff(&current_vdom, &new_view);
        println!("diff: {:#?}", diff);
        apply_patches::apply_patches(root_node, &diff);
        }
        *self.current_vdom.borrow_mut() = new_view;
    }

    fn create_app(mut self: &Rc<Self>)
    where
        APP: Component<MSG> + 'static,
        MSG: Clone + Debug + 'static,
    {
        let uiapp = Application::new("ivanceras.github.io.gtk", ApplicationFlags::FLAGS_NONE)
            .expect("Failed to start app");

        let self_clone = Rc::clone(&self);
        uiapp.connect_activate(move |uiapp| {
            let win = ApplicationWindow::new(uiapp);
            let rc_win = Rc::new(win);
            rc_win.set_default_size(800, 1000);
            rc_win.set_icon_name(Some("applications-graphics"));
            rc_win.set_title("Gtk backend");
            self_clone.draw_widgets(&rc_win);
            rc_win.show_all();
        });
        uiapp.run(&[]);
    }

    fn draw_widgets(self: &Rc<Self>, root_node: &Rc<ApplicationWindow>)
    where
        APP: Component<MSG> + 'static,
        MSG: Clone + Debug + 'static,
    {
        let view = self.app.borrow().view();
        let gtk_widget: GtkWidget = self.convert_widget_node_tree_to_gtk_widget(&root_node, view);
        match &gtk_widget {
            GtkWidget::GBox(gbox) => {
                root_node.add(gbox);
            }
            GtkWidget::Button(btn) => {
                root_node.add(btn);
            }
            GtkWidget::Text(text_view) => {
                root_node.add(text_view);
            }
            GtkWidget::TextBox(textbox) => {
                root_node.add(textbox);
            }
        }
    }

    fn convert_widget_node_tree_to_gtk_widget(
        self: &Rc<Self>,
        root_node: &Rc<ApplicationWindow>,
        widget_node: crate::Node<MSG>,
    ) -> GtkWidget
    where
        MSG: Debug + 'static,
    {
        match widget_node {
            crate::Node::Element(element) => {
                let mut gtk_widget =
                    self.widget_to_gtk_widget(root_node, element.tag, &element.attrs);
                let mut children = vec![];
                for child in element.children {
                    let gtk_child = self.convert_widget_node_tree_to_gtk_widget(root_node, child);
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
        root_node: &Rc<ApplicationWindow>,
        widget: Widget,
        attrs: &Vec<Attribute<MSG>>,
    ) -> GtkWidget
    where
        MSG: Debug + 'static,
    {
        match widget {
            Widget::Vbox => {
                let vbox = gtk::Box::new(Orientation::Vertical, 0);
                vbox.into()
            }
            Widget::Hbox => gtk::Box::new(Orientation::Horizontal, 0).into(),
            Widget::Button => {
                let txt: String = if let Some(attr) = attrs.iter().find(|attr| attr.name == "value")
                {
                    if let Some(value) = attr.get_value() {
                        value.to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                let btn = Button::new_with_label(&txt);
                for attr in attrs {
                    match &attr.value {
                        AttribValue::Callback(cb) => match attr.name {
                            "click" => {
                                let self_clone = Rc::clone(self);
                                let root_node = Rc::clone(&root_node);
                                let cb_clone = cb.clone();
                                btn.connect_clicked(move |_| {
                                    let mouse_event = MouseEvent::default();
                                    let msg = cb_clone.emit(mouse_event);
                                    println!("got msg: {:?}", msg);
                                    //TODO: set the current_vdom after dispatching the callback
                                    self_clone.dispatch(&root_node, msg);
                                });
                            }
                            _ => {}
                        },
                        _ => (),
                    }
                }
                btn.into()
            }
            Widget::Text(txt) => textview(&txt),
            Widget::TextBox(txt) => {
                let buffer = EntryBuffer::new(Some(&*txt));
                let entry = Entry::new_with_buffer(&buffer);

                for attr in attrs {
                    match &attr.value {
                        AttribValue::Callback(cb) => match attr.name {
                            "input" => {
                                let self_clone = Rc::clone(self);
                                let root_node = Rc::clone(&root_node);
                                let cb_clone = cb.clone();
                                entry.connect_property_text_notify(move |entry| {
                                    let input_event =
                                        InputEvent::new(entry.get_buffer().get_text());
                                    let msg = cb_clone.emit(input_event);
                                    println!("got msg: {:?}", msg);
                                    self_clone.dispatch(&root_node, msg);
                                });
                            }
                            _ => {}
                        },
                        _ => (),
                    }
                }
                GtkWidget::TextBox(entry)
            }
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
        let mut rc_app = Rc::new(GtkBackend::new(app));
        rc_app.create_app();
        rc_app
    }
}

enum GtkWidget {
    GBox(gtk::Box),
    Button(Button),
    Text(TextView),
    TextBox(Entry),
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
                        GtkWidget::TextBox(textbox) => {
                            gbox.add(&textbox);
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
