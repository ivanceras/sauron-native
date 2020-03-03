use native_windows_gui as nwg;
use nwg::{BoxLayout, Button, TextInput, Window,ImageDecoder, ImageFrame, Bitmap};
use image::{bmp::BMPEncoder,ColorType,ImageEncoder, GenericImageView};
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

    let mut vbox: BoxLayout = BoxLayout::default();
    let mut hbox: BoxLayout = BoxLayout::default();

    Window::builder()
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .size((800, 400))
        .position((300, 300))
        .title("Basic example")
        .build(&mut window)
        .unwrap();


        /*
    // NOTE: Uncomment this to panic at runtime with:
    // 'There is already a raw event handler bound with the handler ID 0',
    // <..>native-windows-gui-b74f684ad4534f77\14859f5\native-windows-gui\src\win32\window.rs:278:17
    BoxLayout::builder()
        .parent(&window)
        .layout_type(nwg::BoxLayoutType::Horizontal)
        .cell_count(Some(2))
        .build(&mut hbox);
        */

    TextInput::builder()
        .size((280, 25))
        .position((10, 10))
        .text("Heisenberg")
        .parent(&window)
        .build(&mut name_edit)
        .unwrap();

    let mut img1 = ImageFrame::default();

    /*
    let bitmap = ImageDecoder::new()
            .unwrap()
            .from_filename("../cross_widget/horse.jpg")
            .ok()
            .map(|img| img.frame(0).unwrap().as_bitmap().ok())
            .flatten();
    */
    let mut bitmap = Bitmap::default();

    let img = image::load_from_memory(include_bytes!("../horse.jpg")).expect("should load");
    let (width, height) = img.dimensions();
    let mut bytes: Vec<u8> = vec![];
    BMPEncoder::new(&mut bytes).write_image(&img.to_rgb().into_raw(), width, height, ColorType::Rgb8);

    //let bytes = include_bytes!("../horse.bmp");

    Bitmap::builder()
        .source_bin(Some(&bytes))
        .build(&mut bitmap);
    


    Button::builder()
        .size((280, 60))
        .position((10, 40))
        .text("Say my name")
        .parent(&window)
        .build(&mut hello_button)
        .unwrap();

    ImageFrame::builder()
            .size((400, 200))
            .bitmap(Some(&bitmap))
            .parent(&window)
            .build(&mut img1)
            .unwrap();

    BoxLayout::builder()
        .parent(&window)
        .layout_type(nwg::BoxLayoutType::Vertical)
        .cell_count(Some(3))
        .child(0, &name_edit)
        .child(1, &hello_button)
        .child(2, &img1)
        .build(&mut vbox);

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
