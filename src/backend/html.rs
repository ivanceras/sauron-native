use crate::{
    util, widget::attribute::find_value, AttribKey, Attribute, Backend, Component, Widget,
};
use image::ImageFormat;
use sauron::{
    html::{attributes::*, div, events::mapper, img, input, text},
    prelude::*,
    Component as SauronComponent, DomUpdater, Program,
};
use sauron_vdom::Callback;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};
use wasm_bindgen::JsCast;

pub struct HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    app: APP,
    _phantom_data: PhantomData<MSG>,
}

pub struct HtmlBackend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    program: Program<HtmlApp<APP, MSG>, MSG>,
}

impl<APP, MSG> HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) -> Self {
        HtmlApp {
            app,
            _phantom_data: PhantomData,
        }
    }
}

impl<APP, MSG> sauron::Component<MSG> for HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn update(&mut self, msg: MSG) -> sauron::dom::cmd::Cmd<sauron::Program<Self, MSG>, MSG> {
        self.app.update(msg);
        sauron::dom::cmd::Cmd::none()
    }

    fn view(&self) -> sauron::Node<MSG> {
        let view = self.app.view();
        let html_view = widget_tree_to_html_node(view);
        html_view
    }
}

impl<APP, MSG> Backend<APP, MSG> for HtmlBackend<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) -> Self {
        console_log::init_with_level(log::Level::Trace);
        log::trace!("Html app started..");
        let html_app = HtmlApp::new(app);
        let program = sauron::Program::mount_to_body(html_app);
        HtmlBackend { program }
    }
}

/// convert Widget into an equivalent html node
fn widget_to_html<MSG>(widget: &Widget, attrs: Vec<Attribute<MSG>>) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget {
        Widget::Vbox => div(
            vec![styles(vec![
                ("display", "flex"),
                ("flex-direction", "column"),
            ])],
            vec![],
        ),
        Widget::Hbox => div(
            vec![styles(vec![("display", "flex"), ("flex-direction", "row")])],
            vec![],
        ),
        //TODO: vpane and hpane should be draggable
        Widget::Vpane => div(
            vec![styles(vec![
                ("display", "flex"),
                ("flex-direction", "column"),
            ])],
            vec![],
        ),
        Widget::Hpane => div(
            vec![styles(vec![("display", "flex"), ("flex-direction", "row")])],
            vec![],
        ),
        Widget::Button => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let attributes = attrs
                .into_iter()
                .filter_map(|att| match att.name {
                    AttribKey::ClickEvent => {
                        att.take_callback().map(|cb| onclick(move |ev| cb.emit(ev)))
                    }
                    _ => None,
                })
                .collect();
            input(vec![r#type("button"), value(label)], vec![]).add_attributes(attributes)
        }
        Widget::Paragraph => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            p(vec![], vec![text(txt_value)])
        }
        Widget::TextInput => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            let attributes = attrs
                .into_iter()
                .filter_map(|att| match att.name {
                    AttribKey::InputEvent => {
                        att.take_callback().map(|cb| oninput(move |ev| cb.emit(ev)))
                    }
                    _ => None,
                })
                .collect();
            input(vec![r#type("text"), value(txt_value)], vec![]).add_attributes(attributes)
        }
        Widget::TextArea => {
            let txt_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            let attributes = attrs
                .into_iter()
                .filter_map(|att| match att.name {
                    AttribKey::InputEvent => {
                        att.take_callback().map(|cb| oninput(move |ev| cb.emit(ev)))
                    }
                    _ => None,
                })
                .collect();
            textarea(vec![value(&txt_value)], vec![text(txt_value)]).add_attributes(attributes)
        }
        Widget::Checkbox => {
            let cb_label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            let cb_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(false);
            let checked = attrs_flag([("checked", "checked", cb_value)]);

            div(
                vec![],
                vec![
                    input(vec![type_("checkbox")], vec![]).add_attributes(checked),
                    label(vec![], vec![text(cb_label)]),
                ],
            )
        }
        Widget::Radio => {
            let cb_label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());
            let cb_value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(false);
            let checked = attrs_flag([("checked", "checked", cb_value)]);
            div(
                vec![],
                vec![
                    input(vec![type_("radio")], vec![]).add_attributes(checked),
                    label(vec![], vec![text(cb_label)]),
                ],
            )
        }
        Widget::Image => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);

            let mime_type = util::image_mime_type(bytes).expect("unsupported image");
            img(
                vec![
                    styles([
                        ("width", "100%"),
                        ("height", "auto"),
                        ("max-width", "800px"),
                    ]),
                    src(format!(
                        "data:{};base64,{}",
                        mime_type,
                        base64::encode(bytes)
                    )),
                ],
                vec![],
            )
        }
        Widget::Svg => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);
            img(
                vec![
                    styles([
                        ("width", "100%"),
                        ("height", "auto"),
                        ("max-width", "800px"),
                    ]),
                    src(format!(
                        "data:image/svg+xml;base64,{}",
                        base64::encode(bytes)
                    )),
                ],
                vec![],
            )
        }
        Widget::Scroll => div(vec![], vec![]),
    }
}

/// converts widget virtual node tree into an html node tree
pub fn widget_tree_to_html_node<MSG>(widget_node: crate::Node<MSG>) -> sauron::Node<MSG>
where
    MSG: Clone + Debug + 'static,
{
    match widget_node {
        crate::Node::Element(widget) => {
            // convert the Widget tag to html node
            let mut html_node: sauron::Node<MSG> = widget_to_html(&widget.tag, widget.attrs);
            // cast the html node to element
            if let Some(html_element) = html_node.as_element_mut() {
                for widget_child in widget.children {
                    // convert all widget child to an html child node
                    let mut html_child: sauron::Node<MSG> = widget_tree_to_html_node(widget_child);
                    html_element.children.push(html_child);
                }
            }
            html_node
        }
        crate::Node::Text(txt) => text(txt.text),
    }
}
