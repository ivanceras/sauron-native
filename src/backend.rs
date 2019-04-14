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
