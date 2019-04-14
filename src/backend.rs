pub mod html {
    use crate::layout::Direction;
    use crate::widget::Widget;
    use browser::html::attributes::*;
    use browser::html::*;
    use browser::Attribute;
    use browser::Node;

    impl Into<Node> for Widget {
        fn into(self) -> Node {
            match self {
                Widget::View(view) => {
                    let direction: Attribute = view.direction.into();
                    div([display("flexbox")], []).attributes([direction])
                }
                Widget::Row(row) => {
                    let direction: Attribute = row.direction.into();
                    div([display("flexbox")], []).attributes([direction])
                }
            }
        }
    }

    impl<'a> Into<Attribute<'a>> for Direction {
        fn into(self) -> Attribute<'a> {
            match self {
                Direction::Horizontal => flexDirection("row"),
                Direction::Vertical => flexDirection("column"),
            }
        }
    }
}

pub mod tui {

    use crate::Widget;
    use tui::widgets::Block;

    pub struct TuiWidget(Box<tui::widgets::Widget>);

    impl Into<TuiWidget> for crate::Widget{

        fn into(self) -> TuiWidget {
            match self {
                Widget::View(_view) =>  {
                    TuiWidget(Box::new(Block::default()))
                }
                Widget::Row(_row) => {
                    TuiWidget(Box::new(Block::default()))
                }
            }
        }
    }

}
