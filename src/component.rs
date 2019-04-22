use crate::Node;

pub trait Component<MSG> {
    fn update(&mut self, msg: MSG);

    fn view(&self) -> Node<MSG>;
}
