//! provides struct and methods for constructing sauron event
//!
use crate::AttribKey;
use crate::Attribute;
use crate::Value;
use mt_dom::Callback;

/// A container for generic event and the common values
/// needed for the user.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// It is a mouse event
    MouseEvent(MouseEvent),
    /// Event is triggered by keypressed
    KeyEvent(KeyEvent),
    /// Events in text_area or text_input
    InputEvent(InputEvent),
    /// resize event
    Resize(f32, f32),
}

impl From<MouseEvent> for Event {
    fn from(me: MouseEvent) -> Self {
        Event::MouseEvent(me)
    }
}

impl From<InputEvent> for Event {
    fn from(ie: InputEvent) -> Self {
        Event::InputEvent(ie)
    }
}

impl From<KeyEvent> for Event {
    fn from(ke: KeyEvent) -> Self {
        Event::KeyEvent(ke)
    }
}

/// A mouse event contains the (x,y) coordinates, buttons and modifier keys
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct MouseEvent {
    /// the mouse event type
    pub r#type: &'static str,
    /// the location of the mouse event
    pub coordinate: Coordinate,
    /// which modifier keys are pressed
    pub modifier: Modifier,
    /// which mousebutton is pressed
    pub buttons: MouseButton,
}
impl MouseEvent {
    /// creates a mouse click event at x and y location
    pub fn click(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "click",
            coordinate: Coordinate::new(x, y),
            //TODO: specify the buttons
            ..Default::default()
        }
    }

    /// creates a mouse pressed event at x and y location
    pub fn pressed(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mousedown",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
        }
    }

    /// crates a mouse release event at x and y location
    pub fn release(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mouseup",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
        }
    }

    /// creates a mouse move event at x and y location
    pub fn mousemove(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mousemove",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
        }
    }

    /// returns the x component of this mouse event
    pub fn x(&self) -> i32 {
        self.coordinate.x()
    }

    /// returns the y component of this mouse event
    pub fn y(&self) -> i32 {
        self.coordinate.y()
    }
}

/// Keypresses creates a key event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// the key pressed
    pub key_code: KeyCode,
    /// the modifuer key pressed alongside
    pub modifier: Modifier,
}

impl KeyEvent {
    pub fn enter() -> Self {
        KeyEvent {
            key_code: KeyCode::Enter,
            modifier: Modifier::none(),
        }
    }
}

/// The keycode of the character pressed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    /// backspace
    Backspace,
    /// Enter 13
    Enter,
    /// arrow left
    Left,
    /// arrow right
    Right,
    /// arrow up
    Up,
    /// arrow down
    Down,
    /// Home button
    Home,
    /// End button
    End,
    /// Page up
    PageUp,
    /// Page down
    PageDown,
    /// Tab button
    Tab,
    /// Shitf + Tab?
    BackTab,
    /// Delete button
    Delete,
    /// Insert button
    Insert,
    /// Function buttons from F1..F12
    F(u8),
    /// The alphanumeric characters
    Char(char),
    /// Keycode is not identified
    Null,
    /// Escape key
    Esc,
}

/// Input event is triggered by controls such as text_area and text_input
#[derive(Debug, Clone, PartialEq)]
pub struct InputEvent {
    /// the input value
    pub value: Value,
}

impl InputEvent {
    /// creates an input event
    pub fn new<V: Into<Value>>(value: V) -> Self {
        InputEvent {
            value: value.into(),
        }
    }
}

impl From<Value> for InputEvent {
    fn from(value: Value) -> Self {
        InputEvent { value }
    }
}

/// Which mouse button is used
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    /// left mouse button
    Left,
    /// right mouse button
    Right,
    /// middle button
    Middle,
    /// mousewheel up
    WheelUp,
    /// mousewheel down
    WheelDown,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

/// The coordinate of the event
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate {
    /// x component of the client
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX
    pub client_x: i32,
    /// y component of the client
    ///https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY
    pub client_y: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX
    pub movement_x: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY
    pub movement_y: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX
    pub offset_x: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY
    pub offset_y: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX
    pub screen_x: i32,
    /// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY
    pub screen_y: i32,
    /// x component
    pub x: i32,
    /// y component
    pub y: i32,
}

impl Coordinate {
    /// creates a new coordinate with x and y component
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate {
            x,
            y,
            ..Default::default()
        }
    }

    /// returns the x component of the coordinate
    pub fn x(&self) -> i32 {
        self.x
    }

    /// returns the y component of the coordinate
    pub fn y(&self) -> i32 {
        self.y
    }
}

/// Modifier contains the information of which modifier keys are pressed when the event is
/// triggered
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Modifier {
    /// whether the alt key is pressed
    pub alt_key: bool,
    /// whether the ctrl key is pressed
    pub ctrl_key: bool,
    /// whether the shift key is pressed
    pub shift_key: bool,
}

impl Modifier {
    /// set the the ctrl key is pressed
    pub fn ctrl() -> Self {
        Modifier {
            ctrl_key: true,
            ..Default::default()
        }
    }
    pub fn alt() -> Self {
        Modifier {
            alt_key: true,
            ..Default::default()
        }
    }

    pub fn shift() -> Self {
        Modifier {
            shift_key: true,
            ..Default::default()
        }
    }

    pub fn none() -> Self {
        Modifier {
            ..Default::default()
        }
    }
}

/// creates an attribute with name `event` with a value of a callback derive from the function
/// `func`
pub fn on<F, MSG>(event: AttribKey, func: F) -> Attribute<MSG>
where
    F: Fn(Event) -> MSG + 'static,
{
    mt_dom::on(event, Callback::from(func))
}

/// create an attribute which attach a callback to the on_click event
pub fn on_click<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::ClickEvent, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousedown event
pub fn on_mousedown<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseDown, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mouseup event
pub fn on_mouseup<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseUp, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousemove event
pub fn on_mousemove<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::MouseMove, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// TODO:
pub fn on_doubleclick<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::DoubleClickEvent, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// TODO:
pub fn on_blur<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on(AttribKey::BlurEvent, move |ev: Event| match ev {
        Event::MouseEvent(me) => func(me),
        _ => unreachable!(),
    })
}

/// create an attribute which attach a callback to the on_mousemove event
pub fn on_input<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(InputEvent) -> MSG + 'static,
{
    on(AttribKey::InputEvent, move |ev: Event| match ev {
        Event::InputEvent(input) => func(input),
        _ => unreachable!(),
    })
}

pub fn on_keypress<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(KeyEvent) -> MSG + 'static,
{
    on(AttribKey::KeyEvent, move |ev: Event| match ev {
        Event::KeyEvent(ke) => func(ke),
        _ => unreachable!(),
    })
}

pub fn on_enter<F, MSG>(func: F) -> Attribute<MSG>
where
    F: Fn(KeyEvent) -> MSG + 'static,
{
    on(AttribKey::Activate, move |ev: Event| match ev {
        Event::KeyEvent(ke) => func(ke),
        _ => unreachable!(),
    })
}
