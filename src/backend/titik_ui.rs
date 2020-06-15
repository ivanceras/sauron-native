use crate::{
    widget::{
        attribute::{find_callback, find_value},
        Widget,
    },
    AttribKey, Attribute, Backend, Component, Node,
};
use image::{GenericImageView, ImageBuffer, RgbaImage};
use std::{
    cell::Cell,
    cell::RefCell,
    fmt::Debug,
    io::{self, Stdout, Write},
    marker::PhantomData,
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
};
use titik::Dispatch;
use titik::{
    crossterm,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent},
        terminal,
    },
    find_layout, find_widget_mut, set_focused_node,
    stretch::{
        geometry::Size,
        number::Number,
        style::{Dimension, Style},
    },
    widget_node_idx_at, Buffer, Button, Checkbox, FlexBox, Image, LayoutTree, Radio, SvgImage,
    TextArea, TextInput, Widget as Control,
};

mod apply_patches;

pub struct TitikBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_dom: Rc<RefCell<Node<MSG>>>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn from_node_tree(widget_node: crate::Node<MSG>) -> Box<dyn titik::Widget<MSG>>
    where
        MSG: Debug + 'static,
    {
        match widget_node {
            crate::Node::Element(element) => {
                let mut control = Self::from_node(element.tag, &element.attrs);
                for child in element.children {
                    let child_widget = Self::from_node_tree(child);
                    control.add_child(child_widget);
                }
                control
            }
            crate::Node::Text(txt) => unreachable!(),
        }
    }

    fn from_node(widget: Widget, attrs: &Vec<Attribute<MSG>>) -> Box<dyn titik::Widget<MSG>>
    where
        MSG: Debug + 'static,
    {
        match widget {
            Widget::Vbox => {
                let mut vbox = FlexBox::new();
                vbox.vertical();
                Box::new(vbox)
            }
            Widget::Hbox => {
                let mut hbox = FlexBox::new();
                hbox.horizontal();
                Box::new(hbox)
            }
            //TOD: make a draggable pane for titik
            Widget::Vpane => {
                let mut vbox = FlexBox::new();
                vbox.vertical();
                Box::new(vbox)
            }
            Widget::Hpane => {
                let mut hbox = FlexBox::new();
                hbox.horizontal();
                Box::new(hbox)
            }
            Widget::Button => {
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut btn: Button<MSG> = Button::new(&label);
                if let Some(cb) = find_callback(AttribKey::ClickEvent, &attrs) {
                    fn map_event_from_crossterm(
                        event: crossterm::event::Event,
                    ) -> sauron_vdom::Event {
                        sauron_vdom::event::Event::MouseEvent(
                            sauron_vdom::event::MouseEvent::click(1, 1),
                        )
                    }
                    let cb = cb.clone();
                    let cb2 = cb.reform(map_event_from_crossterm);
                    btn.on_click = vec![cb2];
                }
                Box::new(btn)
            }
            Widget::Paragraph => {
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());
                let textarea = TextArea::new(value);
                Box::new(textarea)
            }
            Widget::TextInput => {
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());
                let input = TextInput::new(value);
                Box::new(input)
            }
            Widget::Checkbox => {
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .flatten()
                    .unwrap_or(false);

                let mut cb = Checkbox::new(&label);
                cb.set_checked(value);
                Box::new(cb)
            }
            Widget::Radio => {
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .flatten()
                    .unwrap_or(false);

                let mut rb = Radio::new(label);
                rb.set_checked(value);
                Box::new(rb)
            }
            Widget::Image => {
                let empty = vec![];
                let bytes = find_value(AttribKey::Data, &attrs)
                    .map(|v| v.as_bytes())
                    .flatten()
                    .unwrap_or(&empty);
                let image = image::load_from_memory(&bytes).expect("should load");
                let (width, height) = image.dimensions();
                let mut img = Image::new(bytes.to_vec());
                //TODO: get the image size, divide by 10
                let (width, height) = image.dimensions();
                img.set_size(Some(width as f32 / 10.0), Some(height as f32 / 10.0 / 2.0));
                Box::new(img)
            }
            Widget::Svg => {
                let empty = vec![];
                let bytes = find_value(AttribKey::Data, &attrs)
                    .map(|v| v.as_bytes())
                    .flatten()
                    .unwrap_or(&empty);
                let svg = String::from_utf8(bytes.to_vec()).unwrap_or(String::new());
                Box::new(SvgImage::new(svg))
            }
            Widget::TextArea => {
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());
                let height = find_value(AttribKey::Height, &attrs)
                    .map(|v| v.as_f64().map(|v| v as f32))
                    .flatten();
                let width = find_value(AttribKey::Width, &attrs)
                    .map(|v| v.as_f64().map(|v| v as f32))
                    .flatten();
                let mut textarea = TextArea::new(value);
                textarea.set_size(width, height);
                if let Some(cb) = find_callback(AttribKey::InputEvent, &attrs) {
                    eprintln!("textarea has an input event");
                    textarea.on_input = vec![cb.clone()];
                }
                Box::new(textarea)
            }
            Widget::Label => {
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());
                let height = find_value(AttribKey::Height, &attrs)
                    .map(|v| v.as_f64().map(|v| v as f32))
                    .flatten();
                let width = find_value(AttribKey::Width, &attrs)
                    .map(|v| v.as_f64().map(|v| v as f32))
                    .flatten();
                let mut textarea = TextArea::new(value);
                textarea.set_size(width, height);
                Box::new(textarea)
            }
            Widget::Overlay => {
                let mut flex = FlexBox::new();
                Box::new(flex)
            }
            Widget::Scroll => {
                let mut flex = FlexBox::new();
                flex.horizontal();
                Box::new(flex)
            }
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn init(app: APP) -> Self {
        let mut stdout = io::stdout();
        let vdom = app.view();
        let current_dom = app.view();
        let mut root_node = Rc::new(RefCell::new(Self::from_node_tree(vdom)));

        let mut backend = TitikBackend {
            app: Rc::new(RefCell::new(app)),
            current_dom: Rc::new(RefCell::new(current_dom)),
            _phantom_msg: PhantomData,
        };
        {
            titik::renderer::render(&mut stdout, Some(&backend), root_node);
        }
        backend
    }
}

impl<APP, MSG> Dispatch<MSG> for TitikBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    /// root_node is added as argument in this dispatch function so that they are in the same
    /// borrow, otherwise an AlreadyBorrowedError will be invoke at runtime.
    fn dispatch(&self, msg: MSG, root_node: &mut Box<dyn Control<MSG>>) {
        eprintln!("dispatching...");
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        let current_view = self.app.borrow().view();

        {
            let previous_dom = self.current_dom.borrow();
            let diff = sauron_vdom::diff_with_key(&previous_dom, &new_view, &AttribKey::Key);
            eprintln!("diff: {:#?}", diff);
            apply_patches::apply_patches(&self, root_node, &diff);
        }

        let mut new_node = Self::from_node_tree(new_view);
        *self.current_dom.borrow_mut() = current_view;
    }
}
