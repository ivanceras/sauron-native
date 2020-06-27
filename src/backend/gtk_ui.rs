//! gtk backend
use super::Dispatch;
use crate::{
    util,
    widget::attribute::{find_callback, find_value, util::is_scrollable},
    AttribKey, Attribute, Backend, Component, Node, Widget,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Adjustment, Application, ApplicationWindow, Button, CheckButton, Container, Entry,
    EntryBuffer, EventBox, Frame, Image, Label, Orientation, Overlay, Paned, RadioButton,
    ScrolledWindow, TextBuffer, TextBufferExt, TextTagTable, TextView, TextViewExt, WidgetExt,
};
use log::*;
use sauron_vdom::event::{InputEvent, MouseEvent};
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

mod apply_patches;
mod images;

/// backend using gtk
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
    GroupBox(Frame),
    Paned(Paned),
    Button(Button),
    Label(EventBox),
    Paragraph(TextView),
    TextInput(Entry),
    Checkbox(CheckButton),
    Radio(RadioButton),
    Image(Image),
    ImageScrollable(ScrolledWindow),
    TextView(TextView),
    TextViewScrollable(ScrolledWindow),
    Overlay(Overlay),
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
        let backend = GtkBackend {
            app: Rc::new(RefCell::new(app)),
            current_vdom: Rc::new(RefCell::new(current_vdom)),
            root_node: Rc::new(RefCell::new(root_widget)),
            application: Application::new(
                Some("ivanceras.github.io.gtk"),
                ApplicationFlags::FLAGS_NONE,
            )
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
                let gtk_widget = from_node(program, &element.tag, &element.attrs);
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
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            vbox.set_size_request(width as i32, height as i32);
            GtkWidget::GBox(vbox)
        }
        // hbox can have many children
        Widget::Hbox => {
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            let hbox = gtk::Box::new(Orientation::Horizontal, 0);
            hbox.set_size_request(width as i32, height as i32);
            GtkWidget::GBox(hbox)
        }
        Widget::GroupBox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.as_str())
                .flatten();
            let frame = Frame::new(label);
            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            frame.add(&vbox);
            GtkWidget::GroupBox(frame)
        }
        // paned has only 2 children
        Widget::Hpane => {
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let position = find_value(AttribKey::Position, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(width / 2.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let hpane = Paned::new(Orientation::Horizontal);
            hpane.set_size_request(width as i32, height as i32);
            hpane.set_position(position as i32);
            GtkWidget::Paned(hpane)
        }
        Widget::Vpane => {
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            let vpane = Paned::new(Orientation::Vertical);
            vpane.set_size_request(width as i32, height as i32);
            GtkWidget::Paned(vpane)
        }
        Widget::Button => {
            println!("it's a button");
            let label = find_value(AttribKey::Label, &attrs).map(|v| v.to_string());

            let svg_image_data = find_value(AttribKey::SvgImage, &attrs)
                .map(|v| v.as_bytes())
                .flatten();
            let btn = Button::new();
            if let Some(label) = label {
                btn.set_label(&label);
            }
            if let Some(cb) = find_callback(AttribKey::ClickEvent, &attrs) {
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                btn.connect_clicked(move |_| {
                    println!("btn is clicked..");
                    let mouse_event = MouseEvent::default();
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                });
            }

            if let Some(svg_image_data) = svg_image_data {
                println!("got an svg image here..");
                let svg_image: Image = images::svg_image(&svg_image_data);
                btn.set_image(Some(&svg_image));
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
        Widget::Label => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let label = Label::new(Some(&*value));

            let event_box = EventBox::new();
            if let Some(cb) = find_callback(AttribKey::MouseDown, &attrs) {
                println!("label has some mouse down");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                event_box.connect_button_press_event(move |_view, event| {
                    println!("label is button pressed");
                    let (x, y) = event.get_position();
                    let mouse_event = MouseEvent::pressed(x as i32, y as i32);
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                    Inhibit(false)
                });
            }
            if let Some(cb) = find_callback(AttribKey::MouseUp, &attrs) {
                println!("label has some mouse up");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                event_box.connect_button_release_event(move |_view, event| {
                    println!("label is button released");
                    let (x, y) = event.get_position();
                    let mouse_event = MouseEvent::release(x as i32, y as i32);
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                    Inhibit(false)
                });
            }

            if let Some(cb) = find_callback(AttribKey::MouseMove, &attrs) {
                println!("label has some mouse up");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                event_box.connect_motion_notify_event(move |_view, event| {
                    println!("label is button released");
                    let (x, y) = event.get_position();
                    let mouse_event = MouseEvent::mousemove(x as i32, y as i32);
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                    Inhibit(false)
                });
            }
            event_box.add(&label);
            label.show();
            event_box.show();
            GtkWidget::Label(event_box)
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
            cb.set_property("active", &value)
                .expect("must be able to set property");
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
            rb.set_property("active", &value)
                .expect("must be able to set property");
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

            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            image.set_size_request(width as i32, height as i32);
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
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            if let Some(cb) = find_callback(AttribKey::MouseDown, &attrs) {
                println!("textview has some mouse down");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                image.connect_button_press_event(move |_view, event| {
                    println!("textview is button pressed");
                    let (x, y) = event.get_position();
                    let mouse_event = MouseEvent::pressed(x as i32, y as i32);
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                    Inhibit(false)
                });
            }

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            image.set_size_request(width as i32, height as i32);
            if is_scrollable(&attrs) {
                let scroll = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
                scroll.add(&image);
                GtkWidget::ImageScrollable(scroll)
            } else {
                GtkWidget::Image(image)
            }
        }
        Widget::TextArea => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let editable = find_value(AttribKey::Editable, &attrs)
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(true);

            let buffer = TextBuffer::new(None::<&TextTagTable>);
            buffer.set_text(&value);

            if let Some(cb) = find_callback(AttribKey::InputEvent, &attrs) {
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                buffer.connect_end_user_action(move |buffer| {
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
            text_view.set_editable(editable);

            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            if let Some(cb) = find_callback(AttribKey::MouseDown, &attrs) {
                println!("textview has some mouse down");
                let cb_clone = cb.clone();
                let program_clone = program.clone();
                text_view.connect_button_press_event(move |_view, event| {
                    println!("textview is button pressed");
                    let (x, y) = event.get_position();
                    let mouse_event = MouseEvent::pressed(x as i32, y as i32);
                    let msg = cb_clone.emit(mouse_event);
                    program_clone.dispatch(msg);
                    Inhibit(false)
                });
            }

            text_view.set_size_request(width as i32, height as i32);

            if is_scrollable(&attrs) {
                let scroll = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
                scroll.set_size_request(width as i32, height as i32);
                scroll.add(&text_view);
                GtkWidget::TextViewScrollable(scroll)
            } else {
                GtkWidget::TextView(text_view)
            }
        }
        Widget::Overlay => {
            let overlay = Overlay::new();

            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            overlay.set_size_request(width as i32, height as i32);
            overlay.show_all();
            GtkWidget::Overlay(overlay)
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Self {
        let rc_app = GtkBackend::new(app);
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
            apply_patches::apply_patches(self, &current_vdom, &self.root_container(), &diff);
        }
        *self.current_vdom.borrow_mut() = new_view;
    }
}

impl GtkWidget {
    fn as_widget(&self) -> Option<&gtk::Widget> {
        match self {
            GtkWidget::Button(btn) => {
                let widget: &gtk::Widget = btn.upcast_ref();
                Some(widget)
            }
            GtkWidget::Label(label) => {
                let widget: &gtk::Widget = label.upcast_ref();
                Some(widget)
            }
            GtkWidget::GBox(cbox) => {
                let widget: &gtk::Widget = cbox.upcast_ref();
                Some(widget)
            }
            GtkWidget::GroupBox(group_box) => {
                let widget: &gtk::Widget = group_box.upcast_ref();
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
            GtkWidget::ImageScrollable(scroll) => {
                let widget: &gtk::Widget = scroll.upcast_ref();
                Some(widget)
            }
            GtkWidget::TextView(text_view) => {
                let widget: &gtk::Widget = text_view.upcast_ref();
                Some(widget)
            }
            GtkWidget::TextViewScrollable(scroll) => {
                let widget: &gtk::Widget = scroll.upcast_ref();
                Some(widget)
            }
            GtkWidget::Overlay(overlay) => {
                let widget: &gtk::Widget = overlay.upcast_ref();
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
                }
                if let Some(child2) = children.get(1).map(|c| c.as_widget()).flatten() {
                    paned.pack2(child2, true, true);
                }
            }
            GtkWidget::Overlay(container) => {
                let mut index = 0;
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        container.add_overlay(child_widget);
                        let c_index = container.get_child_index(child_widget);
                        assert_eq!(c_index, index);
                    } else {
                        println!("was not able to add child widget: {:?}", child.as_widget());
                    }
                    index += 1;
                }
            }
            GtkWidget::GBox(container) => {
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        //container.pack_start(child_widget, false, false, 0);
                        container.add(child_widget);
                    } else {
                        println!("was not able to add child widget: {:?}", child.as_widget());
                    }
                }
            }
            GtkWidget::GroupBox(frame_container) => {
                let frame_children = frame_container.get_children();
                let gbox_widget = frame_children.get(0).expect("must have one child");
                let container = gbox_widget
                    .downcast_ref::<Container>()
                    .expect("must be a container");
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        //container.pack_start(child_widget, false, false, 0);
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
