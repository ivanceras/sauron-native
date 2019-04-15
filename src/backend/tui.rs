use crate::widget::Widget;
use crate::Node;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::widgets::Text;

pub enum TuiWidget<'t> {
    Layout(Layout),
    Widget(Box<tui::widgets::Widget>),
    Text(Text<'t>),
}

impl<'t> Into<TuiWidget<'t>> for crate::widget::Widget {
    fn into(self) -> TuiWidget<'t> {
        match self {
            Widget::Column => TuiWidget::Layout(Layout::default().direction(Direction::Vertical)),
            Widget::Row => TuiWidget::Layout(Layout::default().direction(Direction::Horizontal)),
            Widget::Button(txt) => TuiWidget::Text(Text::raw(txt)),
            Widget::Text(txt) => TuiWidget::Text(Text::raw(txt)),
        }
    }
}

impl<'t> Into<TuiWidget<'t>> for Node {
    fn into(self) -> TuiWidget<'t> {
        match self.0 {
            vdom::Node::Element(velm) => {
                let mut tag: TuiWidget = velm.tag.into();
                tag
            }
            vdom::Node::Text(txt) => TuiWidget::Text(Text::raw(txt.text)),
        }
    }
}
