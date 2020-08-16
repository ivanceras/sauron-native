//! functionalities specific to titik ui
//!
use crate::{
    widget::{
        attribute::{find_callback, find_value},
        Widget,
    },
    AttribKey, Attribute, Backend, Component, Node,
};
use image::GenericImageView;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self},
    rc::Rc,
};
use titik::Callback;
use titik::{
    renderer::Renderer, Button, Checkbox, FlexBox, GroupBox, Image, Link,
    Radio, TextArea, TextInput, TextLabel, Widget as Control,
};

mod apply_patches;
mod convert_event;

/// Titik Backend
pub struct TitikBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_dom: Rc<RefCell<Node<MSG>>>,
}

impl<APP, MSG> TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn from_node_tree(widget_node: crate::Node<MSG>) -> Box<dyn titik::Widget>
    where
        MSG: Debug + 'static,
    {
        match widget_node {
            crate::Node::Element(element) => {
                let mut control = from_node(&element.tag, &element.attrs);
                for child in element.children {
                    let child_widget = Self::from_node_tree(child);
                    control.add_child(child_widget);
                }
                control
            }
            crate::Node::Text(_txt) => unreachable!(),
        }
    }

    /// root_node is added as argument in this dispatch function so that they are in the same
    /// borrow, otherwise an AlreadyBorrowedError will be invoke at runtime.
    fn dispatch(&self, msg: MSG, root_node: &mut dyn titik::Widget) {
        eprintln!("dispatching... {:?}", msg);
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        let current_view = self.app.borrow().view();

        {
            let previous_dom = self.current_dom.borrow();
            let diff = mt_dom::diff_with_key(
                &previous_dom,
                &new_view,
                &AttribKey::Key,
            );
            eprintln!("diff: {:#?}", diff);
            apply_patches::apply_patches(&self, root_node, &diff);
        }

        *self.current_dom.borrow_mut() = current_view;
    }
}

pub(crate) fn from_node<MSG>(
    widget: &Widget,
    attrs: &[Attribute<MSG>],
) -> Box<dyn titik::Widget>
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
        Widget::GroupBox => {
            let mut groupbox = GroupBox::new();
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            groupbox.set_label(&label);
            Box::new(groupbox)
        }
        Widget::Button => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let mut btn: Button = Button::new(&label);
            if let Some(callbacks) =
                find_callback(AttribKey::ClickEvent, &attrs)
            {
                for cb in callbacks {
                    let cb = cb.clone();
                    btn.add_click_listener(Callback::from(
                        move |t_event: titik::Event| {
                            cb.emit(convert_event::from_titik(t_event));
                        },
                    ));
                }
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
                .unwrap_or(false);

            let mut checkbox = Checkbox::new(&label);
            if let Some(callbacks) =
                find_callback(AttribKey::InputEvent, &attrs)
            {
                for cb in callbacks {
                    eprintln!("checkbox has an input event");
                    let cb = cb.clone();
                    checkbox.add_input_listener(Callback::from(
                        move |t_event: titik::Event| {
                            cb.emit(convert_event::from_titik(t_event));
                        },
                    ));
                }
            }
            checkbox.set_checked(value);
            Box::new(checkbox)
        }
        Widget::Radio => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
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
            let mut img = Image::new(bytes.to_vec());
            let (width, height) = image.dimensions();
            img.set_size(
                Some(width as f32 / 10.0),
                Some(height as f32 / 10.0 / 2.0),
            );
            Box::new(img)
        }
        Widget::Svg => todo!(),
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
            if let Some(callbacks) =
                find_callback(AttribKey::InputEvent, &attrs)
            {
                for cb in callbacks {
                    eprintln!("textarea has an input event");
                    let cb = cb.clone();
                    textarea.add_input_listener(Callback::from(
                        move |t_event: titik::Event| {
                            cb.emit(convert_event::from_titik(t_event));
                        },
                    ));
                }
            }
            Box::new(textarea)
        }
        Widget::Label => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            let mut text_input = TextLabel::new(value);
            Box::new(text_input)
        }
        Widget::Overlay => {
            let flex = FlexBox::new();
            Box::new(flex)
        }
        Widget::HeaderBar => {
            //TODO: make a header bar equivalent in titik ui
            let mut flex = FlexBox::new();
            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            flex.set_size(width, height);
            Box::new(flex)
        }
        Widget::MenuBar => {
            let mut flex = FlexBox::new();
            flex.set_border(true);
            flex.horizontal();
            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            flex.set_size(width, height);
            Box::new(flex)
        }
        Widget::Menu => {
            let mut flex = FlexBox::new();
            flex.set_border(true);
            flex.vertical();
            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            flex.set_size(width, height);
            Box::new(flex)
        }
        Widget::MenuItem => {
            let mut flex = FlexBox::new();
            flex.set_border(true);
            flex.vertical();
            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64().map(|v| v as f32))
                .flatten();
            flex.set_size(width, height);
            Box::new(flex)
        }
        Widget::SearchInput => {
            // TODO: make a search input equivalent in titik
            let mut search_input = TextInput::new("");
            Box::new(search_input)
        }
        Widget::Link => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            println!("link label: {}", label);

            let uri = find_value(AttribKey::Uri, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            println!("link uri: {}", uri);

            let mut link = Link::new(uri, label);
            Box::new(link)
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn init(app: APP) {
        let mut stdout = io::stdout();
        let vdom = app.view();
        let current_dom = app.view();
        let mut root_node = Self::from_node_tree(vdom);

        let backend = TitikBackend {
            app: Rc::new(RefCell::new(app)),
            current_dom: Rc::new(RefCell::new(current_dom)),
        };
        let mut renderer = Renderer::new(&mut stdout, root_node.as_mut());
        renderer.run().expect("must run");
    }
}
