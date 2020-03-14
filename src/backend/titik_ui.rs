use crate::{
    widget::{attribute::find_value, Widget},
    AttribKey, Attribute, Backend, Component, Node,
};
use sauron_vdom::Dispatch;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, Stdout, Write},
    marker::PhantomData,
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
};
use titik::{
    crossterm,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent},
        terminal,
    },
    stretch::{
        geometry::Size,
        number::Number,
        style::{Dimension, Style},
    },
    Buffer, Button, Checkbox, FlexBox, Image, Radio, TextInput, Widget as Control,
};

pub struct TitikBackend<APP, MSG> {
    app: Rc<RefCell<APP>>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn start_draw_loop(&self) {
        let mut stderr = io::stdout();
        self.run(&mut stderr);
    }

    fn run<W: Write>(&self, w: &mut W) -> crossterm::Result<()> {
        titik::command::init(w);
        let (t_width, t_height) = terminal::size()?;
        let (width, height) = (t_width, t_height);

        let vdom = self.app.borrow().view();
        let mut control = Self::from_node_tree(vdom);
        control.set_size(Some(width as f32), Some(height as f32));

        let layout_tree = titik::compute_layout(
            control.as_mut(),
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
        loop {
            titik::command::reset_top(w)?;
            let mut buf = Buffer::new(width as usize, height as usize);
            control.draw(&mut buf, &layout_tree);
            write!(w, "{}", buf);
            w.flush()?;

            if let Ok(ev) = crossterm::event::read() {
                match ev {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    }) => {
                        if c == 'q' {
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }
        titik::command::finalize(w);
        Ok(())
    }

    fn from_node_tree(widget_node: crate::Node<MSG>) -> Box<dyn titik::Widget>
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

    fn from_node(widget: Widget, attrs: &Vec<Attribute<MSG>>) -> Box<dyn titik::Widget>
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
            Widget::Button => {
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let btn = Button::new(&label);
                Box::new(btn)
            }
            Widget::Text(txt) => {
                let input = TextInput::new(txt);
                Box::new(input)
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
            Widget::Image(bytes) => {
                let mut img = Image::new(bytes);
                img.set_size(Some(100.0), Some(50.0));
                Box::new(img)
            }
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        let backend = TitikBackend {
            app: Rc::new(RefCell::new(app)),
            _phantom_msg: PhantomData,
        };
        let rc_backend = Rc::new(backend);
        rc_backend.start_draw_loop();
        rc_backend
    }
}

impl<APP, MSG> Dispatch<MSG> for TitikBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn dispatch(self: &Rc<Self>, msg: MSG) {}
}
