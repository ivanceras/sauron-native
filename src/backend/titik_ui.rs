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
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent},
        terminal,
    },
    find_widget_mut, set_focused_node,
    stretch::{
        geometry::Size,
        number::Number,
        style::{Dimension, Style},
    },
    widget_node_idx_at, Buffer, Button, Checkbox, FlexBox, Image, Radio, TextInput,
    Widget as Control,
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
        let mut focused_widget_idx: Option<usize> = None;

        loop {
            titik::command::reset_top(w)?;
            let layout_tree = titik::compute_layout(
                control.as_mut(),
                Size {
                    width: Number::Defined(width as f32),
                    height: Number::Defined(height as f32),
                },
            );
            let mut buf = Buffer::new(width as usize, height as usize);
            let cmds = control.draw(&mut buf, &layout_tree);
            write!(w, "{}", buf);
            cmds.iter()
                .for_each(|cmd| cmd.execute(w).expect("must execute"));
            w.flush()?;

            if let Ok(ev) = crossterm::event::read() {
                match ev {
                    Event::Key(key_event @ KeyEvent { code, modifiers }) => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            match code {
                                KeyCode::Char(c) => {
                                    // To quite, press any of the following:
                                    //  - CTRL-c
                                    //  - CTRL-q
                                    //  - CTRL-d
                                    //  - CTRL-z
                                    //

                                    match c {
                                        'c' | 'q' | 'd' | 'z' => {
                                            break;
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                        } else {
                            if let Some(idx) = focused_widget_idx.as_ref() {
                                if let Some(focused_widget) =
                                    find_widget_mut(control.as_mut(), *idx)
                                {
                                    if let Some(txt_input1) =
                                        focused_widget.as_any_mut().downcast_mut::<TextInput>()
                                    {
                                        txt_input1.process_key(key_event);
                                    }
                                }
                            }
                        }
                    }

                    Event::Mouse(MouseEvent::Down(btn, x, y, _modifier)) => {
                        if let Some(idx) = widget_node_idx_at(&layout_tree, x as f32, y as f32) {
                            focused_widget_idx = Some(idx);
                            set_focused_node(control.as_mut(), idx);
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
