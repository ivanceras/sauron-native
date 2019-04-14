use caesar::Node;
use caesar::widget::Row;
use vdom::builder::element;
use caesar::Widget;

fn main(){
    let row = Row::default();
    let widget: Widget = row.into();
    let node: Node<Widget> = element(widget.clone(), [], []);
    println!("node: {:#?}", node);

    let browser_node: browser::Node = widget.clone().into();
    println!("browser_node: {:#?}", browser_node);

    let tui_widget: caesar::backend::tui::TuiWidget = widget.into();
}
