use sauron_vdom::{
    builder::element,
    event::{Coordinate, KeyEvent, Modifier, MouseButton, MouseEvent},
    Event,
};
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, Stdout},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
};
use termion::{
    event::{
        self as term_event, Event as TermEvent, Key as TermKey, MouseButton as TermMouseButton,
        MouseEvent as TermMouseEvent,
    },
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

pub struct ItuiEvent(TermEvent);

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.events() {
                    let evt = evt.unwrap();
                    let itui_event = ItuiEvent(evt);
                    if itui_event.is_break() {
                        println!("control break..");
                    }
                    let event: Event = itui_event.into();
                    tx.send(event);
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    //tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl ItuiEvent {
    fn is_break(&self) -> bool {
        match self.deref() {
            TermEvent::Key(ke) => match ke {
                term_event::Key::Ctrl(ch) => *ch == 'c',
                _ => false,
            },
            _ => false,
        }
    }
}

impl Deref for ItuiEvent {
    type Target = TermEvent;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Event> for ItuiEvent {
    fn into(self) -> Event {
        match self.deref() {
            TermEvent::Key(ke) => match ke {
                TermKey::Char(ch) => {
                    let key_event = KeyEvent::new(ch.to_string());
                    Event::from(key_event)
                }
                TermKey::Ctrl(ch) => {
                    let key_event = KeyEvent {
                        key: ch.to_string(),
                        modifier: Modifier::ctrl(),
                        ..Default::default()
                    };
                    Event::from(key_event)
                }
                _ => panic!("unable to decode char from key event :{:#?}", ke),
            },
            TermEvent::Mouse(me) => match me {
                term_event::MouseEvent::Press(btn, x, y) => {
                    let mb = match btn {
                        term_event::MouseButton::Left => MouseButton::Left,
                        term_event::MouseButton::Right => MouseButton::Right,
                        term_event::MouseButton::Middle => MouseButton::Middle,
                        term_event::MouseButton::WheelUp => MouseButton::WheelUp,
                        term_event::MouseButton::WheelDown => MouseButton::WheelDown,
                    };
                    let mouse_event = MouseEvent {
                        r#type: "click",
                        coordinate: Coordinate::new(*x as i32, *y as i32),
                        buttons: mb,
                        ..Default::default()
                    };
                    Event::from(mouse_event)
                }
                term_event::MouseEvent::Release(x, y) => {
                    let mouse_event = MouseEvent {
                        r#type: "mouseup",
                        coordinate: Coordinate::new(*x as i32, *y as i32),
                        ..Default::default()
                    };
                    Event::from(mouse_event)
                }
                term_event::MouseEvent::Hold(x, y) => {
                    let mouse_event = MouseEvent {
                        r#type: "hold", //drag?
                        coordinate: Coordinate::new(*x as i32, *y as i32),
                        ..Default::default()
                    };
                    Event::from(mouse_event)
                }
            },
            TermEvent::Unsupported(_) => panic!("unsupported event!"),
        }
    }
}
