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
            Widget::Column => TuiWidget::Layout(Layout::default().direction(Direction::Vertical)),
            Widget::Row => TuiWidget::Layout(Layout::default().direction(Direction::Horizontal)),
            Widget::Button(txt) => TuiWidget::Text(txt),
            Widget::Text(txt) => TuiWidget::Text(txt),
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
