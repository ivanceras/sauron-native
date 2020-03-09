use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::{line, rounded},
    widgets::{Borders, Widget},
};
use sauron_vdom::{Attribute, Callback, Event};

/// Base widget to be used with all upper level ones. It may be used to display a box border around
/// the widget and/or add a text.
///
/// # Examples
///
/// ```
/// # use itui::widgets::{Button, Borders};
/// # use itui::style::{Style, Color};
/// # fn main() {
/// Button::default()
///     .text("Button")
///     .title_style(Style::default().fg(Color::Red))
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .style(Style::default().bg(Color::Black));
/// # }
/// ```
#[derive(Clone)]
pub struct Button<MSG> {
    /// Optional text place on the upper left of the block
    pub text: String,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: Borders,
    /// Border style
    border_style: Style,
    /// whether to use rounded border or not
    rounded_border: bool,
    /// Widget style
    style: Style,
    /// area of the block,
    area: Rect,
    /// events attached to this block
    pub events: Vec<Attribute<&'static str, Event, MSG>>,
}

impl<MSG> Default for Button<MSG> {
    fn default() -> Self {
        Button {
            text: String::new(),
            title_style: Default::default(),
            borders: Borders::ALL,
            border_style: Default::default(),
            rounded_border: true,
            style: Default::default(),
            area: Default::default(),
            events: vec![],
        }
    }
}

impl<MSG> Button<MSG>
where
    MSG: 'static,
{
    pub fn new(events: Vec<Attribute<&'static str, Event, MSG>>, label: &str) -> Self {
        Button::default().events(events).label(label)
    }
    pub fn label(mut self, label: &str) -> Self {
        self.text = label.to_string();
        self
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }

    pub fn events(mut self, events: Vec<Attribute<&'static str, Event, MSG>>) -> Self {
        self.events = events;
        self
    }

    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn borders(mut self, flag: Borders) -> Self {
        self.borders = flag;
        self
    }
    pub fn triggers_event(&self, event: &Event) -> Option<&Callback<Event, MSG>> {
        match event {
            Event::MouseEvent(me) => {
                let x = me.coordinate.x();
                let y = me.coordinate.y();
                if self.area.hit(x, y) {
                    for listener in &self.events {
                        if me.r#type == listener.name {
                            return listener.get_callback();
                        }
                    }
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Compute the inner area of a block based on its border visibility rules.
    pub fn inner(&self) -> Rect {
        let mut inner = self.area;
        if self.borders.intersects(Borders::LEFT) {
            inner.x += 1;
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::TOP) {
            inner.y += 1;
            inner.height -= 1;
        }
        if self.borders.intersects(Borders::RIGHT) {
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::BOTTOM) {
            inner.height -= 1;
        }
        inner
    }

    fn get_top_left(&self) -> &'static str {
        if self.rounded_border {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        }
    }

    fn get_top_right(&self) -> &'static str {
        if self.rounded_border {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        }
    }

    fn get_bottom_left(&self) -> &'static str {
        if self.rounded_border {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        }
    }

    fn get_bottom_right(&self) -> &'static str {
        if self.rounded_border {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        }
    }
}

impl<MSG> Widget for Button<MSG>
where
    MSG: 'static,
{
    fn get_area(&self) -> Rect {
        self.area
    }

    fn draw(&mut self, buf: &mut Buffer) {
        self.background(buf, self.style.bg);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in self.area.top()..self.area.bottom() {
                buf.get_mut(self.area.left(), y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in self.area.left()..self.area.right() {
                buf.get_mut(x, self.area.top())
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = self.area.right() - 1;
            for y in self.area.top()..self.area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = self.area.bottom() - 1;
            for x in self.area.left()..self.area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(self.area.left(), self.area.top())
                .set_symbol(self.get_top_left())
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(self.area.right() - 1, self.area.top())
                .set_symbol(self.get_top_right())
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(self.area.left(), self.area.bottom() - 1)
                .set_symbol(self.get_bottom_left())
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(self.area.right() - 1, self.area.bottom() - 1)
                .set_symbol(self.get_bottom_right())
                .set_style(self.border_style);
        }

        if self.area.width > 2 {
            let lx = if self.borders.intersects(Borders::LEFT) {
                1
            } else {
                0
            };
            let rx = if self.borders.intersects(Borders::RIGHT) {
                1
            } else {
                0
            };
            let width = self.area.width - lx - rx;
            buf.set_stringn(
                self.area.left() + lx,
                self.area.top() + 1,
                &self.text,
                width as usize,
                self.title_style,
            );
        }
    }
}
