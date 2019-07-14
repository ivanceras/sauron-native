use crate::{Event, Node};

pub trait Component<MSG> {
    fn update(&mut self, msg: MSG);

    fn on_event(&mut self, event: Event) {
        // only tui backend use this
    }

    fn debug(&mut self, s: String) {
        // only tui backend use this
    }

    fn view(&self) -> Node<MSG>;
}
