//! gtk backend
use super::Dispatch;
use crate::widget::attribute::event::{InputEvent, MouseEvent};
use crate::{
    util,
    widget::attribute::{find_callback, find_value, util::is_scrollable},
    AttribKey, Attribute, Backend, Component, Node, Widget,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Adjustment, Application, ApplicationWindow, Button,
    CheckButton, Container, Entry, EntryBuffer, EventBox, Frame, Image, Label,
    Orientation, Overlay, Paned, RadioButton, ScrolledWindow, TextBuffer,
    TextBufferExt, TextTagTable, TextView, TextViewExt, WidgetExt,
};
use log::*;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

mod apply_patches;
mod convert_widget;
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
        let root_widget = convert_widget::from_node_tree(&backend, root_vdom);
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
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) {
        let rc_app = GtkBackend::new(app);
        rc_app.create_app();
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
            let diff = mt_dom::diff_with_key(
                &current_vdom,
                &new_view,
                &AttribKey::Key,
            );
            apply_patches::apply_patches(
                self,
                &current_vdom,
                &self.root_container(),
                &diff,
            );
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
                if let Some(child1) =
                    children.get(0).map(|c| c.as_widget()).flatten()
                {
                    paned.pack1(child1, true, true);
                }
                if let Some(child2) =
                    children.get(1).map(|c| c.as_widget()).flatten()
                {
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
                        println!(
                            "was not able to add child widget: {:?}",
                            child.as_widget()
                        );
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
                        println!(
                            "was not able to add child widget: {:?}",
                            child.as_widget()
                        );
                    }
                }
            }
            GtkWidget::GroupBox(frame_container) => {
                let frame_children = frame_container.get_children();
                let gbox_widget =
                    frame_children.get(0).expect("must have one child");
                let container = gbox_widget
                    .downcast_ref::<Container>()
                    .expect("must be a container");
                for child in children {
                    if let Some(child_widget) = child.as_widget() {
                        //container.pack_start(child_widget, false, false, 0);
                        container.add(child_widget);
                    } else {
                        println!(
                            "was not able to add child widget: {:?}",
                            child.as_widget()
                        );
                    }
                }
            }
            _ => (),
        }
    }
}
