use crate::widget::attribute::event::{
    InputEvent, KeyCode, KeyEvent, Modifier, MouseEvent,
};
use crate::widget::attribute::Value;
use crate::Event;

//TODO
fn from_titik_value(t_value: titik::Value) -> Value {
    Value::from(false)
}

fn from_titik_key_code(tkk: titik::event::KeyCode) -> KeyCode {
    match tkk {
        titik::event::KeyCode::Backspace => KeyCode::Backspace,
        titik::event::KeyCode::Enter => KeyCode::Enter,
        titik::event::KeyCode::Left => KeyCode::Left,
        titik::event::KeyCode::Right => KeyCode::Right,
        titik::event::KeyCode::Up => KeyCode::Up,
        titik::event::KeyCode::Down => KeyCode::Down,
        titik::event::KeyCode::Home => KeyCode::Home,
        titik::event::KeyCode::End => KeyCode::End,
        titik::event::KeyCode::PageUp => KeyCode::PageUp,
        titik::event::KeyCode::PageDown => KeyCode::PageDown,
        titik::event::KeyCode::Tab => KeyCode::Tab,
        titik::event::KeyCode::BackTab => KeyCode::BackTab,
        titik::event::KeyCode::Delete => KeyCode::Delete,
        titik::event::KeyCode::Insert => KeyCode::Insert,
        titik::event::KeyCode::F(key) => KeyCode::F(key),
        titik::event::KeyCode::Char(ch) => KeyCode::Char(ch),
        titik::event::KeyCode::Null => KeyCode::Null,
        titik::event::KeyCode::Esc => KeyCode::Esc,
    }
}

fn from_titik_key_modifier(md: titik::event::KeyModifiers) -> Modifier {
    Modifier {
        alt_key: md.contains(titik::event::KeyModifiers::ALT),
        ctrl_key: md.contains(titik::event::KeyModifiers::CONTROL),
        shift_key: md.contains(titik::event::KeyModifiers::SHIFT),
    }
}

fn from_titik_key_event(tke: titik::event::KeyEvent) -> KeyEvent {
    KeyEvent {
        key_code: from_titik_key_code(tke.code),
        modifier: from_titik_key_modifier(tke.modifiers),
    }
}

// TODO:
pub fn from_titik(t_event: titik::Event) -> Event {
    match t_event {
        titik::Event::Key(ke) => {
            let key_event = from_titik_key_event(ke);
            Event::KeyEvent(key_event)
        }
        //TODO: put comprehensive conversion here
        titik::Event::Mouse(me) => match me {
            titik::event::MouseEvent::Down(_, _, _, _) => {
                Event::MouseEvent(MouseEvent::click(0, 0))
            }
            titik::event::MouseEvent::Up(_, _, _, _) => {
                Event::MouseEvent(MouseEvent::release(0, 0))
            }
            _ => todo!(),
        },
        titik::Event::InputEvent(ie) => {
            Event::InputEvent(InputEvent::from(from_titik_value(ie.value)))
        }
        titik::Event::Resize(width, height) => todo!(),
    }
}
