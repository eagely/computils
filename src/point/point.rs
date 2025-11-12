#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    pub r: isize,
    pub c: isize,
}

impl Point {
    pub fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }
}
