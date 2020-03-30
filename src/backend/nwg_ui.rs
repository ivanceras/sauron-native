use crate::{
    widget::attribute::{find_callback, find_value},
    AttribKey, Attribute, Backend, Component, Node, Patch, Widget,
};
use image::{bmp::BMPEncoder, ColorType, GenericImageView, ImageEncoder};
use native_windows_gui as nwg;
use nwg::{
    stretch::{
        geometry::Size,
        style::{Dimension, FlexDirection},
    },
    Bitmap, Button, CheckBox, ControlHandle, FlexboxLayout, ImageDecoder, ImageFrame, Label,
    RadioButton, TextInput, Window, TextBox, RichTextBox,
};
use super::Dispatch;

use std::{cell::RefCell, fmt, fmt::Debug, marker::PhantomData, rc::Rc};

pub struct NwgBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_vdom: Rc<RefCell<Node<MSG>>>,
    root_node: Rc<RefCell<Option<NwgWidget>>>,
    window: Rc<Window>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> NwgBackend<APP, MSG> {
    fn new(app: APP) -> Rc<Self>
    where
        MSG: Debug + 'static,
        APP: Component<MSG> + 'static,
    {
        println!("1 new");
        nwg::init().expect("Failed to init Native Windows GUI");
        let current_vdom = app.view();
        let root_vdom = app.view();

        let mut window: Window = Window::default();
        Window::builder()
            .flags(
                nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::RESIZABLE,
            )
            .size((800, 800))
            .position((300, 300))
            .title("Windows Backend")
            .build(&mut window)
            .unwrap();

        println!("2 new");

        let root_widget: Option<NwgWidget> = None;

        let backend = NwgBackend {
            app: Rc::new(RefCell::new(app)),
            current_vdom: Rc::new(RefCell::new(current_vdom)),
            root_node: Rc::new(RefCell::new(root_widget)),
            window: Rc::new(window),
            _phantom_msg: PhantomData,
        };

        println!("3 new");
        let rc_backend = Rc::new(backend);
        println!("3.1 new");
        let (root_widget, children_widgets) =
            NwgWidget::from_node_tree(&*rc_backend.window, &rc_backend, root_vdom);
        println!("3.5 new");
        *rc_backend.root_node.borrow_mut() = Some(root_widget);
        println!("3.6 new");

        let events_window = rc_backend.window.clone();

        println!("4 new");
        let handler = nwg::full_bind_event_handler(
            &rc_backend.window.handle,
            move |evt, _evt_data, handle| {
                use nwg::Event;

                match evt {
                    Event::OnWindowClose => {
                        if &handle == &events_window as &nwg::Window {
                            nwg::stop_thread_dispatch();
                        }
                    }
                    Event::OnButtonClick => {}
                    _ => {}
                }
            },
        );
        nwg::dispatch_thread_events();
        nwg::unbind_event_handler(&handler);
        println!("last part new");

        rc_backend
    }
}

impl<APP, MSG> Backend<APP, MSG> for NwgBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        println!("init app..");
        let mut rc_app = NwgBackend::new(app);
        //rc_app.create_app();
        rc_app
    }
}

impl<APP, MSG> Dispatch<MSG> for NwgBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn dispatch(self: &Rc<Self>, msg: MSG) {}
}

enum NwgWidget {
    Box(FlexboxLayout),
    Button(Button),
    Paragraph(RichTextBox),
    TextInput(TextInput),
    TextArea(TextBox),
    Checkbox(CheckBox),
    Radio(RadioButton),
    Image(ImageFrame, Bitmap),
}

impl fmt::Debug for NwgWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NwgWidget::Box(w) => write!(f, "FlexboxLayout"),
            NwgWidget::Button(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Paragraph(w) => write!(f, "{}", w.class_name()),
            NwgWidget::TextInput(w) => write!(f, "{}", w.class_name()),
            NwgWidget::TextArea(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Checkbox(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Radio(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Image(w, _) => write!(f, "{}", w.class_name()),
        }
    }
}

impl NwgWidget {
    fn as_box(self) -> Option<FlexboxLayout> {
        match self {
            NwgWidget::Box(box_layout) => Some(box_layout),
            _ => None,
        }
    }

    fn from_node_tree<MSG, DSP>(
        window: &Window,
        program: &Rc<DSP>,
        widget_node: crate::Node<MSG>,
    ) -> (Self, Vec<Self>)
    where
        MSG: Debug + 'static,
        DSP: Dispatch<MSG> + 'static,
    {
        println!("from node tree..");
        match widget_node {
            crate::Node::Element(element) => {
                println!("element...");
                let mut children: Vec<(Self, Vec<Self>)> = element
                    .children
                    .into_iter()
                    .map(|child| Self::from_node_tree(window, program, child))
                    .collect();

                let mut all_children = vec![];
                let (direct, indirect): (Vec<Self>, Vec<Vec<Self>>) = children.into_iter().unzip();
                let nwg_widget =
                    Self::from_node(window, program, element.tag, &direct, element.attrs);
                all_children.extend(direct);
                all_children.extend(indirect.into_iter().flatten());
                (nwg_widget, all_children)
            }
            crate::Node::Text(txt) => unreachable!(),
        }
    }

    fn from_node<MSG, DSP>(
        window: &Window,
        program: &Rc<DSP>,
        widget: Widget,
        children: &Vec<Self>,
        attrs: Vec<Attribute<MSG>>,
    ) -> Self
    where
        MSG: Debug + 'static,
        DSP: Dispatch<MSG> + 'static,
    {
        println!("from node...");
        match widget {
            Widget::Vbox => {
                println!("vbox..");
                let mut box_layout = FlexboxLayout::default();

                let mut builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Column);

                for child in children.iter() {
                    match child {
                        NwgWidget::Box(child) => {
                            /*
                            box_layout.add_child(child, Style{
                                size: Size {
                                    width: Dimension::Percent(1.0),
                                    height: Dimension::Percent(1.0),
                                }
                            });*/
                        }
                        NwgWidget::Button(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Points(20.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::Paragraph(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::TextInput(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::TextArea(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::Checkbox(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::Radio(child) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(20.0),
                            });
                        }
                        NwgWidget::Image(child, _) => {
                            builder = builder.child(child).child_size(Size {
                                width: Dimension::Percent(1.0),
                                height: Dimension::Points(400.0),
                            });
                        }
                    }
                }

                builder.build(&mut box_layout);

                NwgWidget::Box(box_layout)
            }
            Widget::Hbox => {
                println!("hbox..");
                let mut box_layout = FlexboxLayout::default();

                let mut builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Row);

                builder.build(&mut box_layout);

                NwgWidget::Box(box_layout)
            }
            Widget::Button => {
                println!("button..");
                let label = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut btn = Button::default();

                Button::builder()
                    .size((280, 20))
                    .text(&label)
                    .parent(window)
                    .build(&mut btn)
                    .expect("must build button");

                NwgWidget::Button(btn)
            }
            Widget::Paragraph => {
                let txt = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
                let mut rtb = RichTextBox::default();

                RichTextBox::builder()
                    .size((280, 20))
                    .text(&txt)
                    .parent(window)
                    .build(&mut rtb)
                    .expect("must build rich textbox");

                NwgWidget::Paragraph(rtb)
            }
            Widget::TextInput => {
                println!("textinput..");
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut text_input = TextInput::default();

                TextInput::builder()
                    .size((280, 20))
                    .text(&value)
                    .parent(window)
                    .build(&mut text_input)
                    .expect("must build text input");

                NwgWidget::TextInput(text_input)
            }
            Widget::TextArea => {
                println!("textinput..");
                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut text_box = TextBox::default();

                TextBox::builder()
                    .size((280, 60))
                    .text(&value)
                    .parent(window)
                    .build(&mut text_box)
                    .expect("must build textbox");

                NwgWidget::TextArea(text_box)
            }

            Widget::Checkbox => {
                println!("checkbox..");
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .flatten()
                    .unwrap_or(false);

                let mut checkbox = CheckBox::default();
                CheckBox::builder()
                    .size((280, 60))
                    .text(&label)
                    .parent(window)
                    .build(&mut checkbox)
                    .expect("must build checkbox");

                NwgWidget::Checkbox(checkbox)
            }
            Widget::Radio => {
                println!("radio button..");
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .flatten()
                    .unwrap_or(false);

                let mut radio = RadioButton::default();
                RadioButton::builder()
                    .size((280, 60))
                    .text(&label)
                    .parent(window)
                    .build(&mut radio)
                    .expect("must build checkbox");

                NwgWidget::Radio(radio)
            }
            Widget::Image => {
                let empty = vec![];
                let blob = find_value(AttribKey::Data, &attrs)
                    .map(|v| v.as_bytes())
                    .flatten()
                    .unwrap_or(&empty);

                let img = image::load_from_memory(&blob).expect("should load");
                let (width, height) = img.dimensions();
                let mut bytes: Vec<u8> = vec![];

                BMPEncoder::new(&mut bytes).write_image(
                    &img.to_rgb().into_raw(),
                    width,
                    height,
                    ColorType::Rgb8,
                );

                let mut bitmap = Bitmap::default();
                Bitmap::builder()
                    .source_bin(Some(&bytes))
                    .build(&mut bitmap);

                let mut image_frame = ImageFrame::default();
                ImageFrame::builder()
                    .size((width as i32, height as i32))
                    .bitmap(Some(&bitmap))
                    .parent(window)
                    .build(&mut image_frame)
                    .expect("must build image_frame");

                NwgWidget::Image(image_frame, bitmap)
            }
            Widget::Svg => {
                let empty = vec![];
                let bytes = find_value(AttribKey::Data, &attrs)
                    .map(|v| v.as_bytes())
                    .flatten()
                    .unwrap_or(&empty);
                let rtree = resvg::usvg::Tree::from_data(&bytes, &resvg::usvg::Options::default())
                    .expect("must be parse into tree");
                let svg_size = rtree.svg_node().size;
                let (width, height) = (svg_size.width() as u32, svg_size.height() as u32);
                let backend = resvg::default_backend();
                let mut img = backend
                    .render_to_image(&rtree, &resvg::Options::default())
                    .expect("must render to image");
                let rgba_vec = img.make_rgba_vec();
                let rgba_raw: Vec<u8> = rgba_vec.chunks(4).flat_map(|pixel|
                    // make transparent pixel white
                    if pixel[3] == 0 {
                        vec![255,255,255]
                    }else{
                        vec![pixel[0], pixel[1], pixel[2]]
                    }
                 ).collect();

                let mut bytes: Vec<u8> = vec![];

                BMPEncoder::new(&mut bytes).write_image(&rgba_raw, width, height, ColorType::Rgb8);

                let mut bitmap = Bitmap::default();
                Bitmap::builder()
                    .source_bin(Some(&bytes))
                    .build(&mut bitmap);

                let mut image_frame = ImageFrame::default();
                ImageFrame::builder()
                    .size((width as i32, height as i32))
                    .bitmap(Some(&bitmap))
                    .parent(window)
                    .build(&mut image_frame)
                    .expect("must build image_frame");

                NwgWidget::Image(image_frame, bitmap)
            }
        }
    }
    /*
    fn add_children(&self, children: &Vec<Self>) {
        println!("adding children...");
        for child in children.iter() {
            println!("child: {:?}", child);
        }
        match self {
            NwgWidget::Box(container) => {
                for (i, child) in children.iter().enumerate() {
                    println!("child {}", i);
                    match child {
                        NwgWidget::Box(child) => (),
                        NwgWidget::Button(child) => container.add_child(i as u32, child),
                        NwgWidget::Text(child) => container.add_child(i as u32, child),
                        NwgWidget::TextInput(child) => container.add_child(i as u32, child),
                        NwgWidget::Checkbox(child) => container.add_child(i as u32, child),
                        NwgWidget::Radio(child) => container.add_child(i as u32, child),
                        NwgWidget::Image(child, _) => container.add_child(i as u32, child),
                        NwgWidget::Button(child) => container.child(i as u32, child),
                        NwgWidget::Text(child) => container.child(i as u32, child),
                        NwgWidget::TextInput(child) => container.child(i as u32, child),
                        NwgWidget::Checkbox(child) => container.child(i as u32, child),
                        NwgWidget::Radio(child) => container.child(i as u32, child),
                        NwgWidget::Image(child,_) => container.child(i as u32, child),
                    }
                    container.child_size(Size{width: Dimension::Percent(1.0), height: Dimension::Auto})
                }
            }
            _ => panic!("can not add children for {:?}", self),
        }
    }
    */
}
