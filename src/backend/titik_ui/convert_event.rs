use crate::widget::attribute::event::{InputEvent, KeyEvent, MouseEvent};
use crate::widget::attribute::Value;
use crate::Event;

//TODO
fn from_titik_value(t_value: titik::Value) -> Value {
    Value::from(false)
}

// TODO:
pub fn from_titik(t_event: titik::Event) -> Event {
    match t_event {
        titik::Event::Key(ke) => {
            let key_event = KeyEvent::new("todo!".to_string());
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
