use crate::{AttribKey, Patch};
use gtk::{prelude::*, Button, Container, ContainerExt, Widget};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

pub fn apply_patches<MSG>(container: &Container, patches: &Vec<Patch<MSG>>)
where
    MSG: Debug,
{
    let nodes_to_patch = find_nodes(container, patches);

    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget = nodes_to_patch
            .get(&patch_node_idx)
            .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(_node_idx, attrs) => {
                for att in attrs {
                    println!("att: {:?}", att);
                    //TODO: actuall set the property of the widget
                    if att.name == AttribKey::Label {
                        if let Some(button) = widget.downcast_ref::<Button>() {
                            if let Some(value) = att.get_value() {
                                button.set_label(&value.to_string());
                            }
                        }
                    }
                }
            }
            Patch::AppendChildren(_node_idx, nodes) => {
                if let Some(container) = widget.downcast_ref::<Container>() {
                    //TODO: instantiate the actual node, instead of just buttons
                    for node in nodes {
                        let btn = Button::new_with_label("btn here..");
                        container.add(&btn);
                        btn.show();
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
    let mut nodes_to_patch: HashMap<usize, Widget> = HashMap::new();

    if nodes_to_find.get(&cur_node_idx).is_some() {
        let root_widget: Widget = root_node.clone().upcast();
        nodes_to_patch.insert(*cur_node_idx, root_widget);
    }

    let children = root_node.get_children();
    let child_node_count = children.len();

    *cur_node_idx += 1;

    for i in 0..child_node_count {
        let child_node = children[i].clone();
        if let Some(container) = child_node.downcast_ref::<Container>() {
            let child_nodes_to_patch = find_nodes_recursive(container, cur_node_idx, nodes_to_find);
            nodes_to_patch.extend(child_nodes_to_patch);
        }
    }
    nodes_to_patch
}
