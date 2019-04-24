use crate::Widget;
use sauron_vdom::{self, Callback, Event};
use std::rc::Rc;
use tui::{
    layout::{Alignment, Constraint, Corner, Direction},
    style::Style,
    widgets::Borders,
};

#[derive(Clone)]
pub enum TuiWidget<'a> {
    Layout(Layout<'a>),
    Paragraph(Paragraph<'a>),
    List(List<'a>),
    Block(Block<'a>),
}

impl<'a> TuiWidget<'a> {
    fn as_layout(&mut self) -> Option<&mut Layout<'a>> {
        match self {
            TuiWidget::Layout(layout) => Some(layout),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Layout<'a> {
    pub direction: Direction,
    pub margin: u16,
    pub constraints: Vec<Constraint>,
    pub children: Vec<TuiWidget<'a>>,
}

impl<'a> Default for Layout<'a> {
    fn default() -> Self {
        Layout {
            direction: Direction::Vertical,
            margin: 0,
            constraints: Vec::new(),
            children: vec![],
        }
    }
}

fn layout<'a, C>(direction: Direction, constraints: Vec<Constraint>, children: C) -> TuiWidget<'a>
where
    C: AsRef<[TuiWidget<'a>]>,
{
    let mut layout = Layout::default();
    layout.direction = direction;
    layout.constraints = constraints;
    for child in children.as_ref().iter() {
        layout.children.push(child.clone());
    }
    TuiWidget::Layout(layout)
}

#[derive(Clone)]
pub struct Paragraph<'a> {
    /// A block to wrap the widget in
    pub block: Option<Block<'a>>,
    /// Widget style
    pub style: Style,
    /// Wrap the text or not
    wrapping: bool,
    /// The text to display
    pub text: Vec<String>,
    /// Scroll
    pub scroll: u16,
    /// Aligenment of the text
    pub alignment: Alignment,
}

impl<'a> Default for Paragraph<'a> {
    fn default() -> Self {
        Paragraph {
            block: None,
            style: Style::default(),
            wrapping: true,
            text: vec![],
            scroll: 0,
            alignment: Alignment::Left,
        }
    }
}

fn paragraph<'a>(block: Option<Block<'a>>, text: Vec<String>) -> TuiWidget<'a> {
    let mut paragraph = Paragraph::default();
    paragraph.block = block;
    paragraph.text = text;
    TuiWidget::Paragraph(paragraph)
}

#[derive(Clone)]
pub struct List<'b> {
    block: Option<Block<'b>>,
    items: Vec<String>,
    style: Style,
    start_corner: Corner,
}

#[derive(Clone)]
pub struct Block<'a> {
    /// Optional title place on the upper left of the block
    pub title: Option<&'a str>,
    /// Title style
    pub title_style: Style,
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// Widget style
    pub style: Style,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Block {
            title: None,
            title_style: Style::default(),
            borders: Borders::ALL,
            border_style: Style::default(),
            style: Style::default(),
        }
    }
}

fn plain_block<'a>(title: &'a str) -> Block<'a> {
    let mut block: Block<'a> = Block::default();
    block.title = Some(title);
    block
}

fn widget_to_tui_node<'a>(widget: Widget) -> TuiWidget<'a> {
    match widget {
        Widget::Column => layout(Direction::Horizontal, vec![], []),
        Widget::Row => layout(Direction::Vertical, vec![], []),
        Widget::Button(txt) => paragraph(Some(plain_block("button")), vec![txt]),
        Widget::Text(txt) => paragraph(Some(plain_block("text")), vec![txt]),
        Widget::Block => paragraph(Some(plain_block("block")), vec!["Im a block".to_string()]),
    }
}
pub fn convert_widget_node_tree_to_tui_widget<'a, MSG>(
    widget_node: crate::Node<MSG>,
) -> TuiWidget<'a> {
    match widget_node {
        crate::Node::Element(element) => {
            let mut tui_node = widget_to_tui_node(element.tag);
            if let Some(mut layout) = tui_node.as_layout() {
                for child in element.children {
                    let tui_child = convert_widget_node_tree_to_tui_widget(child);
                    layout.children.push(tui_child);
                }
            }
            tui_node
        }
        crate::Node::Text(txt) => paragraph(None, vec![txt.text]),
    }
}
