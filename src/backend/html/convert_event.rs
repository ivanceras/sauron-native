use crate::widget::attribute::event::{Coordinate, InputEvent, Modifier, MouseButton, MouseEvent};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};

/// convert html mouse event to sauron native Event
pub fn from_mouse_event(event: sauron::web_sys::MouseEvent) -> MouseEvent {
    let mouse: &web_sys::MouseEvent = event.dyn_ref().expect("Unable to cast to mouse event");

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
        meta_key: mouse.meta_key(),
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
    InputEvent::new(input_event.value.to_string())
}
