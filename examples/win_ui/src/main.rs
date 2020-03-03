use native_windows_gui as nwg;
use nwg::{Button, TextInput, Window};
/**
    A very simple application that show your name in a message box.

    This demo shows how to use NWG without the NativeUi trait boilerplate.
    Note that this way of doing things is alot less extensible and cannot make use of native windows derive.

    See `basic` for the NativeUi version and `basic_d` for the derive version
*/
use std::rc::Rc;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let mut window: Window = Window::default();
    let mut name_edit: TextInput = TextInput::default();
    let mut hello_button: Button = Button::default();

    Window::builder()
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .size((300, 115))
        .position((300, 300))
        .title("Basic example")
        .build(&mut window)
        .unwrap();

    TextInput::builder()
        .size((280, 25))
        .position((10, 10))
        .text("Heisenberg")
        .parent(&window)
        .build(&mut name_edit)
        .unwrap();

    Button::builder()
        .size((280, 60))
        .position((10, 40))
        .text("Say my name")
        .parent(&window)
        .build(&mut hello_button)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
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
                    //nwg::simple_message("Hello", &format!("Hello {}", name_edit.text()));
                }
            }
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
