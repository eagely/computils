use ndarray::Array2;

pub struct Static2DGrid<T> {
    pub data: Array2<Option<T>>,
}

impl<T> Static2DGrid<T> {
    pub fn new(rs: usize, cs: usize) -> Self {
        Self {
            data: Array2::default((rs, cs)),
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        self.data.get((r, c))?.as_ref()
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T> {
        self.data.get_mut((r, c))?.as_mut()
    }

    pub fn set(&mut self, r: usize, c: usize, v: T) -> Option<T> {
        let cell = self.data.get_mut((r, c))?;
        cell.replace(v)
    }

    pub fn in_bounds(&self, r: usize, c: usize) -> bool {
        r < self.data.shape()[0] && c < self.data.shape()[1]
    }
}
