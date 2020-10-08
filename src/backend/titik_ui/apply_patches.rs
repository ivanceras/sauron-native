use super::convert_widget;
use crate::{AttribKey, Attribute, Patch, Widget};
use mt_dom::patch::{AddAttributes, AppendChildren, RemoveNode};
use std::fmt::Debug;
use titik::{Button, TextArea};

pub fn apply_patches<MSG, DSP>(
    _program: &DSP,
    root_node: &mut dyn titik::Widget<MSG>,
    patches: &[Patch<MSG>],
) where
    MSG: Debug + 'static,
{
    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget: &mut dyn titik::Widget<MSG> =
            titik::find_widget_mut(root_node, patch_node_idx)
                .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(AddAttributes {
                tag,
                node_idx: _,
                new_node_idx,
                attrs,
            }) => {
                eprintln!("setting attributes...");
                set_widget_attributes::<MSG>(tag, widget, attrs);
            }
            Patch::AppendChildren(AppendChildren {
                tag,
                node_idx: _,
                children,
            }) => {
                eprintln!("adding children..");
                for (child_idx, child) in children {
                    let child_element =
                        child.as_element_ref().expect("must be an element");
                    let child_widget = convert_widget::from_node(
                        &child_element.tag,
                        &child_element.attrs,
                    );
                    eprintln!("added 1 {:?} to {:?}", child_widget, widget);
                    let added = widget.add_child(child_widget);
                    eprintln!("widget becomes: {:?}", widget);
                    assert!(added);
                }
            }
            Patch::RemoveNode(RemoveNode { tag: _, node_idx }) => {
                titik::remove_widget(root_node, *node_idx);
            }
            // todo for other patches here.
            _ => eprintln!("todo for: {:?}", patch),
        }
    }
}

fn set_widget_attributes<MSG: 'static>(
    tag: &crate::Widget,
    widget: &mut dyn titik::Widget<MSG>,
    attrs: &[&Attribute<MSG>],
) {
    match tag {
        Widget::TextArea => {
            let text_area: &mut TextArea<MSG> = widget
                .as_any_mut()
                .downcast_mut()
                .expect("must be a textarea");
            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Value => {
                            text_area.set_value(&value.to_string());
                        }
                        _ => (),
                    }
                }
            }
        }
        Widget::Button => {
            let btn: &mut Button<MSG> = widget
                .as_any_mut()
                .downcast_mut()
                .expect("must be a button");

            for att in attrs {
                for value in att.get_plain() {
                    match att.name() {
                        AttribKey::Label => {
                            btn.set_label(&value.to_string());
                        }
                        _ => (),
                    }
                }
            }
        }
        _ => eprintln!("todo for other widgets.. {:?}", tag),
    }
}
