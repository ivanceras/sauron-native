use crate::vnode::VNode;

pub trait View<F:Fn()> {

    fn render(&self) -> VNode<F>;
}
