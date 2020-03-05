use crate::{Attribute, Backend, Component, Node, Patch, Widget};
use native_windows_gui as nwg;
use nwg::{
    BoxLayout, Button, CheckBox, ControlHandle, ImageDecoder, ImageFrame, Label, RadioButton,
    TextInput, Window,Bitmap,
};
use sauron_vdom::Dispatch;
use std::{cell::RefCell, fmt, fmt::Debug, marker::PhantomData, rc::Rc};
use image::{bmp::BMPEncoder,ColorType,ImageEncoder, GenericImageView};
use crate::AttribKey;

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
            .title("Basic example")
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
    Box(BoxLayout),
    Button(Button),
    Text(Label),
    TextInput(TextInput),
    Checkbox(CheckBox),
    Radio(RadioButton),
    Image(ImageFrame, Bitmap),
}

impl fmt::Debug for NwgWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NwgWidget::Box(w) => write!(f, "BoxLayout"),
            NwgWidget::Button(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Text(w) => write!(f, "{}", w.class_name()),
            NwgWidget::TextInput(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Checkbox(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Radio(w) => write!(f, "{}", w.class_name()),
            NwgWidget::Image(w,_) => write!(f, "{}", w.class_name()),
        }
    }
}

impl NwgWidget {
    fn as_control_handle(&self) -> ControlHandle {
        match self {
            NwgWidget::Box(w) => panic!("no control handle for box"),
            NwgWidget::Button(w) => w.into(),
            NwgWidget::Text(w) => w.into(),
            NwgWidget::TextInput(w) => w.into(),
            NwgWidget::Checkbox(w) => w.into(),
            NwgWidget::Radio(w) => w.into(),
            NwgWidget::Image(w,_) => w.into(),
        }
    }

    fn as_box(self) -> Option<BoxLayout> {
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
                let nwg_widget = Self::from_node(window, program, element.tag, element.attrs);
                let mut children: Vec<(Self, Vec<Self>)> = element
                    .children
                    .into_iter()
                    .map(|child| Self::from_node_tree(window, program, child))
                    .collect();

                let mut all_children = vec![];
                let (direct, indirect): (Vec<Self>, Vec<Vec<Self>>) = children.into_iter().unzip();
                if !direct.is_empty() {
                    nwg_widget.add_children(&direct);
                }
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
                let mut box_layout = BoxLayout::default();

                BoxLayout::builder()
                    .parent(window)
                    .layout_type(nwg::BoxLayoutType::Vertical)
                    .cell_count(Some(10))
                    .build(&mut box_layout);

                NwgWidget::Box(box_layout)
            }
            Widget::Hbox => {
                println!("hbox..");
                let mut box_layout = BoxLayout::default();

                BoxLayout::builder()
                    .parent(window)
                    .layout_type(nwg::BoxLayoutType::Horizontal)
                    .cell_count(Some(10))
                    .build(&mut box_layout);

                NwgWidget::Box(box_layout)
            }
            Widget::Button => {
                println!("button..");
                let txt: String = if let Some(attr) = attrs.iter().find(|attr| attr.name == AttribKey::Value)
                {
                    if let Some(value) = attr.get_value() {
                        value.to_string()
                    } else {
                        "btn1".to_string()
                    }
                } else {
                    "Btn1".to_string()
                };

                let mut btn = Button::default();

                Button::builder()
                    .size((280, 60))
                    .text(&txt)
                    .parent(window)
                    .build(&mut btn)
                    .expect("must build button");

                NwgWidget::Button(btn)
            }
            Widget::Text(txt) => {
                println!("text..");
                let mut label = Label::default();

                Label::builder()
                    .size((280, 60))
                    .text(&txt)
                    .parent(window)
                    .build(&mut label)
                    .expect("must build label");

                NwgWidget::Text(label)
            }
            Widget::TextInput(txt) => {
                println!("textinput..");
                let mut text_input = TextInput::default();

                TextInput::builder()
                    .size((280, 60))
                    .text(&txt)
                    .parent(window)
                    .build(&mut text_input)
                    .expect("must build label");

                NwgWidget::TextInput(text_input)
            }

            Widget::Checkbox(label, value) => {
                println!("checkbox..");
                let mut checkbox = CheckBox::default();
                CheckBox::builder()
                    .size((280, 60))
                    .text(&label)
                    .parent(window)
                    .build(&mut checkbox)
                    .expect("must build checkbox");

                NwgWidget::Checkbox(checkbox)
            }
            Widget::Radio(label, value) => {
                println!("radio button..");
                let mut radio = RadioButton::default();
                RadioButton::builder()
                    .size((280, 60))
                    .text(&label)
                    .parent(window)
                    .build(&mut radio)
                    .expect("must build checkbox");

                NwgWidget::Radio(radio)
            }
            Widget::Image(blob) => {
                let mut bitmap = Bitmap::default();
                let img = image::load_from_memory(&blob).expect("should load");
                let (width, height) = img.dimensions();
                let mut bytes: Vec<u8> = vec![];

                BMPEncoder::new(&mut bytes).write_image(&img.to_rgb().into_raw(), width, height, ColorType::Rgb8);

                Bitmap::builder()
                    .source_bin(Some(&bytes))
                    .build(&mut bitmap);

                let mut image_frame = ImageFrame::default();
                ImageFrame::builder()
                        .size((400, 200))
                        .bitmap(Some(&bitmap))
                        .parent(window)
                        .build(&mut image_frame)
                        .expect("must build image_frame");

                NwgWidget::Image(image_frame, bitmap)
            }
        }
    }

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
                        NwgWidget::Image(child,_) => container.add_child(i as u32, child),
                    }
                }
            }
            _ => panic!("can not add children for {:?}", self),
        }
    }
}
