use super::convert_event;
use crate::widget::attribute::util::get_layout;
use crate::{util, widget::attribute::find_value, AttribKey, Widget};
use sauron::{
    html::{attributes::*, div, img, input, text},
    prelude::*,
};
use std::fmt::Debug;

/// converts widget virtual node tree into an html node tree
pub fn widget_tree_to_html_node<MSG>(
    widget_node: &crate::Node<MSG>,
    cur_node_idx: &mut usize,
) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget_node {
        crate::Node::Element(widget) => widget_to_html(widget, cur_node_idx),
        crate::Node::Text(txt) => {
            *cur_node_idx += 1;
            text(txt)
        }
    }
}

/// convert Widget into an equivalent html node
fn widget_to_html<MSG>(
    element: &crate::Element<MSG>,
    cur_node_idx: &mut usize,
) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    let attrs = element.get_attributes();

    let layout = get_layout(&element).expect("must have a layout");
    //log::debug!("tag: {:?} layout: {:#?}", element.tag(), layout);

    let mut html_children = vec![];
    for widget_child in element.get_children().iter() {
        *cur_node_idx += 1;
        // convert all widget child to an html child node
        let html_child: sauron::Node<MSG> =
            widget_tree_to_html_node(widget_child, cur_node_idx);
        html_children.push(html_child);
    }
    match element.tag() {
        Widget::Vbox => div(
            vec![
                class("Vbox"),
                styles(vec![("display", "flex"), ("flex-direction", "column")]),
                styles([
                    ("width", px(layout.size.width)),
                    ("height", px(layout.size.height)),
                ]),
            ],
            html_children,
        ),
        Widget::Hbox => div(
            vec![
                class("Hbox"),
                styles(vec![("display", "flex"), ("flex-direction", "row")]),
                styles([
                    ("width", px(layout.size.width)),
                    ("height", px(layout.size.height)),
                ]),
            ],
            html_children,
        ),
        //TODO: vpane and hpane should be draggable
        Widget::Vpane => div(
            vec![
                class("Vpane"),
                styles(vec![("display", "flex"), ("flex-direction", "column")]),
                styles([
                    ("width", px(layout.size.width)),
                    ("height", px(layout.size.height)),
                ]),
            ],
            html_children,
        ),
        // hpane will split the 2 children with 50-50 width
        // and a 100% height
        Widget::Hpane => div(
            vec![
                class("Hpane"),
                styles([("display", "flex"), ("flex-direction", "row")]),
                styles([
                    ("width", px(layout.size.width)),
                    ("height", px(layout.size.height)),
                ]),
            ],
            html_children,
        ),
        // the children in overlay will be all in absolute
        Widget::Overlay => {
            let children_len = html_children.len();
            html_children
                .iter_mut()
                .zip(element.get_children().iter())
                .enumerate()
                .for_each(|(_child_index, (html_child, widget_child))| {
                    let widget_child_element = widget_child
                        .as_element_ref()
                        .expect("must be an element");
                    let child_layout = get_layout(&widget_child_element)
                        .expect("must have a child layout");

                    log::error!(
                        "child attrs: {:#?}",
                        html_child.get_attributes()
                    );

                    let existing_style: Vec<&AttributeValue> = html_child
                        .get_attribute_value(&"style")
                        .unwrap_or(vec![]);

                    log::error!("existing styles: {:#?}", existing_style);

                    let mut styles: Vec<Style> = existing_style
                        .iter()
                        .flat_map(|s| s.as_style().cloned())
                        .flatten()
                        .collect();
                    log::error!("flatten styles: {:#?}", styles);

                    // Remove and override the position to absolute
                    if let Some(style_position) =
                        styles.iter_mut().find(|st| st.name == "position")
                    {
                        style_position.value = Value::from("absolute");
                    } else {
                        styles.push(Style::new("position", "absolute".into()));
                    }

                    // we override the child with
                    if let Some(style_width) =
                        styles.iter_mut().find(|st| st.name == "width")
                    {
                        style_width.value = Value::from(px(child_layout
                            .size
                            .width
                            * children_len as f32));
                    } else {
                        styles.push(Style::new(
                            "width",
                            px(child_layout.size.width * children_len as f32)
                                .into(),
                        ));
                    }

                    // we override the child height
                    if let Some(style_height) =
                        styles.iter_mut().find(|st| st.name == "height")
                    {
                        style_height.value =
                            Value::from(px(child_layout.size.height));
                    } else {
                        styles.push(Style::new(
                            "height",
                            px(child_layout.size.width as f32).into(),
                        ));
                    }

                    // the styles are reconstructed and net to be set again rather than add
                    html_child.set_attributes_ref_mut(vec![mt_dom::attr(
                        "style",
                        AttributeValue::from_styles(styles),
                    )]);
                });
            div(
                vec![
                    class("Overlay"),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                ],
                html_children,
            )
        }
        Widget::GroupBox => div(
            vec![
                class("GroupBox"),
                styles([
                    ("width", px(layout.size.width)),
                    ("height", px(layout.size.height)),
                ]),
            ],
            html_children,
        ),
        Widget::Label => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

            let is_preformatted = find_value(AttribKey::Preformatted, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);

            let is_monospace = find_value(AttribKey::Monospace, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);

            let is_selectable = find_value(AttribKey::Selectable, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(true);

            label(
                vec![
                    class("Label"),
                    styles_flag([
                        ("user-select", "none", !is_selectable),
                        ("font-family", "monospace", is_monospace),
                        ("white-space", "pre", is_preformatted),
                    ]),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                ],
                vec![text(value)],
            )
        }
        Widget::Button => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

            let svg_image_data = find_value(AttribKey::SvgImage, &attrs)
                .map(|v| v.as_bytes().map(|v| v.to_vec()))
                .flatten();

            let mut attributes = vec![];
            for att in attrs {
                match att.name() {
                    AttribKey::ClickEvent => {
                        for cb in att.get_callback() {
                            let cb = cb.clone();
                            attributes.push(on_click(move |ev| {
                                cb.emit(convert_event::from_mouse_event(ev))
                            }))
                        }
                    }
                    _ => (),
                }
            }

            button(
                vec![
                    class("Button"),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                ],
                vec![
                    text(label),
                    if let Some(svg_image_data) = svg_image_data {
                        img(
                            vec![src(format!(
                                "data:image/svg+xml;base64,{}",
                                base64::encode(&svg_image_data)
                            ))],
                            vec![],
                        )
                    } else {
                        text("")
                    },
                ],
            )
            .add_attributes(attributes)
        }
        Widget::Paragraph => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();
            p(
                vec![
                    class("Paragraph"),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                ],
                vec![text(txt_value)],
            )
        }
        Widget::TextInput => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();
            let mut attributes = vec![];
            for att in attrs {
                match att.name() {
                    AttribKey::InputEvent => {
                        for cb in att.get_callback() {
                            let cb = cb.clone();
                            attributes.push(on_input(move |ev| {
                                cb.emit(convert_event::to_input_event(ev))
                            }));
                        }
                    }
                    _ => (),
                }
            }
            input(
                vec![
                    class("TextInput"),
                    r#type("text"),
                    value(txt_value),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                ],
                vec![],
            )
            .add_attributes(attributes)
        }
        Widget::TextArea => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();
            let mut attributes = vec![];
            for att in attrs {
                match att.name() {
                    AttribKey::InputEvent => {
                        for cb in att.get_callback() {
                            let cb = cb.clone();
                            attributes.push(on_input(move |ev| {
                                cb.emit(convert_event::to_input_event(ev))
                            }));
                        }
                    }
                    _ => (),
                }
            }

            let is_preformatted = find_value(AttribKey::Preformatted, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);
            log::warn!("preformatted: {}", is_preformatted);

            let is_monospace = find_value(AttribKey::Monospace, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);

            let is_selectable = find_value(AttribKey::Selectable, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(true);
            let st: Attribute<MSG> = styles_flag([
                ("user-select", "none", !is_selectable),
                ("font-family", "monospace", is_monospace),
                ("white-space", "pre", is_preformatted),
            ]);

            log::warn!("st: {:#?}", st);

            textarea(
                vec![
                    class("TextArea"),
                    value(&txt_value),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                    styles_flag([
                        ("user-select", "none", !is_selectable),
                        ("font-family", "monospace", is_monospace),
                        ("white-space", "pre", is_preformatted),
                    ]),
                ],
                vec![text(txt_value)],
            )
            .add_attributes(attributes)
        }
        Widget::Checkbox => {
            let cb_label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();
            let cb_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);
            let checked = attrs_flag([("checked", "checked", cb_value)]);
            let widget_id = format!("checkbox_{}", cur_node_idx);

            div(
                vec![
                    class("Checkbox"),
                    /*styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ])*/
                ],
                vec![
                    input(vec![type_("checkbox"), id(&widget_id)], vec![])
                        .add_attributes(checked),
                    label(vec![for_(&widget_id)], vec![text(cb_label)]),
                ],
            )
        }
        Widget::Radio => {
            let cb_label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();
            let cb_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);
            let checked = attrs_flag([("checked", "checked", cb_value)]);
            let widget_id = format!("radio_{}", cur_node_idx);
            div(
                vec![
                    class("Radio"),
                    /*styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ])*/
                ],
                vec![
                    input(vec![type_("radio"), id(&widget_id)], vec![])
                        .add_attributes(checked),
                    label(vec![for_(&widget_id)], vec![text(cb_label)]),
                ],
            )
        }
        Widget::Image => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);

            let mime_type =
                util::image_mime_type(bytes).expect("unsupported image");
            div(
                vec![
                    class("Image"),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                    styles([("overflow", "auto")]),
                ],
                vec![img(
                    vec![src(format!(
                        "data:{};base64,{}",
                        mime_type,
                        base64::encode(bytes)
                    ))],
                    vec![],
                )],
            )
        }
        Widget::Svg => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);
            div(
                vec![
                    class("Svg"),
                    styles([
                        ("width", px(layout.size.width)),
                        ("height", px(layout.size.height)),
                    ]),
                    styles([("overflow", "auto")]),
                ],
                vec![img(
                    vec![src(format!(
                        "data:image/svg+xml;base64,{}",
                        base64::encode(bytes)
                    ))],
                    vec![],
                )],
            )
        }
    }
}
