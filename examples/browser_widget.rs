use caesar::widgets::View;
use caesar::widgets::Widget;
use browser::Node;

fn main() {
    println!("browser widget");
    let view = View::default();
    println!("{:?}", view);
    let widget: Widget = view.into();
    let node: Node = widget.into();
    println!("node: {}", node);
}
