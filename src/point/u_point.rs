pub struct UPoint {
    pub r: usize,
    pub c: usize,
}

impl UPoint {
    pub fn new(r: usize, c: usize) -> Self {
        Self { r, c }
    }
}

