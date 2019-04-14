use crate::WidgetNode;
use vdom::builder::element;
use vdom::builder::Attribute;

#[derive(Debug, Clone)]
pub enum Widget {
    View,
    Row,
    Button(String),
}

pub fn widget<'a, A, C>(widget: Widget, attrs: A, children: C) -> WidgetNode
where
    C: AsRef<[WidgetNode]>,
    A: AsRef<[Attribute<'a>]>,
{
    element(widget, attrs, children)
}

pub fn view<'a, A, C>(attrs: A, children: C) -> WidgetNode
where
    C: AsRef<[WidgetNode]>,
    A: AsRef<[Attribute<'a>]>,
{
    widget(Widget::View, attrs, children)
}

pub fn row<'a, A, C>(attrs: A, children: C) -> WidgetNode
where
    C: AsRef<[WidgetNode]>,
    A: AsRef<[Attribute<'a>]>,
{
    widget(Widget::Row, attrs, children)
}

pub fn button<'a, A>(attrs: A, txt: &str) -> WidgetNode
where
    A: AsRef<[Attribute<'a>]>,
{
    widget(Widget::Button(txt.to_string()), attrs, [])
}
