use crate::point::static_point::UPoint;
use ndarray::{
    Array2, Dim,
    iter::{IntoIter, Iter, IterMut},
};

pub struct Static2DGrid<T> {
    pub data: Array2<Option<T>>,
}

impl<T> Static2DGrid<T> {
    pub fn new(rs: usize, cs: usize) -> Self {
        Self {
            data: Array2::default((rs, cs)),
        }
    }

    pub fn rows(&self) -> usize {
        self.data.shape()[0]
    }

    pub fn columns(&self) -> usize {
        self.data.shape()[1]
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

    /// Check if the grid has 0 rows and 0 columns
    pub fn is_empty(&self) -> bool {
        self.rows() == 0 && self.columns() == 0
    }

    /// Check if the grid contains only None values
    pub fn is_blank(&self) -> bool {
        self.data.iter().all(|opt| opt.is_none())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<T>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> {
        self.data.iter_mut()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (usize, usize, &Option<T>)> {
        self.data.indexed_iter().map(|((r, c), v)| (r, c, v))
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Option<T>)> {
        self.data.indexed_iter_mut().map(|((r, c), v)| (r, c, v))
    }

    pub fn into_indexed_iter(self) -> impl Iterator<Item = (usize, usize, Option<T>)> {
        let cs = self.columns();
        self.data
            .into_iter()
            .enumerate()
            .map(move |(i, v)| (i / cs, i % cs, v))
    }

    pub fn all(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.iter().all(|cell| f(cell.as_ref()))
    }

    pub fn indexed_all(&self, f: impl Fn(usize, usize, Option<&T>) -> bool) -> bool {
        self.data
            .indexed_iter()
            .all(|((r, c), cell)| f(r, c, cell.as_ref()))
    }

    pub fn any(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.iter().any(|cell| f(cell.as_ref()))
    }

    pub fn indexed_any(&self, f: impl Fn(usize, usize, Option<&T>) -> bool) -> bool {
        self.data
            .indexed_iter()
            .any(|((r, c), cell)| f(r, c, cell.as_ref()))
    }

    pub fn filter(&self, f: impl Fn(Option<&T>) -> bool) -> impl Iterator<Item = Option<&T>> {
        self.data
            .iter()
            .filter(move |cell| f(cell.as_ref()))
            .map(|opt| opt.as_ref())
    }

    pub fn indexed_filter(
        &self,
        f: impl Fn(usize, usize, Option<&T>) -> bool,
    ) -> impl Iterator<Item = (usize, usize, Option<&T>)> {
        self.data
            .indexed_iter()
            .filter(move |((r, c), opt)| f(*r, *c, opt.as_ref()))
            .map(|((r, c), opt)| (r, c, opt.as_ref()))
    }

    pub fn retain(&mut self, f: impl Fn(Option<&T>) -> bool) {
        for cell in self.data.iter_mut() {
            if !f(cell.as_ref()) {
                *cell = None;
            }
        }
    }

    pub fn indexed_retain(&mut self, f: impl Fn(usize, usize, Option<&T>) -> bool) {
        for ((r, c), cell) in self.data.indexed_iter_mut() {
            if !f(r, c, cell.as_ref()) {
                *cell = None;
            }
        }
    }

    pub fn map<U>(&self, f: impl Fn(Option<&T>) -> Option<U>) -> impl Iterator<Item = Option<U>> {
        self.data.iter().map(move |cell| f(cell.as_ref()))
    }

    pub fn indexed_map<U>(
        &self,
        f: impl Fn(usize, usize, Option<&T>) -> Option<U>,
    ) -> impl Iterator<Item = (usize, usize, Option<U>)> {
        self.data
            .indexed_iter()
            .map(move |((r, c), cell)| (r, c, f(r, c, cell.as_ref())))
    }

    pub fn update(&mut self, f: impl Fn(Option<&T>) -> Option<T>) {
        for cell in self.data.iter_mut() {
            *cell = f(cell.as_ref());
        }
    }

    pub fn indexed_update(&mut self, f: impl Fn(usize, usize, Option<&T>) -> Option<T>) {
        for ((r, c), cell) in self.data.indexed_iter_mut() {
            *cell = f(r, c, cell.as_ref());
        }
    }

    pub fn cardinal_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = UPoint> {
        [(-1isize, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(dr, dc)| {
                let nr = r.checked_add_signed(dr)?;
                let nc = c.checked_add_signed(dc)?;
                if self.in_bounds(nr, nc) {
                    Some(UPoint::new(nr, nc))
                } else {
                    None
                }
            })
    }

    pub fn all_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = UPoint> {
        let deltas = [
            (-1isize, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        deltas.into_iter().filter_map(move |(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            if self.in_bounds(nr, nc) {
                Some(UPoint::new(nr, nc))
            } else {
                None
            }
        })
    }
}

impl<'a, T> IntoIterator for &'a Static2DGrid<T> {
    type Item = &'a Option<T>;
    type IntoIter = Iter<'a, Option<T>, Dim<[usize; 2]>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Static2DGrid<T> {
    type Item = &'a mut Option<T>;
    type IntoIter = IterMut<'a, Option<T>, Dim<[usize; 2]>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> IntoIterator for Static2DGrid<T> {
    type Item = Option<T>;

    type IntoIter = IntoIter<Option<T>, Dim<[usize; 2]>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
