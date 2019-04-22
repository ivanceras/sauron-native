use crate::Component;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

/// Holds the app and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
pub struct Program<APP, MSG> {
    pub app: Rc<RefCell<APP>>,
    _phantom_data: PhantomData<MSG>,
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    /// Create an Rc wrapped instance of program, initializing DomUpdater with the initial view
    /// and root node, but doesn't mount it yet.
    pub fn new(app: APP) -> Rc<Self> {
        let program = Program {
            app: Rc::new(RefCell::new(app)),
            _phantom_data: PhantomData,
        };
        Rc::new(program)
    }

    /// This is called when an event is triggered in the html DOM.
    fn dispatch(self: &Rc<Self>, msg: MSG) {
        self.app.borrow_mut().update(msg);
        let _view = self.app.borrow().view();
    }
}
