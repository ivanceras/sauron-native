use crate::layout::Direction;

#[derive(Debug,Clone)]
pub struct View {
    pub direction: Direction,
}

impl Default for View {
    fn default() -> Self {
        View {
            direction: Direction::Vertical,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Row {
    pub direction: Direction,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            direction: Direction::Vertical,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Widget {
    View(View),
    Row(Row),
}

impl From<View> for Widget {
    fn from(view: View) -> Self {
        Widget::View(view)
    }
}

impl From<Row> for Widget {
    fn from(row: Row) -> Self {
        Widget::Row(row)
    }
}
