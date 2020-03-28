use crate::{
    widget::{
        attribute::{find_callback, find_value},
        Widget,
    },
    AttribKey, Attribute, Backend, Component, Node,
};
use image::GenericImageView;
use image::ImageBuffer;
use image::RgbaImage;
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
    find_layout,
    find_widget_mut, set_focused_node,
    stretch::{
        geometry::Size,
        number::Number,
        style::{Dimension, Style},
    },
    widget_node_idx_at, Buffer, Button, Checkbox, FlexBox, Image, LayoutTree, Radio, TextInput,
    Widget as Control,
    SvgImage,
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

    fn draw_buffer<W: Write>(
        w: &mut W,
        buf: &mut Buffer,
        control: &titik::Widget<MSG>,
        layout_tree: &LayoutTree,
    ) -> crossterm::Result<()> {
        let cmds = control.draw(buf, &layout_tree);
        //write!(w, "{}", buf);
        buf.render(w);
        cmds.iter()
            .for_each(|cmd| cmd.execute(w).expect("must execute"));
        w.flush()?;
        Ok(())
    }

    fn run<W: Write>(&self, w: &mut W) -> crossterm::Result<()> {
        titik::command::init(w);

        let mut focused_widget_idx: Option<usize> = None;
        //NOTE: Can not be done every draw loop, since the titik's widget/control will be
        //reset, therefore we need to apply_patches to the titik root_node
        let vdom = self.app.borrow().view();
        let mut control = Self::from_node_tree(vdom);
        titik::command::reset_top(w);

        loop {
            let (width, height) = terminal::size()?;
            control.set_size(Some(width as f32), Some(height as f32));

            let layout_tree = titik::compute_layout(
                control.as_mut(),
                Size {
                    width: Number::Defined(width as f32),
                    height: Number::Defined(height as f32),
                },
            );
            let mut buf = Buffer::new(width as usize, height as usize);
            titik::command::move_top(w)?;

            Self::draw_buffer(w, &mut buf, &*control, &layout_tree)?;

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

                            let focused_layout =
                                find_layout(&layout_tree, idx)
                                    .expect("must have a layout tree");

                            if let Some(focused_widget) = find_widget_mut(control.as_mut(), idx) {
                                if let Some(btn) =
                                    focused_widget.as_any_mut().downcast_mut::<Button<MSG>>()
                                {
                                    let msgs = btn.process_event(ev, &focused_layout.layout);
                                    for msg in msgs.into_iter() {
                                        self.app.borrow_mut().update(msg);
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        titik::command::finalize(w);
        Ok(())
    }

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
                let image = image::load_from_memory(&bytes).expect("should load");
                let (width, height) = image.dimensions();
                let mut img = Image::new(bytes);
                //TODO: get the image size, divide by 10
                let (width, height) = image.dimensions();
                img.set_size(Some(width as f32 / 10.0), Some(height as f32 / 10.0 / 2.0));
                Box::new(img)
            }
            Widget::Svg(svg) => {
                Box::new(SvgImage::new(svg))
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
