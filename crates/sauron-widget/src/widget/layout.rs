use crate::widget::attribute::util::get_style;
use crate::widget::attribute::AttribKey;
use crate::widget::attribute::Value;
use expanse::geometry::Size;
use expanse::number::Number;
use expanse::Stretch;
use mt_dom::attr;

/// calculate the layout of the nodes utilizing the styles set on each of the widget
/// and its children widget styles
#[allow(unused)]
pub fn compute_node_layout<MSG>(
    widget_node: &mut crate::Node<MSG>,
    parent_size: Size<Number>,
) {
    let mut stretch = Stretch::new();
    let stretch_node = build_stretch_node_recursive(&mut stretch, &widget_node)
        .expect("must have built a style node");
    stretch
        .compute_layout(stretch_node, parent_size)
        .expect("must compute the layout");
    set_node_layout_from_stretch_node(widget_node, stretch_node, &stretch)
}

fn build_stretch_node_recursive<MSG>(
    stretch: &mut Stretch,
    widget_node: &crate::Node<MSG>,
) -> Option<expanse::node::Node> {
    let children_styles = if let Some(children) = widget_node.get_children() {
        children
            .iter()
            .filter_map(|c| build_stretch_node_recursive(stretch, c))
            .collect()
    } else {
        vec![]
    };
    let node_style = get_style(widget_node).cloned().unwrap_or_default();
    stretch.new_node(node_style, &children_styles).ok()
}

fn set_node_layout_from_stretch_node<MSG>(
    widget_node: &mut crate::Node<MSG>,
    stretch_node: expanse::node::Node,
    stretch: &Stretch,
) {
    let layout = *stretch.layout(stretch_node).expect("must have layout");
    let stretch_node_children: Vec<expanse::node::Node> =
        stretch.children(stretch_node).expect("must get children");

    let widget_children = widget_node.children_mut().unwrap_or(&mut []);

    stretch_node_children
        .into_iter()
        .zip(widget_children.iter_mut())
        .for_each(|(stretch_node_child, widget_child)| {
            set_node_layout_from_stretch_node(
                widget_child,
                stretch_node_child,
                stretch,
            )
        });

    widget_node.add_attributes_ref_mut(vec![attr(
        AttribKey::Layout,
        Value::from(layout),
    )]);
}
