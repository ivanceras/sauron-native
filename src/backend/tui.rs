use crate::widget::Widget;
use std::fmt::Debug;
use tui::{
    layout::{Direction, Layout},
    widgets::Text,
};

pub enum TuiWidget<'t,W> 
    where W: tui::widgets::Widget,
{
    Layout(Layout),
    Widget(W),
    Text(Text<'t>),
}

impl <'t,W>TuiWidget<'t,W>
    where W: tui::widgets::Widget,
{

    /*
    fn add_children<B>(&mut self, children: Vec<TuiWidget<'t, W>>, frame: &mut tui::Frame<B>)
        where B: tui::backend::Backend,
    {
        if let Some(layout) = self.as_layout(){
            let chunks = layout.split(frame.size());
            for (i,child) in children.into_iter().enumerate(){
                if let Some(child_layout) = child.as_layout(){
                    child_layout.split(chunks[i]);
                }

                if let Some(tui_widget) = self.as_tui_widget(){
                    tui_widget.render(frame, chunks[i]);
                }
            }
        }
    }
    */

    fn as_layout(self) -> Option<tui::layout::Layout>{
        match self{
            TuiWidget::Layout(layout) => Some(layout),
            _ => None,
        }
    }

    fn as_tui_widget(self) -> Option<W>{
        match self{
            TuiWidget::Widget(tui_widget) => Some(tui_widget),
            _ => None,
        }
    }

    fn as_tui_text(self) -> Option<tui::widgets::Text<'t>> {
        match self{
            TuiWidget::Text(text) => Some(text),
            _ => None,
        }
    }

}

fn widget_to_tui_widget<'t,W>(widget: crate::Widget) -> TuiWidget<'t,W> 
    where W: tui::widgets::Widget,
{
    match widget {
        Widget::Column => TuiWidget::Layout(Layout::default().direction(Direction::Vertical)),
        Widget::Row => TuiWidget::Layout(Layout::default().direction(Direction::Horizontal)),
        Widget::Button(txt) => TuiWidget::Text(Text::raw(txt)),
        Widget::Text(txt) => TuiWidget::Text(Text::raw(txt)),
    }
}

#[allow(unused)]
pub fn widget_node_tree_to_tui_widget<'t, W,MSG>(widget_node: crate::Node<MSG>) -> TuiWidget<'t,W>
where
    MSG: Clone + Debug + 'static,
    W: tui::widgets::Widget,
{
    match widget_node {
        crate::Node::Element(widget) => {
            let tui_node:TuiWidget<W> = widget_to_tui_widget(widget.tag);
            for widget_child in widget.children {
                let mut _tui_child:TuiWidget<W> = widget_node_tree_to_tui_widget(widget_child);
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


/// draw this widget node tree into the terminal buffer
pub fn draw_widget_node_tree<'t, W,MSG,B>(widget_node: crate::Node<MSG>, frame: &mut tui::Frame<B>)
        where B: tui::backend::Backend,
{
}
