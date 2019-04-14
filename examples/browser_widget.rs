use browser::Node;
use caesar::widget::View;
use caesar::widget::Widget;

fn main() {
    println!("browser widget");
    let view = View::default();
    println!("{:?}", view);
    let widget: Widget = view.into();
    let node: Node = widget.into();
    println!("node: {}", node);
}
