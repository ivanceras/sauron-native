/// A container for generic event and the common values
/// needed for the user.
/// This events are derived from their corresponding backend source
/// ie: html events from mouse, keypresses and input changes.
/// This events should also be recreatable from gtk-rs, libui-rs,
/// orbtk, ncurses, etc.
///
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
    InputEvent(InputEvent),
    Generic(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputEvent {
    pub value: String,
}
