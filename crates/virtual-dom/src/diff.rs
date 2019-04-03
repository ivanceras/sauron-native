
use crate::Patch;
use crate::VNode;
use crate::Value;
use std::cmp::min;
use std::collections::HashMap;
use std::mem;

/// Given two VNode's generate Patch's that would turn the old virtual node's
/// real DOM node equivalent into the new VNode's real DOM node equivalent.
pub fn diff<'a>(old: &'a VNode, new: &'a VNode) -> Vec<Patch<'a>> {
    diff_recursive(&old, &new, &mut 0)
}

fn diff_recursive<'a, 'b>(
    old: &'a VNode,
    new: &'a VNode,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a>> {
    let mut patches = vec![];
    let mut replace = false;

    // Different enum variants, replace!
    if mem::discriminant(old) != mem::discriminant(new) {
        replace = true;
    }

    if let (VNode::Element(old_element), VNode::Element(new_element)) = (old, new) {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }

        // Replace if two elements have different keys
        // TODO: More robust key support. This is just an early stopgap to allow you to force replace
        // an element... say if it's event changed. Just change the key name for now.
        // In the future we want keys to be used to create a Patch::ReOrder to re-order siblings
        if old_element.attrs.get("key").is_some()
            && old_element.attrs.get("key") != new_element.attrs.get("key")
        {
            replace = true;
        }
    }

    // Handle replacing of a node
    if replace {
        patches.push(Patch::Replace(*cur_node_idx, &new));
        if let VNode::Element(old_element_node) = old {
            for child in old_element_node.children.iter() {
                increment_node_idx_for_children(child, cur_node_idx);
            }
        }
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old, new) {
        // We're comparing two text nodes
        (VNode::Text(old_text), VNode::Text(new_text)) => {
            if old_text != new_text {
                patches.push(Patch::ChangeText(*cur_node_idx, &new_text));
            }
        }

        // We're comparing two element nodes
        (VNode::Element(old_element), VNode::Element(new_element)) => {
            let mut add_attributes: HashMap<&str, &Value> = HashMap::new();
            let mut remove_attributes: Vec<&str> = vec![];

            // TODO: -> split out into func
            for (new_attr_name, new_attr_val) in new_element.attrs.iter() {
                match old_element.attrs.get(new_attr_name) {
                    Some(ref old_attr_val) => {
                        if old_attr_val != &new_attr_val {
                            add_attributes.insert(new_attr_name, new_attr_val);
                        }
                    }
                    None => {
                        add_attributes.insert(new_attr_name, new_attr_val);
                    }
                };
            }

            // TODO: -> split out into func
            for (old_attr_name, old_attr_val) in old_element.attrs.iter() {
                if add_attributes.get(&old_attr_name[..]).is_some() {
                    continue;
                };

                match new_element.attrs.get(old_attr_name) {
                    Some(ref new_attr_val) => {
                        if new_attr_val != &old_attr_val {
                            remove_attributes.push(old_attr_name);
                        }
                    }
                    None => {
                        remove_attributes.push(old_attr_name);
                    }
                };
            }

            if add_attributes.len() > 0 {
                patches.push(Patch::AddAttributes(*cur_node_idx, add_attributes));
            }
            if remove_attributes.len() > 0 {
                patches.push(Patch::RemoveAttributes(*cur_node_idx, remove_attributes));
            }

            let old_child_count = old_element.children.len();
            let new_child_count = new_element.children.len();

            if new_child_count > old_child_count {
                let append_patch: Vec<&'a VNode> =
                    new_element.children[old_child_count..].iter().collect();
                patches.push(Patch::AppendChildren(*cur_node_idx, append_patch))
            }

            if new_child_count < old_child_count {
                patches.push(Patch::TruncateChildren(*cur_node_idx, new_child_count))
            }

            let min_count = min(old_child_count, new_child_count);
            for index in 0..min_count {
                *cur_node_idx = *cur_node_idx + 1;
                let old_child = &old_element.children[index];
                let new_child = &new_element.children[index];
                patches.append(&mut diff_recursive(&old_child, &new_child, cur_node_idx))
            }
            if new_child_count < old_child_count {
                for child in old_element.children[min_count..].iter() {
                    increment_node_idx_for_children(child, cur_node_idx);
                }
            }
        }
        (VNode::Text(_), VNode::Element(_))
        | (VNode::Element(_), VNode::Text(_)) => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    //    new_root.create_element()
    patches
}

fn increment_node_idx_for_children<'a, 'b>(old: &'a VNode, cur_node_idx: &'b mut usize) {
    *cur_node_idx += 1;
    if let VNode::Element(element_node) = old {
        for child in element_node.children.iter() {
            increment_node_idx_for_children(&child, cur_node_idx);
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::*;
    use super::*;

    #[test]
    fn test_replace_node(){
        let old = VNode::Element(VElement {
            tag: "div".into(),
            attrs: HashMap::new(),
            events: HashMap::new(),
            children: vec![],
        });
        let new = VNode::Element(VElement {
            tag: "span".into(),
            attrs: HashMap::new(),
            events: HashMap::new(),
            children: vec![],
        });

        let diff = diff::diff(&old, &new);
        assert_eq!(diff, vec![Patch::Replace(0, &new)], "Should replace the first node");
    }

    #[test]
    fn test_simple_diff(){
        let old = VNode::Element(VElement {
            tag: "div".into(),
            attrs: {
                let mut hm: HashMap<String, Value> = HashMap::new();
                hm.insert("id".into(), "some-id".into());
                hm.insert("class".into(), "some-class".into());
                hm
            },
            events: HashMap::new(),
            children: vec![],
        });

        let new = VNode::Element(VElement {
            tag: "div".into(),
            attrs: {
                let mut hm: HashMap<String, Value> = HashMap::new();
                hm.insert("id".into(), "some-id".into());
                hm.insert("class".into(), "some-class".into());
                hm
            },
            events: HashMap::new(),
            children: vec![],
        });

        let diff = diff(&old, &new);
        assert_eq!(diff, vec![])
    }

    #[test]
    fn test_class_changed(){
        let old = VNode::Element(VElement {
            tag: "div".into(),
            attrs: {
                let mut hm: HashMap<String, Value> = HashMap::new();
                hm.insert("id".into(), "some-id".into());
                hm.insert("class".into(), "some-class".into());
                hm
            },
            events: HashMap::new(),
            children: vec![],
        });

        let new = VNode::Element(VElement {
            tag: "div".into(),
            attrs: {
                let mut hm: HashMap<String, Value> = HashMap::new();
                hm.insert("id".into(), "some-id".into());
                hm.insert("class".into(), "some-class2".into());
                hm
            },
            events: HashMap::new(),
            children: vec![],
        });

        let diff = diff(&old, &new);
        let class2 = Value::String("some-class2".to_string());
        assert_eq!(diff, vec![
                   Patch::AddAttributes(0, {
                       let mut hm = HashMap::new();
                       hm.insert("class", &class2);
                       hm
        })])
    }
}
