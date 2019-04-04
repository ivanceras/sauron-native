use crate::vnode::Node;

pub trait View<F:Fn()> {

    fn render(&self) -> Node<F>;
}
