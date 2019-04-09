use super::Node;

pub trait View {
    fn view(&self) -> Node;
}
