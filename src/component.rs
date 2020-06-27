use crate::Node;

/// Applications must implement Component
pub trait Component<MSG> {
    /// updates the state of the Application based on the supplied msg
    fn update(&mut self, msg: MSG);

    /// returns a Node tree for rendering the view
    fn view(&self) -> Node<MSG>;
}
