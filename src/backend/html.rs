
use crate::widget::Widget;
use browser::html::attributes::*;
use browser::html::*;

impl Into<browser::Node> for Widget {
    fn into(self) -> browser::Node {
        match self {
            Widget::View => div(
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
            Widget::Button(txt) => button([], [text(txt)]),
        }
    }
}

impl Into<browser::Node> for crate::Node {
    fn into(self) -> browser::Node {
        match self.0 {
            vdom::Node::Element(element) => {
                let mut tag: browser::Node = element.tag.into();
                if let Some(elm) = tag.as_element() {
                    for child in element.children {
                        let child_node = crate::Node(child);
                        let cnode: browser::Node = child_node.into();
                        elm.children.push(cnode);
                    }
                }
                tag
            }
            vdom::Node::Text(txt) => text(txt.text),
        }
    }
}
