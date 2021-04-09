use crate::widget::attribute;
use crate::widget::attribute::style;
use crate::widget::attribute::util::find_value;
use crate::Widget;
use crate::{AttribKey, Attribute, Node, Value};
use expanse::geometry::Size;
use expanse::style::Dimension;
use expanse::style::FlexDirection;
use expanse::style::PositionType;
use expanse::style::Style;
use mt_dom::{attr, element};

/// a helper function to create widget elements
pub fn widget<MSG>(
    widget: Widget,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    element(widget, attrs, children)
}

/// a vertically oriented flexbox
pub fn column<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Column,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Vbox, attrs, children)
}

/// create a horizontally oriented flexbox
pub fn row<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Row,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Hbox, attrs, children)
}

/// create a vertically oriented resizable flexbox
pub fn vpane<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Column,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Vpane, attrs, children)
}

/// create a horizontally oriented resizable flexbox
pub fn hpane<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        flex_direction: FlexDirection::Row,
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Hpane, attrs, children)
}

/// overlay can be on top of other widgets
pub fn overlay<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    mut children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    children.iter_mut().for_each(|child| {
        child.add_attributes_ref_mut(vec![attr(
            AttribKey::PositionType,
            Value::from(PositionType::Absolute),
        )]);
    });

    attrs.push(style(Style {
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Overlay, attrs, children)
}

/// group widges together will a visible label and border enclosure
pub fn groupbox<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::GroupBox, attrs, children)
}

/// create a button
pub fn button<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        ..Default::default()
    }));
    widget(Widget::Button, attrs, vec![])
}

/// create a text paragraph
pub fn paragraph<MSG>(txt: &str) -> Node<MSG>
where
    MSG: 'static,
{
    widget(
        Widget::Paragraph,
        vec![
            attribute::value(txt.to_string()),
            style(Style {
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                ..Default::default()
            }),
        ],
        vec![],
    )
}

/// create a text input
pub fn text_input<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::TextInput, attrs, vec![])
}

/// create a checkbox control
pub fn checkbox<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    widget(Widget::Checkbox, attrs, vec![])
}

/// create a radio control
pub fn radio<MSG>(attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    widget(Widget::Radio, attrs, vec![])
}

/// create an image control
pub fn image<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Image, attrs, vec![])
}

/// create an image control from svg
pub fn svg<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Svg, attrs, vec![])
}

/// create a text area
pub fn textarea<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::TextArea, attrs, vec![])
}

/// create a text label
pub fn text_label<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::Label, attrs, vec![])
}

pub fn header_bar<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Auto
            },
        },
        min_size: Size {
            height: Dimension::Points(30.0),
            ..Default::default()
        },
        ..Default::default()
    }));
    widget(Widget::HeaderBar, attrs, children)
}

pub fn menu_bar<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Auto
            },
        },
        ..Default::default()
    }));
    widget(Widget::MenuBar, attrs, children)
}

pub fn menu<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Auto
            },
        },
        ..Default::default()
    }));
    widget(Widget::Menu, attrs, children)
}

pub fn menu_item<MSG>(
    mut attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Auto
            },
        },
        ..Default::default()
    }));
    widget(Widget::MenuItem, attrs, children)
}

/// create a text input
pub fn search_input<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        },
        ..Default::default()
    }));
    widget(Widget::SearchInput, attrs, vec![])
}

/// create a link button
pub fn link<MSG>(mut attrs: Vec<Attribute<MSG>>) -> Node<MSG>
where
    MSG: 'static,
{
    let spec_width = find_value(AttribKey::Width, &attrs)
        .map(|w| w.as_f64())
        .flatten();
    let spec_height = find_value(AttribKey::Height, &attrs)
        .map(|w| w.as_f64())
        .flatten();

    let spec_position = find_value(AttribKey::PositionType, &attrs)
        .map(|w| w.as_position_type())
        .flatten();

    attrs.push(style(Style {
        position_type: if let Some(spec_position) = spec_position {
            spec_position
        } else {
            Default::default()
        },
        size: Size {
            width: if let Some(width) = spec_width {
                Dimension::Points(width as f32)
            } else {
                Dimension::Percent(1.0)
            },
            height: if let Some(height) = spec_height {
                Dimension::Points(height as f32)
            } else {
                Dimension::Percent(1.0)
            },
        },
        /*
        min_size: Size {
            height: Dimension::Points(30.0),
            ..Default::default()
        },
        */
        ..Default::default()
    }));
    widget(Widget::Link, attrs, vec![])
}
