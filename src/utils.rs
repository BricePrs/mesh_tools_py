
#[derive(Copy, Clone)]
pub enum Direction {
    Positive,
    Null,
    Negative,
}

pub(crate) type Axis2d = [Direction; 2];


