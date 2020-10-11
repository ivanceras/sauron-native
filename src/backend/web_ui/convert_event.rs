use crate::widget::event::{
    Coordinate, InputEvent, KeyCode, KeyEvent, Modifier, MouseButton,
    MouseEvent,
};
use wasm_bindgen::JsCast;

/// convert html mouse event to sauron native Event
pub fn from_mouse_event(event: sauron::web_sys::MouseEvent) -> MouseEvent {
    let mouse: &web_sys::MouseEvent =
        event.dyn_ref().expect("Unable to cast to mouse event");

    let coordinate = Coordinate {
        client_x: mouse.client_x(),
        client_y: mouse.client_y(),
        movement_x: mouse.movement_x(),
        movement_y: mouse.movement_y(),
        offset_x: mouse.offset_x(),
        offset_y: mouse.offset_y(),
        screen_x: mouse.screen_x(),
        screen_y: mouse.screen_y(),
        x: mouse.x(),
        y: mouse.y(),
    };
    let modifier = Modifier {
        alt_key: mouse.alt_key(),
        ctrl_key: mouse.ctrl_key(),
        shift_key: mouse.shift_key(),
    };
    let buttons = match mouse.button() {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Left,
        3 => MouseButton::WheelUp,
        4 => MouseButton::WheelDown,
        _ => Default::default(), // defaults to left
    };
    let r#type = match &*event.type_() {
        "click" => "click",
        "mouseup" => "mouseup",
        "mousedown" => "mousedown",
        "mousemove" => "mousemove",
        "dblclick" => "dblclick",
        _e => {
            panic!("unhandled event type: {}", _e);
        }
    };
    MouseEvent {
        r#type,
        coordinate,
        modifier,
        buttons,
    }
}

pub fn to_input_event(input_event: sauron::InputEvent) -> InputEvent {
    // TODO: make a comprehensive conversion here
    InputEvent::new(input_event.value)
}

pub fn to_key_event(keyboard_event: &sauron::KeyboardEvent) -> KeyEvent {
    KeyEvent {
        key_code: get_keycode(keyboard_event),
        modifier: get_modifier(keyboard_event),
    }
}

fn get_keycode(keyboard_event: &sauron::KeyboardEvent) -> KeyCode {
    let keycode = keyboard_event.code();
    match keycode.as_ref() {
        "Enter" => KeyCode::Enter,
        "Digit1" => KeyCode::Char('1'),
        "Digit2" => KeyCode::Char('2'),
        "Digit3" => KeyCode::Char('3'),
        "Digit4" => KeyCode::Char('4'),
        "Digit5" => KeyCode::Char('5'),
        "Digit6" => KeyCode::Char('6'),
        "Digit7" => KeyCode::Char('7'),
        "Digit8" => KeyCode::Char('8'),
        "Digit9" => KeyCode::Char('9'),
        "Digit0" => KeyCode::Char('0'),
        "KeyA" => KeyCode::Char('a'),
        "KeyB" => KeyCode::Char('b'),
        "KeyC" => KeyCode::Char('c'),
        "KeyD" => KeyCode::Char('d'),
        "KeyE" => KeyCode::Char('e'),
        "KeyF" => KeyCode::Char('f'),
        "KeyG" => KeyCode::Char('g'),
        "KeyH" => KeyCode::Char('h'),
        "KeyI" => KeyCode::Char('i'),
        "KeyJ" => KeyCode::Char('j'),
        "KeyK" => KeyCode::Char('k'),
        "KeyL" => KeyCode::Char('l'),
        "KeyM" => KeyCode::Char('m'),
        "KeyN" => KeyCode::Char('n'),
        "KeyO" => KeyCode::Char('o'),
        "KeyP" => KeyCode::Char('p'),
        "KeyQ" => KeyCode::Char('q'),
        "KeyR" => KeyCode::Char('r'),
        "KeyS" => KeyCode::Char('s'),
        "KeyT" => KeyCode::Char('t'),
        "KeyU" => KeyCode::Char('u'),
        "KeyV" => KeyCode::Char('v'),
        "KeyW" => KeyCode::Char('w'),
        "KeyX" => KeyCode::Char('x'),
        "KeyY" => KeyCode::Char('y'),
        "KeyZ" => KeyCode::Char('z'),
        _ => panic!("not yet implemented for {:?}", keycode),
    }
}

fn get_modifier(keyboard_event: &sauron::KeyboardEvent) -> Modifier {
    match keyboard_event.code().as_ref() {
        "AltLeft" | "AltRight" => Modifier::alt(),
        "ControlLeft" | "ControlRight" => Modifier::ctrl(),
        "ShiftLeft" | "ShiftRight" => Modifier::shift(),
        _ => Modifier::none(),
    }
}
