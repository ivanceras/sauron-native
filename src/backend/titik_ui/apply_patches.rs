use crate::AttribKey;
use crate::Attribute;
use crate::Patch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use titik::TextArea;
use titik::Widget as Control;

pub fn apply_patches<MSG, DSP>(
    program: &DSP,
    root_node: &mut Box<dyn Control<MSG>>,
    patches: &[Patch<MSG>],
) where
    MSG: Debug,
{
    for patch in patches {
        let patch_node_idx = patch.node_idx();
        let widget: &mut dyn Control<MSG> =
            titik::find_widget_mut(root_node.as_mut(), patch_node_idx)
                .expect("must have a node to patch");
        match patch {
            Patch::AddAttributes(tag, _node_idx, attrs) => {
                eprintln!("setting attributes...");
                set_widget_attributes::<MSG>(tag, widget, attrs);
            }
            _ => println!("todo for: {:?}", patch),
        }
    }
}

fn set_widget_attributes<MSG: 'static>(
    tag: &crate::Widget,
    widget: &mut dyn Control<MSG>,
    attrs: &[Attribute<MSG>],
) {
    if let Some(text_area) = widget.as_any_mut().downcast_mut::<TextArea<MSG>>() {
        eprintln!("this is a textarea");
        for att in attrs {
            if let Some(value) = att.get_value() {
                match att.name {
                    AttribKey::Value => {
                        text_area.set_value(&value.to_string());
                    }
                    _ => (),
                }
            }
        }
    }
}
