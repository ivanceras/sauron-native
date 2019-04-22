use crate::widget::Widget;
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

fn widget_node_tree_to_tui_widget<'t, MSG>(widget_node: crate::Node<MSG>) -> TuiWidget<'t> {
    match widget_node {
        crate::Node::Element(widget) => {
            let mut tui_node: TuiWidget = widget_to_tui_widget(widget.tag);
            if let Some(tui_element) = widget_node.as_element() {
                for widget_child in widget.children {
                    let mut tui_child: TuiWidget = widget_node_tree_to_tui_widget(widget_child);
                    if let Some(child_element) = tui_child.as_element() {
                        for (name, value) in &child_element.attrs {
                            println!(
                                "What to do with {}={} in an {} tui widget",
                                name, value, child_element
                            );
                        }
                        for (event, cb) in &child_element.events {
                            println!(
                                "What to do with event {} in {} tui widget",
                                event, child_element
                            );
                        }
                    }
                }
            }
            tui_node
        }
        crate::Node::Text(txt) => TuiWidget::Text(Text::raw(txt.text)),
    }
}
