use crate::{Event, Node};

pub trait Component<MSG> {
    fn update(&mut self, msg: MSG);

    fn view(&self) -> Node<MSG>;
}
