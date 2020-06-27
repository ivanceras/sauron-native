use super::Dispatch;
use crate::{widget::attribute::util::is_scrollable, AttribKey, Attribute, Node, Patch};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gtk::{
    prelude::*, Button, Container, ContainerExt, EventBox, Image, Label, Overlay, TextView, Widget,
};
use std::{collections::HashMap, fmt::Debug};

pub fn apply_patches<MSG, DSP>(
    program: &DSP,
    node: &Node<MSG>,
    root_container: &Container,
    patches: &Vec<Patch<MSG>>,
) where
    MSG: Debug,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let nodes_to_patch = find_nodes(node, root_container, patches);

    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget = nodes_to_patch
            .get(&patch_node_idx)
            .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(tag, _node_idx, attrs) => {
                set_widget_attributes::<MSG>(tag, widget, attrs);
            }
            Patch::AppendChildren(tag, _node_idx, nodes) => {
                match tag {
                    crate::Widget::Overlay => {
                        eprintln!("appending children..");
                        let overlay = widget
                            .downcast_ref::<Overlay>()
                            .expect("must be an overlay");
                        for node in nodes {
                            if let Some(element) = node.as_element_ref() {
                                let child = super::from_node(program, &element.tag, &element.attrs);
                                let widget = child.as_widget().expect("must be a widget");
                                //Note: overlay have different behavior when adding child widget
                                overlay.add_overlay(widget);
                                overlay.set_child_index(widget, -1);
                                widget.show();
                                overlay.show_all();
                            }
                        }
                    }

                    _ => {
                        let container = widget
                            .downcast_ref::<Container>()
                            .expect("must be a container");
                        for node in nodes {
                            if let Some(element) = node.as_element_ref() {
                                let child = super::from_node(program, &element.tag, &element.attrs);
                                let widget = child.as_widget().expect("must be a widget");
                                println!("appending children: {:?}", widget);
                                //Note: overlay have different behavior when adding child widget
                                container.add(widget);
                                widget.show();
                            }
                        }
                    }
                }
            }
            Patch::TruncateChildren(_tag, _node_idx, num_children_remaining) => {
                println!("truncating children..");
                if let Some(container) = widget.downcast_ref::<Container>() {
                    let children = container.get_children();
                    for i in *num_children_remaining..children.len() {
                        println!("truncating children: {:?}", children[i]);
                        container.remove(&children[i]);
                    }
                }
            }
            Patch::Replace(_tag, _node_idx, new_node) => {
                println!("replacing...");
                root_container.remove(widget);
                if let Some(new_element) = new_node.as_element_ref() {
                    let new_widget =
                        super::from_node(program, &new_element.tag, &new_node.get_attributes());
                    let new_widget = new_widget.as_widget().expect("must be a widget");
                    root_container.add(new_widget);
                    new_widget.show();
                }
            }
            _ => {
                println!("container: {:?}", root_container);
                println!("todo for: {:?}", patch);
            }
        }
    }
}

fn set_widget_attributes<MSG: 'static>(
    tag: &crate::Widget,
    widget: &Widget,
    attrs: &Vec<Attribute<MSG>>,
) {
    match tag {
        crate::Widget::Button => {
            let button = widget.downcast_ref::<Button>().expect("must be a button");
            for att in attrs {
                if let Some(value) = att.get_value() {
                    match att.name {
                        AttribKey::Label => button.set_label(&value.to_string()),
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::TextArea => {
            let text_view = widget
                .downcast_ref::<TextView>()
                .unwrap_or_else(|| panic!("must be a text_view, found: {:?}", widget));
            for att in attrs {
                if let Some(value) = att.get_value() {
                    match att.name {
                        AttribKey::Value => {
                            if let Some(buffer) = text_view.get_buffer() {
                                buffer.set_text(&value.to_string());
                            }
                        }
                        AttribKey::Editable => {
                            let editable = value.as_bool().unwrap_or(false);
                            text_view.set_editable(editable);
                        }
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::Svg => {
            let image = widget
                .downcast_ref::<Image>()
                .unwrap_or_else(|| panic!("must be an image {:?}", widget));
            for att in attrs {
                if let Some(value) = att.get_value() {
                    match att.name {
                        AttribKey::Data => {
                            if let Some(bytes) = value.as_bytes() {
                                let pixbuf_loader =
                                    PixbufLoader::new_with_mime_type("image/svg+xml")
                                        .expect("error loader");
                                pixbuf_loader
                                    .write(bytes)
                                    .expect("Unable to write svg data into pixbuf_loader");
                                pixbuf_loader.close().expect("error creating pixbuf");
                                let pixbuf = pixbuf_loader.get_pixbuf();
                                image.set_from_pixbuf(Some(
                                    &pixbuf.expect("error in pixbuf_loader"),
                                ));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        crate::Widget::Label => {
            let event_box = widget
                .downcast_ref::<EventBox>()
                .unwrap_or_else(|| panic!("must be an eventbox, found: {:?}", widget));
            let event_box_children = event_box.get_children();
            let child1 = event_box_children.get(0).expect("must have one child");
            let label = child1
                .downcast_ref::<Label>()
                .unwrap_or_else(|| panic!("must be a label, found: {:?}", widget));
            for att in attrs {
                if let Some(value) = att.get_value() {
                    match att.name {
                        AttribKey::Value => label.set_text(&value.to_string()),
                        _ => (),
                    }
                }
            }
        }
        _ => {
            println!("todo for other widgets");
        }
    }
}

fn find_nodes<MSG>(
    node: &Node<MSG>,
    container: &Container,
    patches: &[Patch<MSG>],
) -> HashMap<usize, Widget> {
    let mut nodes_to_find: HashMap<usize, &crate::Widget> = HashMap::new();
    let mut cur_node_idx = 0;

    for patch in patches {
        let tag = patch.tag().expect("must have a tag");
        nodes_to_find.insert(patch.node_idx(), tag);
    }
    find_nodes_recursive(node, container, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive<MSG>(
    node: &Node<MSG>,
    container: &Container,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashMap<usize, &crate::Widget>,
) -> HashMap<usize, Widget> {
    let tag = node.tag().expect("must have a tag");
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();
    println!("tag: {:?}", tag);
    match tag {
        crate::Widget::Hpane
        | crate::Widget::Vbox
        | crate::Widget::Hbox
        | crate::Widget::GroupBox
        | crate::Widget::Overlay => {
            let node_children = node.get_children().expect("must have children");

            //GroupBox(Frame(Box))
            let widget_children = if *tag == crate::Widget::GroupBox {
                let frame_children = container.get_children();
                let gbox_widget = frame_children.get(0).expect("must have one child");
                let container = gbox_widget
                    .downcast_ref::<Container>()
                    .expect("must be a container");
                container.get_children()
            } else {
                container.get_children()
            };

            assert_eq!(node_children.len(), widget_children.len());
            for (child_node, widget_child) in node_children.iter().zip(widget_children.iter()) {
                *cur_node_idx += 1;
                let child_tag = child_node.tag().expect("must have a child tag");
                let attrs = child_node.get_attributes();
                if let Some(_patch_tag) = nodes_to_find.get(&cur_node_idx) {
                    println!("got some: {:?}", tag);
                    match child_tag {
                        crate::Widget::TextArea => {
                            if is_scrollable(&attrs) {
                                // ScrolledWindow -> TextArea
                                let scrolled_window = widget_child
                                    .downcast_ref::<Container>()
                                    .expect("must be a scrolled window container");
                                let scrolled_window_children = scrolled_window.get_children();
                                let text_area =
                                    scrolled_window_children.get(0).expect("must have a child");
                                let text_area: Widget = text_area.clone().upcast();
                                nodes_to_patch.insert(*cur_node_idx, text_area);
                            } else {
                                let text_area: Widget = widget_child.clone().upcast();
                                nodes_to_patch.insert(*cur_node_idx, text_area);
                            }
                        }
                        crate::Widget::Svg => {
                            if is_scrollable(&attrs) {
                                // ScrolledWindow -> ViewPort -> Image
                                let scrolled_window = widget_child
                                    .downcast_ref::<Container>()
                                    .expect("must be a scrolled window container");

                                let scrolled_window_children = scrolled_window.get_children();
                                let view_port = scrolled_window_children
                                    .get(0)
                                    .expect("scrolled window must have a child");
                                let view_port = view_port
                                    .downcast_ref::<Container>()
                                    .expect("must be a viewport container");

                                let view_port_children = view_port.get_children();
                                let svg_image = view_port_children
                                    .get(0)
                                    .expect("view port must have svg image as child");
                                let svg_image: Widget = svg_image.clone().upcast();
                                nodes_to_patch.insert(*cur_node_idx, svg_image);
                            } else {
                                let svg_image: Widget = widget_child.clone().upcast();
                                nodes_to_patch.insert(*cur_node_idx, svg_image);
                            }
                        }
                        _ => {
                            let widget: Widget = widget_child.clone().upcast();
                            nodes_to_patch.insert(*cur_node_idx, widget);
                        }
                    }
                }
                println!("child tag: {:?}", child_tag);
                match child_tag {
                    crate::Widget::TextArea | crate::Widget::Svg => {
                        println!("skipping leaf widgets that are containers..");
                    }
                    _ => {
                        if let Some(container) = widget_child.downcast_ref::<Container>() {
                            let child_nodes_to_patch = find_nodes_recursive(
                                child_node,
                                container,
                                cur_node_idx,
                                nodes_to_find,
                            );
                            nodes_to_patch.extend(child_nodes_to_patch);
                        }
                    }
                }
            }
        }
        _ => println!("todo for: {:?}", tag),
    }
    nodes_to_patch
}
