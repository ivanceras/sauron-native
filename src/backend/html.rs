use crate::{Backend, Component, Widget};
use sauron::{
    html::{attributes::*, div, input, text},
    Component as SauronComponent, DomUpdater, Program,
};
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

pub struct HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    app: APP,
    _phantom_data: PhantomData<MSG>,
}

pub struct HtmlBackend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    program: Rc<Program<HtmlApp<APP, MSG>, MSG>>,
}

impl<APP, MSG> HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) -> Self {
        HtmlApp {
            app,
            _phantom_data: PhantomData,
        }
    }
}

impl<APP, MSG> sauron::Component<MSG> for HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn update(&mut self, msg: MSG) {
        self.app.update(msg)
    }

    fn view(&self) -> sauron::Node<MSG> {
        let view = self.app.view();
        let html_view = widget_tree_to_html_node(view);
        html_view
    }
}

impl<APP, MSG> Backend<APP, MSG> for HtmlBackend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        let html_app = HtmlApp::new(app);
        let program = sauron::Program::mount_to_body(html_app);
        let backend = HtmlBackend { program };
        Rc::new(backend)
    }
}

/// convert Widget into an equivalent html node
fn widget_to_html<MSG>(widget: Widget) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget {
        Widget::Column => div(
            [style(
                "display:flexbox;\
                 flex-direction:column;\
                 border: 1px solid blue;\
                 ",
            )],
            [text("This is a Column")],
        ),
        Widget::Row => div(
            [style(
                "display:flexbox;\
                 flex-direction:row;\
                 border: 1px solid green;\
                 ",
            )],
            [text("This is a Row")],
        ),
        Widget::Button(txt) => input([r#type("button"), value(txt)], []),
        Widget::Text(txt) => text(&txt),
        Widget::Block => text("This is a block"),
    }
}

#[allow(unused)]
/// converts widget virtual node tree into an html node tree
pub fn widget_tree_to_html_node<MSG>(widget_node: crate::Node<MSG>) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget_node {
        crate::Node::Element(widget) => {
            // convert the Widget tag to html node
            let mut html_node: sauron::Node<MSG> = widget_to_html(widget.tag);
            // cast the html node to element
            if let Some(html_element) = html_node.as_element() {
                for widget_child in widget.children {
                    // convert all widget child to an html child node
                    let mut html_child: sauron::Node<MSG> = widget_tree_to_html_node(widget_child);
                    // attached as children to the html node, all of the widget's children
                    html_element.children.push(html_child);
                }

                // attach the attributes and event callbacks
                for (name, value) in &widget.attrs {
                    html_element.attrs.insert(name.to_string(), value.clone());
                }
                for (event, cb) in &widget.events {
                    html_element.events.insert(event.to_string(), cb.clone());
                }
            }
            html_node
        }
        crate::Node::Text(txt) => text(txt.text),
    }
}
