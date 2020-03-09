use std::{
    convert::AsRef,
    iter::{self, Iterator},
};

use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    widgets::{Block, Text, Widget},
};
use sauron_vdom::{Attribute, Event};

pub struct List<'b, L, MSG>
where
    L: Iterator<Item = Text<'b>>,
{
    block: Option<Block<'b, MSG>>,
    items: L,
    style: Style,
    start_corner: Corner,
    area: Rect,
    /// events attached to this block
    pub events: Vec<Attribute<&'static str, Event, MSG>>,
}

impl<'b, L, MSG> Default for List<'b, L, MSG>
where
    L: Iterator<Item = Text<'b>> + Default,
{
    fn default() -> Self {
        List {
            block: None,
            items: L::default(),
            style: Default::default(),
            start_corner: Corner::TopLeft,
            area: Default::default(),
            events: vec![],
        }
    }
}

impl<'b, L, MSG> List<'b, L, MSG>
where
    L: Iterator<Item = Text<'b>>,
{
    pub fn new(items: L) -> Self {
        List {
            block: None,
            items,
            style: Default::default(),
            start_corner: Corner::TopLeft,
            area: Default::default(),
            events: vec![],
        }
    }

    pub fn block(mut self, block: Block<'b, MSG>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = Text<'b>, IntoIter = L>,
    {
        self.items = items.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> Self {
        self.start_corner = corner;
        self
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }
}

impl<'b, L, MSG> Widget for List<'b, L, MSG>
where
    L: Iterator<Item = Text<'b>>,
    MSG: 'static,
{
    fn get_area(&self) -> Rect {
        self.area
    }
    fn draw(&mut self, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => {
                b.draw(buf);
                b.inner()
            }
            None => self.area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        self.background(buf, self.style.bg);

        for (i, item) in self
            .items
            .by_ref()
            .enumerate()
            .take(list_area.height as usize)
        {
            let (x, y) = match self.start_corner {
                Corner::TopLeft => (list_area.left(), list_area.top() + i as u16),
                Corner::BottomLeft => (list_area.left(), list_area.bottom() - (i + 1) as u16),
                // Not supported
                _ => (list_area.left(), list_area.top() + i as u16),
            };
            match item {
                Text::Raw(ref v) => {
                    buf.set_stringn(x, y, v, list_area.width as usize, Style::default());
                }
                Text::Styled(ref v, s) => {
                    buf.set_stringn(x, y, v, list_area.width as usize, s);
                }
            };
        }
    }
}

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use itui::widgets::{Block, Borders, SelectableList};
/// # use itui::style::{Style, Color, Modifier};
/// # fn main() {
/// SelectableList::default()
///     .block(Block::default().title("SelectableList").borders(Borders::ALL))
///     .items(&["Item 1", "Item 2", "Item 3"])
///     .select(Some(1))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// # }
/// ```
pub struct SelectableList<'b, MSG> {
    block: Option<Block<'b, MSG>>,
    /// Items to be displayed
    items: Vec<&'b str>,
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'b str>,
    area: Rect,
    /// events attached to this block
    pub events: Vec<Attribute<&'static str, Event, MSG>>,
}

impl<'b, MSG> Default for SelectableList<'b, MSG> {
    fn default() -> Self {
        SelectableList {
            block: None,
            items: Vec::new(),
            selected: None,
            style: Default::default(),
            highlight_style: Default::default(),
            highlight_symbol: None,
            area: Default::default(),
            events: vec![],
        }
    }
}

impl<'b, MSG> SelectableList<'b, MSG> {
    pub fn block(mut self, block: Block<'b, MSG>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: &'b [I]) -> Self
    where
        I: AsRef<str> + 'b,
    {
        self.items = items.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'b str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }
}

impl<'b, MSG> Widget for SelectableList<'b, MSG>
where
    MSG: Clone + 'static,
{
    fn get_area(&self) -> Rect {
        self.area
    }
    fn draw(&mut self, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => b.inner(),
            None => self.area,
        };

        let list_height = list_area.height as usize;

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (Some(i), self.highlight_style),
            None => (None, self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();
        // Make sure the list show the selected item
        let offset = if let Some(selected) = selected {
            if selected >= list_height {
                selected - list_height + 1
            } else {
                0
            }
        } else {
            0
        };

        // Render items
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if let Some(s) = selected {
                    if i == s {
                        Text::styled(format!("{} {}", highlight_symbol, item), highlight_style)
                    } else {
                        Text::styled(format!("{} {}", blank_symbol, item), self.style)
                    }
                } else {
                    Text::styled(item, self.style)
                }
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.clone().unwrap_or_default())
            .style(self.style)
            .draw(buf);
    }
}
