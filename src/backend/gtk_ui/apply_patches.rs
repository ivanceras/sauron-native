use super::{Dispatch, GtkBackend};
use crate::{AttribKey, Attribute, Patch};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gtk::{
    prelude::*, Button, Container, ContainerExt, Image, ScrolledWindow, TextView, Viewport, Widget,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

pub fn apply_patches<MSG, DSP>(program: &DSP, container: &Container, patches: &Vec<Patch<MSG>>)
where
    MSG: Debug,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let nodes_to_patch = find_nodes(container, patches);

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
                if let Some(container) = widget.downcast_ref::<Container>() {
                    for node in nodes {
                        if let Some(element) = node.as_element_ref() {
                            let child =
                                super::from_node(program, &element.tag, &node.get_attributes());
                            let widget = child.as_widget().expect("must be a widget");
                            container.add(widget);
                            widget.show();
                        }
                    }
                }
            }
            Patch::TruncateChildren(tag, _node_idx, num_children_remaining) => {
                if let Some(container) = widget.downcast_ref::<Container>() {
                    let children = container.get_children();
                    for i in *num_children_remaining..children.len() {
                        container.remove(&children[i]);
                    }
                }
            }
            _ => {}
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
        _ => {
            println!("todo for other widgets");
        }
    }
}

fn find_nodes<MSG>(node: &Container, patches: &[Patch<MSG>]) -> HashMap<usize, Widget> {
    let mut nodes_to_find = HashSet::new();
    let mut cur_node_idx = 0;

    for patch in patches {
        nodes_to_find.insert(patch.node_idx());
    }
    find_nodes_recursive(node, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive(
    node: &Container,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashSet<usize>,
) -> HashMap<usize, Widget> {
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();

    let is_gbox = node.downcast_ref::<gtk::Box>().is_some();
    let is_overlay = node.downcast_ref::<gtk::Overlay>().is_some();
    let is_paned = node.downcast_ref::<gtk::Paned>().is_some();
    // Note: ScrolledWindow has a viewport
    let is_scrolled_window = node.downcast_ref::<gtk::ScrolledWindow>().is_some();
    // prevent other container other than gtk::Box to be traverse otherwise widget such as textarea or textinput will
    // be traverse
    if is_scrolled_window {
        let root_node_children = node.get_children();
        assert_eq!(root_node_children.len(), 1);
        let view_port_child = root_node_children
            .get(0)
            .expect("scroll window must have 1 child");
        let is_view_port = view_port_child.downcast_ref::<gtk::Viewport>().is_some();
        //NOTE: if the child widget is a text area, there is no viewport
        //if the child widget is an image, then there is viewport.. geez.
        let view_port = root_node_children
            .get(0)
            .expect("must have 1 children")
            .downcast_ref::<Container>()
            .expect("must be a container");
        let children = if is_view_port {
            view_port.get_children()
        } else {
            root_node_children
        };

        for child in children {
            *cur_node_idx += 1;
            if nodes_to_find.get(&cur_node_idx).is_some() {
                let widget: Widget = child.clone().upcast();
                nodes_to_patch.insert(*cur_node_idx, widget);
            }
            if let Some(container) = child.downcast_ref::<Container>() {
                let child_nodes_to_patch =
                    find_nodes_recursive(container, cur_node_idx, nodes_to_find);
                nodes_to_patch.extend(child_nodes_to_patch);
            }
        }
    }
    if is_gbox || is_paned || is_overlay {
        let children = node.get_children();
        let child_node_count = children.len();

        for child in children {
            *cur_node_idx += 1;
            if nodes_to_find.get(&cur_node_idx).is_some() {
                let widget: Widget = child.clone().upcast();
                nodes_to_patch.insert(*cur_node_idx, widget);
            }
            if let Some(container) = child.downcast_ref::<Container>() {
                let child_nodes_to_patch =
                    find_nodes_recursive(container, cur_node_idx, nodes_to_find);
                nodes_to_patch.extend(child_nodes_to_patch);
            }
        }
    }
    nodes_to_patch
}
