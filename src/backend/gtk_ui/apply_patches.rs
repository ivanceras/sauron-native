use crate::Patch;
use gtk::{prelude::*, ApplicationWindow, Container, ContainerExt};
use std::{fmt::Debug, rc::Rc};

pub fn apply_patches<MSG>(root_node: &Rc<ApplicationWindow>, patches: &Vec<Patch<MSG>>)
where
    MSG: Debug,
{
    println!("Applying patches: {:#?}", patches);
    let children = root_node.get_children();
    println!("There are {} children", children.len());
    let container: Option<&Container> = children[0].downcast_ref();
    println!("now a container: {:#?}", container);
    if let Some(container) = container {
        let grandchildren = container.get_children();
        println!("There are {} grandchildren", grandchildren.len());
    }
}
