use crate::{backend::Backend, Component, Node};
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

/// Holds the app and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Program<APP, MSG, B> {
    backend: B,
    _phantom_data: PhantomData<MSG>,
    _phantom_app: PhantomData<APP>,
}

impl<APP, MSG, B> Program<APP, MSG, B>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
    B: Backend<APP, MSG>,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(app: APP) -> Rc<Self> {
        let program = Program {
            backend: B::init(app),
            _phantom_data: PhantomData,
            _phantom_app: PhantomData,
        };
        Rc::new(program)
    }
}
