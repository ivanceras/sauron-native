use crate::{Attribute, Widget};
use itui::{
    layout::{Alignment, Constraint, Corner, Direction},
    style::Style,
    widgets::{Borders, Button},
};
use sauron_vdom::{self, Callback, Event};
use std::rc::Rc;

#[derive(Clone)]
pub enum TuiWidget<MSG> {
    Layout(Layout<MSG>),
    Paragraph(Paragraph<MSG>),
    Button(Button<MSG>),
}
#[derive(Clone)]
pub struct Paragraph<MSG> {
    /// A block to wrap the widget in
    pub block: Option<Block<MSG>>,
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
    /// events attached to this block
    pub events: Vec<Attribute<MSG>>,
}

#[derive(Clone)]
pub struct Layout<MSG> {
    pub direction: Direction,
    pub margin: u16,
    pub constraints: Vec<Constraint>,
    pub children: Vec<TuiWidget<MSG>>,
}

#[derive(Clone)]
pub struct List<MSG> {
    block: Option<Block<MSG>>,
    items: Vec<String>,
    style: Style,
    start_corner: Corner,
    /// events attached to this block
    pub events: Vec<Attribute<MSG>>,
}

#[derive(Clone)]
pub struct Block<MSG> {
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
    /// events attached to this block
    pub events: Vec<Attribute<MSG>>,
}

impl<MSG> TuiWidget<MSG> {
    fn as_layout(&mut self) -> Option<&mut Layout<MSG>> {
        match self {
            TuiWidget::Layout(layout) => Some(layout),
            _ => None,
        }
    }
}

impl<MSG> Layout<MSG> {
    fn add_children(&mut self, children: Vec<TuiWidget<MSG>>) {
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

impl<MSG> Default for Layout<MSG> {
    fn default() -> Self {
        Layout {
            direction: Direction::Vertical,
            margin: 0,
            constraints: Vec::new(),
            children: vec![],
        }
    }
}
impl<MSG> Default for Paragraph<MSG> {
    fn default() -> Self {
        Paragraph {
            block: None,
            style: Style::default(),
            wrapping: true,
            text: vec![],
            scroll: 0,
            alignment: Alignment::Left,
            events: vec![],
        }
    }
}

impl<MSG> Default for Block<MSG> {
    fn default() -> Self {
        Block {
            title: None,
            title_style: Style::default(),
            borders: Borders::ALL,
            border_style: Style::default(),
            style: Style::default(),
            preferred_constraint: None,
            events: vec![],
        }
    }
}

fn layout<MSG>(
    direction: Direction,
    constraints: Vec<Constraint>,
    children: Vec<TuiWidget<MSG>>,
) -> TuiWidget<MSG> {
    let mut layout = Layout::default();

    layout.direction = direction;
    layout.add_children(children);
    TuiWidget::Layout(layout)
}

fn paragraph<MSG>(
    events: Vec<Attribute<MSG>>,
    block: Option<Block<MSG>>,
    text: Vec<String>,
) -> TuiWidget<MSG> {
    let mut paragraph = Paragraph::default();
    paragraph.block = block;
    paragraph.text = text;
    paragraph.events = events;
    TuiWidget::Paragraph(paragraph)
}

fn button<MSG>(events: Vec<Attribute<MSG>>, text: &str) -> TuiWidget<MSG>
where
    MSG: 'static,
{
    let button = Button::new(events, text);
    TuiWidget::Button(button)
}

fn plain_block<MSG>(events: Vec<Attribute<MSG>>) -> Block<MSG> {
    let mut block: Block<MSG> = Block::default();
    block.events = events;
    block
}

fn widget_to_tui_node<MSG>(widget: Widget, attrs: Vec<Attribute<MSG>>) -> TuiWidget<MSG>
where
    MSG: 'static,
{
    let value_txt: String = if let Some(attr) = attrs.iter().find(|attr| attr.name == "value") {
        if let Some(value) = attr.get_value() {
            value.to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    match widget {
        Widget::Vbox => layout(Direction::Vertical, vec![], vec![]),
        Widget::Hbox => layout(Direction::Horizontal, vec![], vec![]),
        Widget::Button => button(attrs, &value_txt),
        Widget::Text(txt) => paragraph(attrs, Some(plain_block(vec![])), vec![txt]),
        Widget::TextInput(txt) => paragraph(attrs, Some(plain_block(vec![])), vec![txt]),
        Widget::Checkbox(label, value) => button(vec![], "X"),
        Widget::Radio(label, value) => button(vec![], "O"),
        Widget::Image(bytes) => button(vec![], "Image here soon..."),
    }
}
pub fn convert_widget_node_tree_to_tui_widget<'a, MSG>(
    widget_node: crate::Node<MSG>,
) -> TuiWidget<MSG> {
    match widget_node {
        crate::Node::Element(element) => {
            let mut tui_node = widget_to_tui_node(element.tag, element.attrs);
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
        crate::Node::Text(txt) => paragraph(vec![], None, vec![txt.text]),
    }
}
