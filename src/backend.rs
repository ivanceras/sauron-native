pub mod html {
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

}

pub mod tui {

    use crate::widget::Widget;
    use crate::Node;
    use tui::layout::Direction;
    use tui::layout::Layout;

    pub enum TuiWidget {
        Layout(Layout),
        Widget(Box<tui::widgets::Widget>),
        Text(String),
    }

    impl Into<TuiWidget> for crate::widget::Widget {
        fn into(self) -> TuiWidget {
            match self {
                Widget::View => TuiWidget::Layout(Layout::default().direction(Direction::Vertical)),
                Widget::Row => {
                    TuiWidget::Layout(Layout::default().direction(Direction::Horizontal))
                }
                Widget::Button(txt) => TuiWidget::Text(txt),
            }
        }
    }

    impl From<Node> for TuiWidget {
        fn from(node: Node) -> Self {
            match node.0 {
                vdom::Node::Element(element) => element.tag.into(),
                vdom::Node::Text(txt) => TuiWidget::Text(txt.text),
            }
        }
    }

}
