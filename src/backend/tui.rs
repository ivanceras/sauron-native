use crate::widget::Widget;
use std::fmt::Debug;
use tui::{
    layout::{Direction, Layout},
    widgets::Text,
};

pub enum TuiWidget<'t> {
    Layout(Layout),
    Widget(Box<tui::widgets::Widget>),
    Text(Text<'t>),
}

fn widget_to_tui_widget<'t>(widget: crate::Widget) -> TuiWidget<'t> {
    match widget {
        Widget::Column => TuiWidget::Layout(Layout::default().direction(Direction::Vertical)),
        Widget::Row => TuiWidget::Layout(Layout::default().direction(Direction::Horizontal)),
        Widget::Button(txt) => TuiWidget::Text(Text::raw(txt)),
        Widget::Text(txt) => TuiWidget::Text(Text::raw(txt)),
    }
}

#[allow(unused)]
pub fn widget_node_tree_to_tui_widget<'t, MSG>(widget_node: crate::Node<MSG>) -> TuiWidget<'t>
where
    MSG: Clone + Debug + 'static,
{
    match widget_node {
        crate::Node::Element(widget) => {
            let tui_node: TuiWidget = widget_to_tui_widget(widget.tag);
            for widget_child in widget.children {
                let mut _tui_child: TuiWidget = widget_node_tree_to_tui_widget(widget_child);
                for (name, value) in &widget.attrs {
                    println!("What to do with {}={} in an tui widget", name, value);
                }
                for (event, _cb) in &widget.events {
                    println!("What to do with event {} in tui widget", event,);
                }
            }
            tui_node
        }
        crate::Node::Text(txt) => TuiWidget::Text(Text::raw(txt.text)),
    }
}
