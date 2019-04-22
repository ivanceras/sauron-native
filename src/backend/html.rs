use crate::Widget;
use sauron::html::{attributes::*, div, input, text};
use std::fmt::Debug;

/// convert Widget into an equivalent html node
fn widget_to_html<MSG>(widget: Widget) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget {
        Widget::Column => div(
            [style(
                "display:flexbox;\
                 flex-direction:column",
            )],
            [],
        ),
        Widget::Row => div(
            [style(
                "display:flexbox;\
                 flex-direction:row",
            )],
            [],
        ),
        Widget::Button(txt) => input([r#type("button"), value(txt)], []),
        Widget::Text(txt) => text(&txt),
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
