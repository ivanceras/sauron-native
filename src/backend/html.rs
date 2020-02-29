use crate::{Attribute, Backend, Component, Widget};
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
    program: Rc<Program<HtmlApp<APP, MSG>, MSG>>,
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
    fn update(&mut self, msg: MSG) -> sauron_vdom::Cmd<sauron::Program<Self, MSG>, MSG> {
        self.app.update(msg);
        sauron_vdom::Cmd::none()
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
    fn init(app: APP) -> Rc<Self> {
        console_log::init_with_level(log::Level::Trace);
        log::trace!("Html app started..");
        let html_app = HtmlApp::new(app);
        let program = sauron::Program::mount_to_body(html_app);
        let backend = HtmlBackend { program };
        Rc::new(backend)
    }
}

/// convert Widget into an equivalent html node
fn widget_to_html<MSG>(widget: &Widget, attrs: &Vec<Attribute<MSG>>) -> sauron::Node<MSG>
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
        Widget::Button => {
            /*
            let txt: String = if let Some(attr) = attrs.iter().find(|attr| attr.name == "value") {
                if let Some(value) = attr.get_value() {
                    value.to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };
            */
            let ce = if let Some(ce) = attrs.iter().find(|att| att.name == "click") {
                vec![ce.clone().reform(map_to_event)]
            } else {
                vec![]
            };
            input(vec![r#type("button")], vec![]).add_attributes(ce)
        }
        Widget::Text(txt) => text(&txt),
        Widget::TextInput(txt) => {
            let ie = if let Some(ie) = attrs.iter().find(|att| att.name == "input") {
                vec![ie.clone().reform(map_to_event)]
            } else {
                vec![]
            };
            input(vec![r#type("text"), value(txt)], vec![]).add_attributes(ie)
        }
        Widget::Block(title) => div(vec![], vec![text(title)]),
        Widget::Checkbox(value) => {
            let checked = attrs_flag([("checked", "checked", *value)]);
            input(vec![type_("checkbox")], vec![]).add_attributes(checked)
        }
        Widget::Image(image) => {
            use image::GenericImageView;
            let mime_type = "image/jpeg";
            let format = image::guess_format(&image).ok();
            let image_mime = if let Some(format) = format {
                format!("{:?}", format)
            } else {
                "cant_guess".to_string()
            };
            //let dyn_image = image::load_from_memory(&image);
            //let width = dyn_image.ok().map(|im| im.width());
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
                        base64::encode(image)
                    )),
                    attr("format", image_mime),
                ],
                vec![],
            )
        }
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
            let mut html_node: sauron::Node<MSG> = widget_to_html(&widget.tag, &widget.attrs);
            // cast the html node to element
            if let Some(html_element) = html_node.as_element_mut() {
                for attr in widget.attributes() {
                    html_element.attrs.push(attr.reform(map_to_event));
                }
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

/// convert html event into sauron_vdom event
fn map_to_event(event: sauron::Event) -> sauron_vdom::Event {
    //sauron_vdom::Event::KeyEvent(sauron_vdom::event::KeyEvent::new("k".to_string()))
    if let Some(_mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
        let me = mapper::mouse_event_mapper(event);
        sauron_vdom::Event::MouseEvent(me)
    } else if let Some(_input_event) = event.dyn_ref::<web_sys::InputEvent>() {
        let input_event = mapper::input_event_mapper(event);
        sauron_vdom::Event::InputEvent(input_event)
    } else if let Some(_key_event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
        let key_event = mapper::keyboard_event_mapper(event);
        sauron_vdom::Event::KeyEvent(key_event)
    } else {
        panic!("unsupported event!")
    }
}
