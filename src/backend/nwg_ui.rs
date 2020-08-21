//! native windows gui backend
//!
use super::Dispatch;
use crate::{
    widget::attribute::find_value, AttribKey, Attribute, Backend, Component,
    Node, Widget,
};
use image::{bmp::BMPEncoder, ColorType, GenericImageView, ImageEncoder};
use native_windows_gui as nwg;
use nwg::{
    Bitmap, Button, CheckBox, FlexboxLayout, ImageFrame, Label, RadioButton,
    RichTextBox, TextBox, TextInput, Window,
};
use std::{cell::RefCell, fmt, fmt::Debug, marker::PhantomData, rc::Rc};
use stretch::style::FlexDirection;

/// native windows Gui backend
pub struct NwgBackend<APP, MSG>
where
    MSG: 'static,
{
    #[allow(unused)]
    app: Rc<RefCell<APP>>,
    #[allow(unused)]
    current_vdom: Rc<RefCell<Node<MSG>>>,
    root_node: Rc<RefCell<Option<NwgWidget>>>,
    window: Rc<Window>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> NwgBackend<APP, MSG> {
    fn new(app: APP) -> Self
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
                nwg::WindowFlags::WINDOW
                    | nwg::WindowFlags::VISIBLE
                    | nwg::WindowFlags::RESIZABLE,
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
        println!("3.1 new");
        let (root_widget, _children_widgets) =
            NwgWidget::from_node_tree(&backend.window, &backend, root_vdom);
        println!("3.5 new");
        *backend.root_node.borrow_mut() = Some(root_widget);
        println!("3.6 new");

        let events_window = backend.window.clone();

        println!("4 new");
        let handler = nwg::full_bind_event_handler(
            &backend.window.handle,
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

        backend
    }
}

impl<APP, MSG> Backend<APP, MSG> for NwgBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) {
        println!("init app..");
        NwgBackend::new(app);
    }
}

impl<APP, MSG> Dispatch<MSG> for NwgBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn dispatch(&self, _msg: MSG) {}
}

enum NwgWidget {
    Box(FlexboxLayout),
    Overlay(FlexboxLayout),
    GroupBox(FlexboxLayout),
    Button(Button),
    Label(Label),
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
            NwgWidget::Box(_w) => write!(f, "FlexboxLayout"),
            NwgWidget::Overlay(_w) => write!(f, "Overlay"),
            NwgWidget::GroupBox(_w) => write!(f, "GroupBox"),
            NwgWidget::Button(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Label(w) => write!(f, "{}", w.class_name()),
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
    #[allow(unused)]
    fn as_box(self) -> Option<FlexboxLayout> {
        match self {
            NwgWidget::Box(box_layout) => Some(box_layout),
            _ => None,
        }
    }

    fn from_node_tree<MSG, DSP>(
        window: &Window,
        program: &DSP,
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
                let children: Vec<(Self, Vec<Self>)> = element
                    .children
                    .into_iter()
                    .map(|child| Self::from_node_tree(window, program, child))
                    .collect();

                let mut all_children = vec![];
                let (direct, indirect): (Vec<Self>, Vec<Vec<Self>>) =
                    children.into_iter().unzip();
                let nwg_widget = Self::from_node(
                    window,
                    program,
                    element.tag,
                    &direct,
                    element.attrs,
                );
                all_children.extend(direct);
                all_children.extend(indirect.into_iter().flatten());
                (nwg_widget, all_children)
            }
            crate::Node::Text(_txt) => unreachable!(),
        }
    }

    fn from_node<MSG, DSP>(
        window: &Window,
        _program: &DSP,
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
                        NwgWidget::Box(_child) => {
                            /*
                            box_layout.add_child(child, Style{
                                size: Size {
                                    width: Dimension::Percent(1.0),
                                    height: Dimension::Percent(1.0),
                                }
                            });*/
                        }

                        NwgWidget::Overlay(_child) => {}
                        NwgWidget::GroupBox(_child) => {}
                        NwgWidget::Button(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Label(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Paragraph(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::TextInput(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::TextArea(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Checkbox(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Radio(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Image(child, _) => {
                            builder = builder.child(child)
                        }
                    }
                }

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::Box(box_layout)
            }
            Widget::Hbox => {
                println!("hbox..");
                let mut box_layout = FlexboxLayout::default();

                let mut builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Row);

                for child in children.iter() {
                    match child {
                        NwgWidget::Box(_child) => {
                            /*
                            box_layout.add_child(child, Style{
                                size: Size {
                                    width: Dimension::Percent(1.0),
                                    height: Dimension::Percent(1.0),
                                }
                            });*/
                        }

                        NwgWidget::Overlay(_child) => {}
                        NwgWidget::GroupBox(_child) => {}
                        NwgWidget::Button(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Label(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Paragraph(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::TextInput(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::TextArea(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Checkbox(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Radio(child) => {
                            builder = builder.child(child)
                        }
                        NwgWidget::Image(child, _) => {
                            builder = builder.child(child)
                        }
                    }
                }

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::Box(box_layout)
            }
            Widget::Hpane => {
                println!("hpane");
                let mut box_layout = FlexboxLayout::default();

                let builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Row);

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::Box(box_layout)
            }
            Widget::Vpane => {
                println!("hpane");
                let mut box_layout = FlexboxLayout::default();

                let builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Column);

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::Box(box_layout)
            }
            Widget::Button => {
                println!("button..");
                let label = find_value(AttribKey::Label, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut btn = Button::default();

                Button::builder()
                    .text(&label)
                    .parent(window)
                    .build(&mut btn)
                    .expect("must build button");

                NwgWidget::Button(btn)
            }

            Widget::Label => {
                println!("label..");
                let label_value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());

                let mut lbl = Label::default();

                Label::builder()
                    .text(&label_value)
                    .parent(window)
                    .build(&mut lbl)
                    .expect("must build button");

                NwgWidget::Label(lbl)
            }
            Widget::Paragraph => {
                let txt = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.to_string())
                    .unwrap_or(String::new());
                let mut rtb = RichTextBox::default();

                RichTextBox::builder()
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

                let _value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .unwrap_or(false);

                let mut checkbox = CheckBox::default();
                CheckBox::builder()
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

                let _value = find_value(AttribKey::Value, &attrs)
                    .map(|v| v.as_bool())
                    .unwrap_or(false);

                let mut radio = RadioButton::default();
                RadioButton::builder()
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

                BMPEncoder::new(&mut bytes)
                    .write_image(
                        &img.to_rgb().into_raw(),
                        width,
                        height,
                        ColorType::Rgb8,
                    )
                    .expect("must write image");

                let mut bitmap = Bitmap::default();
                Bitmap::builder()
                    .source_bin(Some(&bytes))
                    .build(&mut bitmap)
                    .expect("must not error");

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
                let rtree = resvg::usvg::Tree::from_data(
                    &bytes,
                    &resvg::usvg::Options::default(),
                )
                .expect("must be parse into tree");
                let svg_size = rtree.svg_node().size;
                let (width, height) =
                    (svg_size.width() as u32, svg_size.height() as u32);
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

                BMPEncoder::new(&mut bytes)
                    .write_image(&rgba_raw, width, height, ColorType::Rgb8)
                    .expect("must write image");

                let mut bitmap = Bitmap::default();
                Bitmap::builder()
                    .source_bin(Some(&bytes))
                    .build(&mut bitmap)
                    .expect("must not error");

                let mut image_frame = ImageFrame::default();
                ImageFrame::builder()
                    .size((width as i32, height as i32))
                    .bitmap(Some(&bitmap))
                    .parent(window)
                    .build(&mut image_frame)
                    .expect("must build image_frame");

                NwgWidget::Image(image_frame, bitmap)
            }

            // TODO:
            Widget::Overlay => {
                let mut box_layout = FlexboxLayout::default();

                let builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Row);

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::Overlay(box_layout)
            }
            // TODO:
            Widget::GroupBox => {
                let mut box_layout = FlexboxLayout::default();

                let builder = FlexboxLayout::builder()
                    .parent(window)
                    .flex_direction(FlexDirection::Row);

                builder.build(&mut box_layout).expect("must not error");

                NwgWidget::GroupBox(box_layout)
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
