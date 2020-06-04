use super::Dispatch;
use crate::{
    util,
    widget::attribute::{find_callback, find_value},
    AttribKey, Attribute, Backend, Component, Node, Patch, Widget,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::{prelude::*, ApplicationFlags};
use glib::Value;
use gtk::{
    prelude::*, Adjustment, Application, ApplicationWindow, Button, CheckButton, Container,
    CssProvider, Entry, EntryBuffer, Image, IsA, Label, Orientation, Paned, RadioButton,
    ScrolledWindow, StyleContext, TextBuffer, TextBufferExt, TextTagTable, TextView, TextViewExt,
    WidgetExt, Window, WindowPosition, WindowType,
};
use image::ImageFormat;
use log::*;
use sauron_vdom::{
    event::{InputEvent, MouseEvent},
    AttribValue,
};
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

mod apply_patches;

pub struct GtkBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_vdom: Rc<RefCell<Node<MSG>>>,
    root_node: Rc<RefCell<Option<GtkWidget>>>,
    application: Application,
    _phantom_msg: PhantomData<MSG>,
}

pub(crate) enum GtkWidget {
    GBox(gtk::Box),
    Paned(Paned),
    Button(Button),
    Paragraph(TextView),
    TextInput(Entry),
    Checkbox(CheckButton),
    Radio(RadioButton),
    Image(Image),
    TextView(TextView),
    ScrollView(ScrolledWindow),
}

impl<APP, MSG> Clone for GtkBackend<APP, MSG> {
    fn clone(&self) -> Self {
        GtkBackend {
            app: Rc::clone(&self.app),
            current_vdom: Rc::clone(&self.current_vdom),
            root_node: Rc::clone(&self.root_node),
            application: self.application.clone(),
            _phantom_msg: PhantomData,
        }
    }
}

impl<APP, MSG> GtkBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) -> Self {
        let current_vdom = app.view();
        let root_vdom = app.view();

        if gtk::init().is_err() {
            println!("failed to initialize GTK Application");
        }
        let root_widget: Option<GtkWidget> = None;
        let mut backend = GtkBackend {
            app: Rc::new(RefCell::new(app)),
            current_vdom: Rc::new(RefCell::new(current_vdom)),
            root_node: Rc::new(RefCell::new(root_widget)),
            application: Application::new("ivanceras.github.io.gtk", ApplicationFlags::FLAGS_NONE)
                .expect("Failed to start app"),
            _phantom_msg: PhantomData,
        };
        let root_widget = Self::from_node_tree(&backend, root_vdom);
        *backend.root_node.borrow_mut() = Some(root_widget);
        backend
    }

    fn root_container(&self) -> Rc<Container> {
        let root_widget = self.root_node.borrow();
        if let Some(root_widget) = &*root_widget {
            match &root_widget {
                GtkWidget::GBox(gbox) => {
                    let container: &Container = gbox.upcast_ref();
                    Rc::new(container.clone())
                }
                GtkWidget::Paned(paned) => {
                    let container: &Container = paned.upcast_ref();
                    Rc::new(container.clone())
                }
                _ => panic!("expecting it to be a container"),
            }
        } else {
            panic!("must have a root widget");
        }
    }

    fn create_app(&self)
    where
        APP: Component<MSG> + 'static,
        MSG: Clone + Debug + 'static,
    {
        let self_clone = self.clone();
        self.application.connect_activate(move |uiapp| {
            let win = ApplicationWindow::new(uiapp);
            let rc_win = Rc::new(win);
            rc_win.set_default_size(800, 1000);
            rc_win.set_icon_name(Some("applications-graphics"));
            rc_win.set_title("Gtk backend");
            self_clone.attach_root_widget(&rc_win);
            rc_win.show_all();
        });
        self.application.run(&[]);
    }

    fn attach_root_widget(&self, window: &Rc<ApplicationWindow>)
    where
        APP: Component<MSG> + 'static,
        MSG: Clone + Debug + 'static,
    {
        if let Some(root_widget) = self.root_node.borrow().as_ref() {
            if let Some(root_widget) = root_widget.as_widget() {
                window.add(root_widget);
            }
        }
    }

    fn from_node_tree<DSP>(program: &DSP, widget_node: crate::Node<MSG>) -> GtkWidget
    where
        MSG: Debug + 'static,
        DSP: Clone + Dispatch<MSG> + 'static,
    {
        match widget_node {
            crate::Node::Element(element) => {
                let mut gtk_widget = from_node(program, &element.tag, &element.attrs);
                let mut children = vec![];
                for child in element.children {
                    let gtk_child = Self::from_node_tree(program, child);
                    children.push(gtk_child);
                }
                gtk_widget.add_children(children);
                gtk_widget
            }
            crate::Node::Text(txt) => {
                let btn = Button::new_with_label(&txt.text);
                GtkWidget::Button(btn)
            }
        }
    }
}

pub(crate) fn from_node<MSG, DSP>(
    program: &DSP,
    widget: &Widget,
    attrs: &Vec<Attribute<MSG>>,
) -> GtkWidget
where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    match widget {
        // vbox can have many children
        Widget::Vbox => {
            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            GtkWidget::GBox(vbox)
        }
        // hbox can have many children
        Widget::Hbox => {
            let hbox = gtk::Box::new(Orientation::Horizontal, 0);
            GtkWidget::GBox(hbox)
        }
        // paned has only 2 children
        Widget::Hpane => {
            let hpane = Paned::new(Orientation::Horizontal);
            GtkWidget::Paned(hpane)
        }
        Widget::Vpane => {
            let vpane = Paned::new(Orientation::Vertical);
            GtkWidget::Paned(vpane)
        }
        Widget::Button => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let btn = Button::new_with_label(&label);
            if let Some(cb) = find_callback(AttribKey::ClickEvent, &attrs) {
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                btn.connect_clicked(move |_| {
                    let mouse_event = MouseEvent::default();
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                });
            }
            GtkWidget::Button(btn)
        }
        Widget::Paragraph => {
            let buffer = TextBuffer::new(None::<&TextTagTable>);
            let text_view = TextView::new_with_buffer(&buffer);

            let txt = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            buffer.set_text(&txt);

            GtkWidget::Paragraph(text_view)
        }
        Widget::TextInput => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let buffer = EntryBuffer::new(Some(&*value));
            let entry = Entry::new_with_buffer(&buffer);

            if let Some(cb) = find_callback(AttribKey::InputEvent, &attrs) {
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                entry.connect_property_text_notify(move |entry| {
                    let input_event = InputEvent::new(entry.get_buffer().get_text());
                    let msg = cb_clone.emit(input_event);
                    program_clone.dispatch(msg);
                });
            }
            GtkWidget::TextInput(entry)
        }
        Widget::Checkbox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(false);

            let cb = CheckButton::new_with_label(&label);
            cb.set_property("active", &value);
            GtkWidget::Checkbox(cb)
        }
        Widget::Radio => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(false);
            let rb = RadioButton::new_with_label(&label);
            rb.set_property("active", &value);
            GtkWidget::Radio(rb)
        }
        Widget::Image => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);
            let image = Image::new();
            let mime = util::image_mime_type(&bytes).expect("unsupported have mime type");
            let pixbuf_loader = PixbufLoader::new_with_mime_type(mime).expect("error loader");
            pixbuf_loader
                .write(&bytes)
                .expect("Unable to write svg data into pixbuf_loader");

            pixbuf_loader.close().expect("error creating pixbuf");

            let pixbuf = pixbuf_loader.get_pixbuf();

            image.set_from_pixbuf(Some(&pixbuf.expect("error in pixbuf_loader")));
            GtkWidget::Image(image)
        }
        Widget::Svg => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);
            let image = Image::new();
            let pixbuf_loader =
                PixbufLoader::new_with_mime_type("image/svg+xml").expect("error loader");
            pixbuf_loader
                .write(bytes)
                .expect("Unable to write svg data into pixbuf_loader");

            pixbuf_loader.close().expect("error creating pixbuf");

            let pixbuf = pixbuf_loader.get_pixbuf();

            image.set_from_pixbuf(Some(&pixbuf.expect("error in pixbuf_loader")));
            GtkWidget::Image(image)
        }
        Widget::TextArea => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let buffer = TextBuffer::new(None::<&TextTagTable>);
            buffer.set_text(&value);

            if let Some(cb) = find_callback(AttribKey::InputEvent, &attrs) {
                println!("textarea has a callback..");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                buffer.connect_changed(move |buffer| {
                    let buffer_text =
                        buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), true);
                    if let Some(buffer_text) = buffer_text {
                        let input_event = InputEvent::new(buffer_text.to_string());
                        let msg = cb_clone.emit(input_event);
                        program_clone.dispatch(msg);
                    }
                });
            }

            let text_view = TextView::new_with_buffer(&buffer);
            text_view.set_monospace(true);

            GtkWidget::TextView(text_view)
        }
        Widget::Scroll => {
            let scroll_view = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
            GtkWidget::ScrollView(scroll_view)
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Self {
        let mut rc_app = GtkBackend::new(app);
        rc_app.create_app();
        rc_app
    }
}

impl<APP, MSG> Dispatch<MSG> for GtkBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn dispatch(&self, msg: MSG)
    where
        MSG: Debug,
    {
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        {
            let current_vdom = self.current_vdom.borrow();
            let diff = sauron_vdom::diff_with_key(&current_vdom, &new_view, &AttribKey::Key);
            apply_patches::apply_patches(self, &self.root_container(), &diff);
        }
        *self.current_vdom.borrow_mut() = new_view;
    }
}

impl GtkWidget {
    fn as_container(&self) -> Option<&Container> {
        match self {
            GtkWidget::GBox(gbox) => {
                let container: &Container = gbox.upcast_ref();
                Some(container)
            }
            GtkWidget::Paned(paned) => {
                let container: &Container = paned.upcast_ref();
                Some(container)
            }
            GtkWidget::ScrollView(scroll_view) => {
                let container: &Container = scroll_view.upcast_ref();
                Some(container)
            }
            _ => None,
        }
    }

    fn as_widget(&self) -> Option<&gtk::Widget> {
        match self {
            GtkWidget::Button(btn) => {
                let widget: &gtk::Widget = btn.upcast_ref();
                Some(widget)
            }
            GtkWidget::GBox(cbox) => {
                let widget: &gtk::Widget = cbox.upcast_ref();
                Some(widget)
            }
            GtkWidget::Paned(paned) => {
                let widget: &gtk::Widget = paned.upcast_ref();
                Some(widget)
            }
            GtkWidget::Paragraph(text_view) => {
                let widget: &gtk::Widget = text_view.upcast_ref();
                Some(widget)
            }
            GtkWidget::TextInput(textbox) => {
                let widget: &gtk::Widget = textbox.upcast_ref();
                Some(widget)
            }
            GtkWidget::Checkbox(checkbox) => {
                let widget: &gtk::Widget = checkbox.upcast_ref();
                Some(widget)
            }
            GtkWidget::Radio(radio) => {
                let widget: &gtk::Widget = radio.upcast_ref();
                Some(widget)
            }
            GtkWidget::Image(image) => {
                let widget: &gtk::Widget = image.upcast_ref();
                Some(widget)
            }
            GtkWidget::TextView(text_view) => {
                let widget: &gtk::Widget = text_view.upcast_ref();
                Some(widget)
            }
            GtkWidget::ScrollView(scroll_view) => {
                let widget: &gtk::Widget = scroll_view.upcast_ref();
                Some(widget)
            }
        }
    }

    fn add_children(&self, children: Vec<GtkWidget>) {
        match self {
            GtkWidget::Paned(paned) => {
                if children.len() != 2 {
                    warn!("pane should have 2 children");
                }
                if children.len() > 2 {
                    warn!("pane children excess of 2 is ignored");
                }
                if let Some(child1) = children.get(0).map(|c| c.as_widget()).flatten() {
                    paned.pack1(child1, true, true);
                    child1.set_size_request(200, 200); //set the size accdg to the child
                }
                if let Some(child2) = children.get(1).map(|c| c.as_widget()).flatten() {
                    paned.pack2(child2, true, true);
                    child2.set_size_request(100, 200);
                }
            }
            GtkWidget::ScrollView(container) => {
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        container.add(child_widget);
                    } else {
                        println!("was not able to add child widget: {:?}", child.as_widget());
                    }
                }
            }
            GtkWidget::GBox(container) => {
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        container.add(child_widget);
                    } else {
                        println!("was not able to add child widget: {:?}", child.as_widget());
                    }
                }
            }
            _ => (),
        }
    }
}
