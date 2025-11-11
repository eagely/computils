pub struct Point {
    r: isize,
    c: isize,
}

impl Point {
    pub fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }
}
