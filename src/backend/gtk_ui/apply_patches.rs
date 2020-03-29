use super::{Dispatch, GtkBackend};
use crate::{AttribKey, Patch};
use gtk::{prelude::*, Button, Container, ContainerExt, TextView, Widget};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

pub fn apply_patches<MSG, DSP>(program: &Rc<DSP>, container: &Container, patches: &Vec<Patch<MSG>>)
where
    MSG: Debug,
    DSP: Dispatch<MSG> + 'static,
{
    println!("patches: {:#?}", patches);
    let nodes_to_patch = find_nodes(container, patches);
    println!("nodes to patch: {:?}", nodes_to_patch);

    for patch in patches {
        let patch_node_idx = patch.node_idx();
        println!("trying to get patch_node_idx: {}", patch_node_idx);
        let widget = nodes_to_patch
            .get(&patch_node_idx)
            .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(_node_idx, attrs) => {
                for att in attrs {
                    println!("att: {:?}", att);
                    //TODO: actuall set the property of the widget
                    match att.name {
                        AttribKey::Label => {
                            if let Some(button) = widget.downcast_ref::<Button>() {
                                if let Some(value) = att.get_value() {
                                    button.set_label(&value.to_string());
                                }
                            }
                        }
                        AttribKey::Value => {
                            if let Some(text_view) = widget.downcast_ref::<TextView>() {
                                if let Some(value) = att.get_value() {
                                    if let Some(buffer) = text_view.get_buffer() {
                                        buffer.set_text(&value.to_string());
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            Patch::AppendChildren(_node_idx, nodes) => {
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
            Patch::TruncateChildren(_node_idx, num_children_remaining) => {
                println!("Truncating children {}", num_children_remaining);
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

fn find_nodes<MSG>(root_node: &Container, patches: &[Patch<MSG>]) -> HashMap<usize, Widget> {
    let mut nodes_to_find = HashSet::new();
    let mut cur_node_idx = 0;

    for patch in patches {
        nodes_to_find.insert(patch.node_idx());
    }
    find_nodes_recursive(root_node, &mut cur_node_idx, &nodes_to_find)
}

fn find_nodes_recursive(
    root_node: &Container,
    cur_node_idx: &mut usize,
    nodes_to_find: &HashSet<usize>,
) -> HashMap<usize, Widget> {
    println!("nodes to find: {:?}", nodes_to_find);
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();
    println!("cur_node_idx: {} {:?}", cur_node_idx, root_node);
    // prevent other container other than gtk::Box to be traverse otherwise widget such as textarea or textinput will
    // be traverse
    if root_node.downcast_ref::<gtk::Box>().is_some() {
        let children = root_node.get_children();
        let child_node_count = children.len();
        println!("children node count: {}", child_node_count);

        for child in children {
            println!("this child is a {:?}", child);
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
