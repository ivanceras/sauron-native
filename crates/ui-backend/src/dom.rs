use std::ops::Deref;
use vdom::Callback;
use vdom::{self, diff, Value};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{self, Element, EventTarget, Node, Text};

use apply_patches::patch;
use std::collections::HashMap;
use std::sync::Mutex;
use web_sys::console;

mod apply_patches;

// Used to uniquely identify elements that contain closures so that the DomUpdater can
// look them up by their unique id.
// When the DomUpdater sees that the element no longer exists it will drop all of it's
// Rc'd Closures for those events.
use lazy_static::lazy_static;
lazy_static! {
    static ref ELEM_UNIQUE_ID: Mutex<u32> = Mutex::new(0);
}

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
pub struct CreatedNode<T> {
    /// A `Node` or `Element` that was created from a `Node`
    pub node: T,
    closures: ActiveClosure,
}

/// Used for keeping a real DOM node up to date based on the current Node
/// and a new incoming Node that represents our latest DOM state.
pub struct DomUpdater {
    current_vdom: vdom::Node,
    root_node: Node,

    /// The closures that are currently attached to elements in the page.
    ///
    /// We keep these around so that they don't get dropped (and thus stop working);
    ///
    /// FIXME: Drop them when the element is no longer in the page. Need to figure out
    /// a good strategy for when to do this.
    pub active_closures: ActiveClosure,
}

pub type ActiveClosure = HashMap<u32, Vec<Closure<FnMut()>>>;

impl<T> CreatedNode<T> {
    pub fn without_closures<N: Into<T>>(node: N) -> Self {
        CreatedNode {
            node: node.into(),
            closures: HashMap::with_capacity(0),
        }
    }

    pub fn create_text_node(text: &vdom::Text) -> Text {
        let document = web_sys::window().unwrap().document().unwrap();
        document.create_text_node(&text.text)
    }

    /// Create and return a `CreatedNode` instance (containing a DOM `Node`
    /// together with potentially related closures) for this virtual node.
    pub fn create_dom_node(vnode: &vdom::Node) -> CreatedNode<Node> {
        match vnode {
            vdom::Node::Text(text_node) => {
                CreatedNode::without_closures(Self::create_text_node(text_node))
            }
            vdom::Node::Element(element_node) => {
                let created_element: CreatedNode<Node> =
                    Self::create_element_node(element_node).into();
                created_element
            }
        }
    }

    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    pub fn create_element_node(velem: &vdom::Element) -> CreatedNode<Element> {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = if let Some(ref namespace) = velem.namespace {
            document
                .create_element_ns(Some(namespace), &velem.tag)
                .unwrap()
        } else {
            document.create_element(&velem.tag).unwrap()
        };

        let mut closures = HashMap::new();

        velem.attrs.iter().for_each(|(name, value)| {
            element
                .set_attribute(name, &value.to_string())
                .expect("Set element attribute in create element");
        });

        if velem.events.len() > 0 {
            let unique_id = create_unique_identifier();

            element
                .set_attribute("data-vdom-id".into(), &unique_id.to_string())
                .expect("Could not set attribute on element");

            closures.insert(unique_id, vec![]);

            velem
                .events
                .iter()
                .for_each(|(event, callback): (&String, &Callback<Value>)| {
                    let current_elem: &EventTarget = element.dyn_ref().unwrap();

                    let callback_clone = callback.clone();

                    let closure_wrap: Closure<FnMut()> = Closure::wrap(Box::new(move || {
                        console::log_1(&"Im triggered here...".into());
                        let value: Value = "hi".into();
                        callback_clone.emit(value);
                    }));

                    current_elem
                        .add_event_listener_with_callback(
                            event,
                            closure_wrap.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    closures
                        .get_mut(&unique_id)
                        .unwrap()
                        .push(closure_wrap);

                });
        }

        let mut previous_node_was_text = false;

        velem.children.iter().for_each(|child| {
            match child {
                vdom::Node::Text(text_node) => {
                    let current_node = element.as_ref() as &web_sys::Node;

                    // We ensure that the text siblings are patched by preventing the browser from merging
                    // neighboring text nodes. Originally inspired by some of React's work from 2016.
                    //  -> https://reactjs.org/blog/2016/04/07/react-v15.html#major-changes
                    //  -> https://github.com/facebook/react/pull/5753
                    //
                    // `ptns` = Percy text node separator
                    if previous_node_was_text {
                        let separator = document.create_comment("ptns");
                        current_node
                            .append_child(separator.as_ref() as &web_sys::Node)
                            .unwrap();
                    }

                    current_node
                        .append_child(&Self::create_text_node(&text_node))
                        .unwrap();

                    previous_node_was_text = true;
                }
                vdom::Node::Element(element_node) => {
                    previous_node_was_text = false;

                    let child = Self::create_element_node(element_node);
                    let child_elem: Element = child.node;
                    closures.extend(child.closures);

                    element.append_child(&child_elem).unwrap();
                }
            }
        });

        CreatedNode {
            node: element,
            closures,
        }
    }
}

impl DomUpdater {
    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created but not added to your DOM.
    pub fn new(current_vdom: vdom::Node) -> DomUpdater {
        let created_node = CreatedNode::<Node>::create_dom_node(&current_vdom);
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
        }
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and appended (as a child) to your passed
    /// in mount element.
    pub fn new_append_to_mount(current_vdom: vdom::Node, mount: &Element) -> DomUpdater {
        let created_node: CreatedNode<Node> = CreatedNode::<Node>::create_dom_node(&current_vdom);
        mount
            .append_child(&created_node.node)
            .expect("Could not append child to mount");
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
        }
    }

    /// Create a new `DomUpdater`.
    ///
    /// A root `Node` will be created and it will replace your passed in mount
    /// element.
    pub fn new_replace_mount(current_vdom: vdom::Node, mount: Element) -> DomUpdater {
        let created_node = CreatedNode::<Node>::create_dom_node(&current_vdom);
        mount
            .replace_with_with_node_1(&created_node.node)
            .expect("Could not replace mount element");
        DomUpdater {
            current_vdom,
            root_node: created_node.node,
            active_closures: created_node.closures,
        }
    }

    /// Diff the current virtual dom with the new virtual dom that is being passed in.
    ///
    /// Then use that diff to patch the real DOM in the user's browser so that they are
    /// seeing the latest state of the application.
    pub fn update(&mut self, new_vdom: vdom::Node) {
        let patches = diff(&self.current_vdom, &new_vdom);
        let active_closures = patch(self.root_node.clone(), &patches).unwrap();
        self.active_closures.extend(active_closures);
        self.current_vdom = new_vdom;
    }

    /// Return the root node of your application, the highest ancestor of all other nodes in
    /// your real DOM tree.
    pub fn root_node(&self) -> Node {
        // Note that we're cloning the `web_sys::Node`, not the DOM element.
        // So we're effectively cloning a pointer here, which is fast.
        self.root_node.clone()
    }
}

fn create_unique_identifier() -> u32 {
    let mut elem_unique_id = ELEM_UNIQUE_ID.lock().unwrap();
    *elem_unique_id += 1;
    *elem_unique_id
}

impl From<CreatedNode<Element>> for CreatedNode<Node> {
    fn from(other: CreatedNode<Element>) -> CreatedNode<Node> {
        CreatedNode {
            node: other.node.into(),
            closures: other.closures,
        }
    }
}

impl<T> Deref for CreatedNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
