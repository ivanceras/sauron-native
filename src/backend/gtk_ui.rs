//! gtk backend
use super::Dispatch;
use crate::widget::layout::compute_node_layout;
use crate::{AttribKey, Backend, Component, Node};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Application, ApplicationWindow, Button, CheckButton, Container,
    Entry, EventBox, Frame, Image, Overlay, Paned, RadioButton, ScrolledWindow,
    TextView, WidgetExt,
};
use log::*;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};
use stretch::geometry::Size;
use stretch::number::Number;

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
    application_window: Rc<RefCell<Option<ApplicationWindow>>>,
    window_size: Rc<RefCell<(i32, i32)>>,
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
            application_window: self.application_window.clone(),
            window_size: self.window_size.clone(),
            _phantom_msg: PhantomData,
        }
    }
}

impl<APP, MSG> GtkBackend<APP, MSG>
where
    MSG: Debug + Clone + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) {
        let (initial_width, initial_height) = (800, 1000);
        let current_vdom =
            Self::calculate_view_layout(&app, (initial_width, initial_height));
        let root_vdom = current_vdom.clone();

        if gtk::init().is_err() {
            println!("failed to initialize GTK Application");
        }
        let application = Application::new(
            Some("ivanceras.github.io.gtk"),
            ApplicationFlags::FLAGS_NONE,
        )
        .expect("Failed to start app");

        let root_widget: Option<GtkWidget> = None;
        let backend = GtkBackend {
            app: Rc::new(RefCell::new(app)),
            current_vdom: Rc::new(RefCell::new(current_vdom)),
            root_node: Rc::new(RefCell::new(root_widget)),
            application_window: Rc::new(RefCell::new(None)),
            application,
            window_size: Rc::new(RefCell::new((initial_width, initial_height))),
            _phantom_msg: PhantomData,
        };

        let root_widget = convert_widget::from_node_tree(&backend, &root_vdom);
        *backend.root_node.borrow_mut() = Some(root_widget);

        let backend_clone = backend.clone();

        backend.application.connect_activate(move |application| {
            let application_window = ApplicationWindow::new(application);
            application_window.set_default_size(initial_width, initial_height);
            application_window.set_icon_name(Some("applications-graphics"));
            application_window.set_title("Gtk backend");
            application_window.add(
                backend_clone
                    .root_node
                    .borrow()
                    .as_ref()
                    .expect("must have a root node")
                    .as_widget()
                    .expect("must be a widget"),
            );
            let backend_clone2 = backend_clone.clone();
            application_window.connect_size_allocate(move |_win, rect| {
                println!(
                    "moved in ({},{}) resized to ({},{})",
                    rect.x, rect.y, rect.width, rect.height
                );
                *backend_clone2.window_size.borrow_mut() =
                    (rect.width, rect.height);
                backend_clone2.redraw();
            });

            application_window.show_all();
            *backend_clone.application_window.borrow_mut() =
                Some(application_window);
        });

        backend.application.run(&[]);
    }

    fn calculate_view_layout(app: &APP, window_size: (i32, i32)) -> Node<MSG> {
        let mut new_view = app.view();

        let (w, h) = window_size;
        let (adjusted_w, adjusted_h) = (w as f32 - 100.0, h as f32 - 20.0);

        compute_node_layout(
            &mut new_view,
            Size {
                width: Number::Defined(adjusted_w),
                height: Number::Defined(adjusted_h),
            },
        );
        new_view
    }

    /// redraw the UI due to layout changes caused by resize on the main window
    fn redraw(&self)
    where
        MSG: Debug,
    {
        let mut new_view = Self::calculate_view_layout(
            &self.app.borrow(),
            *self.window_size.borrow(),
        );
        let (w, h) = *self.window_size.borrow();
        let (adjusted_w, adjusted_h) = (w as f32 - 0.0, h as f32 - 0.0);

        compute_node_layout(
            &mut new_view,
            Size {
                width: Number::Defined(adjusted_w),
                height: Number::Defined(adjusted_h),
            },
        );
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
}

impl<APP, MSG> Backend<APP, MSG> for GtkBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) {
        GtkBackend::new(app);
    }
}

impl<APP, MSG> Dispatch<MSG> for GtkBackend<APP, MSG>
where
    MSG: Debug + Clone + 'static,
    APP: Component<MSG> + 'static,
{
    fn dispatch(&self, msg: MSG)
    where
        MSG: Debug,
    {
        self.app.borrow_mut().update(msg);
        self.redraw();
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
                for (index, child) in children.iter().enumerate() {
                    if let Some(child_widget) = child.as_widget() {
                        container.add_overlay(child_widget);
                        let c_index = container.get_child_index(child_widget);
                        assert_eq!(c_index, index as i32);
                    } else {
                        println!(
                            "was not able to add child widget: {:?}",
                            child.as_widget()
                        );
                    }
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
