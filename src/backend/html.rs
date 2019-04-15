use crate::widget::Widget;
use browser::html::attributes::*;
use browser::html::*;

impl Into<browser::Node> for Widget {
    fn into(self) -> browser::Node {
        match self {
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
            Widget::Text(txt) => text(txt),
        }
    }
}

impl Into<browser::Node> for crate::Node {
    fn into(self) -> browser::Node {
        match self.0 {
            vdom::Node::Element(velm) => {
                let mut tag: browser::Node = velm.tag.into();
                if let Some(element) = tag.as_element() {
                    for child in velm.children {
                        let child_node = crate::Node(child);
                        let cnode: browser::Node = child_node.into();
                        element.children.push(cnode);
                    }
                    for (att, att_value) in velm.attrs {
                        element.attrs.insert(att, att_value);
                    }
                    for (evt, callback) in velm.events {
                        element.events.insert(evt, callback);
                    }
                }
                tag
            }
            vdom::Node::Text(txt) => text(txt.text),
        }
    }
}
