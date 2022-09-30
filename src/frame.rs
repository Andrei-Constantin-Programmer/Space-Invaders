use crate::{COLUMNS, ROWS};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    let mut columns = Vec::with_capacity(COLUMNS);

    for _ in 0..COLUMNS {
        let mut column = Vec::with_capacity(ROWS);
        for _ in 0..ROWS {
            column.push(" ");
        }

        columns.push(column);
    }

    columns
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}