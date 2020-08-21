//! functionalities specific to titik ui
//!
use crate::{
    widget::{
        attribute::{find_callback, find_value},
        Widget,
    },
    AttribKey, Attribute, Backend, Component, Node,
};
use image::GenericImageView;
use mt_dom::Callback;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self},
    marker::PhantomData,
    rc::Rc,
};
use titik::{
    renderer::Renderer, Button, Checkbox, Dispatch, FlexBox, GroupBox, Image,
    Link, Radio, TextArea, TextInput, TextLabel, Widget as Control,
};

mod apply_patches;
mod convert_event;
mod convert_widget;

/// Titik Backend
pub struct TitikBackend<APP, MSG>
where
    MSG: 'static,
{
    app: Rc<RefCell<APP>>,
    current_dom: Rc<RefCell<Node<MSG>>>,
    _phantom_msg: PhantomData<MSG>,
}

impl<APP, MSG> TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
}

impl<APP, MSG> Backend<APP, MSG> for TitikBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Debug + 'static,
{
    fn init(app: APP) {
        let mut stdout = io::stdout();
        let vdom = app.view();
        let current_dom = app.view();
        let mut root_node = convert_widget::from_node_tree(vdom);

        let backend = TitikBackend {
            app: Rc::new(RefCell::new(app)),
            current_dom: Rc::new(RefCell::new(current_dom)),
            _phantom_msg: PhantomData,
        };
        let mut renderer =
            Renderer::new(&mut stdout, Some(&backend), root_node.as_mut());
        renderer.run().expect("must run");
    }
}

impl<APP, MSG> Dispatch<MSG> for TitikBackend<APP, MSG>
where
    MSG: Debug + 'static,
    APP: Component<MSG> + 'static,
{
    /// root_node is added as argument in this dispatch function so that they are in the same
    /// borrow, otherwise an AlreadyBorrowedError will be invoke at runtime.
    fn dispatch(&self, msg: MSG, root_node: &mut dyn titik::Widget<MSG>) {
        eprintln!("dispatching... {:?}", msg);
        self.app.borrow_mut().update(msg);
        let new_view = self.app.borrow().view();
        let current_view = self.app.borrow().view();

        {
            let previous_dom = self.current_dom.borrow();
            let diff = mt_dom::diff_with_key(
                &previous_dom,
                &new_view,
                &AttribKey::Key,
            );
            eprintln!("diff: {:#?}", diff);
            apply_patches::apply_patches(&self, root_node, &diff);
        }

        *self.current_dom.borrow_mut() = current_view;
    }
}
