use crate::Backend;
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::fmt::Debug;
use crate::Component;
use native_windows_gui as nwg;
use nwg::Window;
use nwg::TextInput;
use nwg::Button;
use nwg::BoxLayout;

pub struct NwgBackend<APP,MSG>
where MSG:'static,
{
    app: Rc<RefCell<APP>>,
    window: Rc<Window>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP,MSG> NwgBackend<APP,MSG>{
    fn create_app(&self){
        let mut name_edit: TextInput = TextInput::default();
        let mut hello_button: Button = Button::default();
        let mut vbox: BoxLayout = BoxLayout::default();
    
        TextInput::builder()
            .size((280, 25))
            .text("Heisenberg")
            .parent(&*self.window)
            .build(&mut name_edit)
            .unwrap();
    
        Button::builder()
            .size((280, 60))
            .text("Say my name")
            .parent(&*self.window)
            .build(&mut hello_button)
            .unwrap();

            
        BoxLayout::builder()
            .parent(&*self.window)
            .layout_type(nwg::BoxLayoutType::Horizontal)
            .cell_count(Some(2))
            .child(0, &hello_button)
            .child(1, &name_edit)
            .build(&mut vbox);
    
        let events_window = self.window.clone();
    
        let handler = nwg::full_bind_event_handler(&self.window.handle, move |evt, _evt_data, handle| {
            use nwg::Event;
    
            match evt {
                Event::OnWindowClose => 
                    if &handle == &events_window as &nwg::Window {
                        //nwg::simple_message("Goodbye", &format!("Goodbye {}", name_edit.text()));
                        nwg::stop_thread_dispatch();
                    },
                Event::OnButtonClick => 
                    if &handle == &hello_button {
                        nwg::simple_message("Hello", &format!("Hello {}", name_edit.text()));
                    },
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
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((300, 115))
            .position((300, 300))
            .title("Basic example")
            .build(&mut window)
            .unwrap();

        let backend = NwgBackend{
            app: Rc::new(RefCell::new(app)),
            window: Rc::new(window),
            _phantom_msg: PhantomData,
        };
        backend.create_app();
        Rc::new(backend)
    }
}

impl<APP,MSG> Backend<APP,MSG> for NwgBackend<APP,MSG>
where APP: Component<MSG> + 'static,
MSG: Clone + Debug + 'static,
{
    fn init(app:APP) -> Rc<Self> {
        println!("init app..");
        let mut rc_app = NwgBackend::new(app);
        rc_app
    }
}