use crate::{Backend, Component};
use native_windows_gui as nwg;
use nwg::{
    BoxLayout, Button, CheckBox, ImageDecoder, ImageFrame, Label, RadioButton, TextInput, Window,
};
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

pub struct NwgBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    root_node: Rc<Window>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> NwgBackend<APP, MSG> {
    fn create_app(&self) {
        let mut name_edit: TextInput = TextInput::default();
        let mut hello_button: Button = Button::default();
        let mut hello_button2: Button = Button::default();
        let mut cb1 = CheckBox::default();
        let mut rb1 = RadioButton::default();
        let mut txt1 = TextInput::default();
        let mut lbl1 = Label::default();
        let mut img1 = ImageFrame::default();

        let mut vbox: BoxLayout = BoxLayout::default();
        let mut hbox: BoxLayout = BoxLayout::default();

        TextInput::builder()
            .size((280, 25))
            .text("Heisenberg")
            .parent(&*self.root_node)
            .build(&mut name_edit)
            .unwrap();

        Button::builder()
            .size((280, 60))
            .text("Say my name")
            .parent(&*self.root_node)
            .build(&mut hello_button)
            .unwrap();

        Button::builder()
            .size((280, 60))
            .text("Button2")
            .parent(&*self.root_node)
            .build(&mut hello_button2)
            .unwrap();

        CheckBox::builder()
            .size((280, 60))
            .text("Checkbox1")
            .parent(&*self.root_node)
            .build(&mut cb1)
            .unwrap();

        RadioButton::builder()
            .size((280, 60))
            .text("Radio1")
            .parent(&*self.root_node)
            .build(&mut rb1)
            .unwrap();

        TextInput::builder()
            .size((280, 60))
            .text("TextInput1")
            .parent(&*self.root_node)
            .build(&mut txt1)
            .unwrap();

        Label::builder()
            .size((280, 60))
            .text("Label1")
            .parent(&*self.root_node)
            .build(&mut lbl1)
            .unwrap();

        let bitmap = ImageDecoder::new()
            .unwrap()
            .from_filename("horse.jpg")
            .ok()
            .map(|img| img.frame(0).unwrap().as_bitmap().ok())
            .flatten();

        ImageFrame::builder()
            .size((400, 200))
            .bitmap(bitmap.as_ref())
            .parent(&*self.root_node)
            .build(&mut img1)
            .unwrap();

        BoxLayout::builder()
            .parent(&*self.root_node)
            .layout_type(nwg::BoxLayoutType::Vertical)
            .cell_count(Some(15))
            .child(0, &hello_button)
            .child(1, &name_edit)
            .child(2, &hello_button2)
            .child(3, &cb1)
            .child(4, &rb1)
            .child(5, &txt1)
            .child(6, &lbl1)
            .child(7, &img1)
            .build(&mut vbox);

        let mut btns = vec![];
        for i in 0..5 {
            let mut btn = Button::default();
            btns.push(btn);

            Button::builder()
                .size((280, 60))
                .text(&format!("Button {}", 3 + i))
                .parent(&*self.root_node)
                .build(&mut btns[i])
                .unwrap();

            vbox.add_child((8 + i) as u32, &btns[i]);
        }

        let events_window = self.root_node.clone();

        let handler =
            nwg::full_bind_event_handler(&self.root_node.handle, move |evt, _evt_data, handle| {
                use nwg::Event;

                match evt {
                    Event::OnWindowClose => {
                        if &handle == &events_window as &nwg::Window {
                            //nwg::simple_message("Goodbye", &format!("Goodbye {}", name_edit.text()));
                            nwg::stop_thread_dispatch();
                        }
                    }
                    Event::OnButtonClick => {
                        if &handle == &hello_button {
                            nwg::simple_message("Hello", &format!("Hello {}", name_edit.text()));
                        }
                    }
                    _ => {}
                }
            });
        nwg::dispatch_thread_events();
        nwg::unbind_event_handler(&handler);
    }
    fn new(app: APP) -> Rc<Self> {
        nwg::init().expect("Failed to init Native Windows GUI");
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

        let backend = NwgBackend {
            app: Rc::new(RefCell::new(app)),
            root_node: Rc::new(window),
            _phantom_msg: PhantomData,
        };
        backend.create_app();
        Rc::new(backend)
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
        rc_app
    }
}

enum NwgWidget {
    Box(BoxLayout),
    Button(Button),
    Text(Label),
    TextInput(TextInput),
    Checkbox(CheckBox),
    Radio(RadioButton),
    Image(ImageFrame),
}
