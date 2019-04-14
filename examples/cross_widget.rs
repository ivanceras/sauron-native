use caesar::backend::tui::TuiWidget;
use caesar::widget::*;
use caesar::Node;

fn main() {
    let fields = row([], [view([], []), button([], "Hello")]);
    let node: Node = Node(fields);
    println!("node: {:#?}", node);
    let html: browser::Node = node.into();
    println!("html: {}", html);
}
