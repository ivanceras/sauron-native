use crate::Node;

/// Applications must implement Component
pub trait Component<MSG> {
    /// returns a string that sets the title of the app
    fn title(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }

    /// updates the state of the Application based on the supplied msg
    fn update(&mut self, msg: MSG);

    /// returns a Node tree for rendering the view
    fn view(&self) -> Node<MSG>;
}
