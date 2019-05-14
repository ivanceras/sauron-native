use crate::Widget;
use sauron_vdom::{self, Callback, Event};
use std::rc::Rc;
use tui::{
    layout::{Alignment, Constraint, Corner, Direction},
    style::Style,
    widgets::Borders,
};

#[derive(Clone)]
pub enum TuiWidget {
    Layout(Layout),
    Paragraph(Paragraph),
    List(List),
    Block(Block),
}
#[derive(Clone)]
pub struct Paragraph {
    /// A block to wrap the widget in
    pub block: Option<Block>,
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

#[derive(Clone)]
pub struct Layout {
    pub direction: Direction,
    pub margin: u16,
    pub constraints: Vec<Constraint>,
    pub children: Vec<TuiWidget>,
}

#[derive(Clone)]
pub struct List {
    block: Option<Block>,
    items: Vec<String>,
    style: Style,
    start_corner: Corner,
}

#[derive(Clone)]
pub struct Block {
    /// Optional title place on the upper left of the block
    pub title: Option<String>,
    /// Title style
    pub title_style: Style,
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// Widget style
    pub style: Style,
    /// preferred constraint when parent is a layout
    pub preferred_constraint: Option<Constraint>,
}

impl TuiWidget {
    fn as_layout(&mut self) -> Option<&mut Layout> {
        match self {
            TuiWidget::Layout(layout) => Some(layout),
            _ => None,
        }
    }

    pub fn preferred_constraint(&self) -> Option<Constraint> {
        match self {
            TuiWidget::Block(block) => block.preferred_constraint,
            _ => None,
        }
    }
}

impl Layout {
    fn add_children(&mut self, children: Vec<TuiWidget>) {
        for child in children {
            self.children.push(child);
        }
        let child_count = self.children.len();
        if self.constraints.is_empty() {
            if child_count > 0 {
                let alloted = 100 / child_count as u16;
                let new_constraints = self
                    .children
                    .iter()
                    .map(|child| Constraint::Percentage(alloted))
                    .collect();
                self.constraints = new_constraints;
            }
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            direction: Direction::Vertical,
            margin: 0,
            constraints: Vec::new(),
            children: vec![],
        }
    }
}
impl Default for Paragraph {
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

impl Default for Block {
    fn default() -> Self {
        Block {
            title: None,
            title_style: Style::default(),
            borders: Borders::ALL,
            border_style: Style::default(),
            style: Style::default(),
            preferred_constraint: None,
        }
    }
}

fn layout<'a, C>(direction: Direction, constraints: Vec<Constraint>, children: C) -> TuiWidget
where
    C: AsRef<[TuiWidget]>,
{
    let mut layout = Layout::default();

    layout.direction = direction;
    let mut all_children: Vec<TuiWidget> = vec![];
    for child in children.as_ref() {
        all_children.push(child.clone());
    }
    layout.add_children(all_children);
    TuiWidget::Layout(layout)
}

fn paragraph(block: Option<Block>, text: Vec<String>) -> TuiWidget {
    let mut paragraph = Paragraph::default();
    paragraph.block = block;
    paragraph.text = text;
    TuiWidget::Paragraph(paragraph)
}

fn button(text: &str) -> TuiWidget {
    layout(
        Direction::Horizontal,
        vec![Constraint::Max(1), Constraint::Min(1)],
        [paragraph(Some(plain_block()), vec![text.to_string()])],
    )
}

fn plain_block() -> Block {
    let mut block: Block = Block::default();
    block
}

fn block(title: &str) -> TuiWidget {
    let mut block: Block = Block::default();
    block.title = Some(title.to_string());
    block.borders = Borders::ALL;
    TuiWidget::Block(block)
}

fn widget_to_tui_node(widget: Widget) -> TuiWidget {
    match widget {
        Widget::Vbox => layout(Direction::Vertical, vec![], []),
        Widget::Hbox => layout(Direction::Horizontal, vec![], []),
        Widget::Button(txt) => button(&txt),
        Widget::Text(txt) => paragraph(Some(plain_block()), vec![txt]),
        Widget::Block(title) => block(&*title),
    }
}
pub fn convert_widget_node_tree_to_tui_widget<'a, MSG>(widget_node: crate::Node<MSG>) -> TuiWidget {
    match widget_node {
        crate::Node::Element(element) => {
            let mut tui_node = widget_to_tui_node(element.tag);
            if let Some(mut layout) = tui_node.as_layout() {
                let mut children = vec![];
                for child in element.children {
                    let tui_child = convert_widget_node_tree_to_tui_widget(child);
                    children.push(tui_child);
                }
                layout.add_children(children);
            }
            tui_node
        }
        crate::Node::Text(txt) => paragraph(None, vec![txt.text]),
    }
}
