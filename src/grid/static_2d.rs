use crate::grid_point::unsigned::UGridPoint;

use ndarray::{
    Array2, Dim,
    iter::{IntoIter, Iter, IterMut},
};
use std::fmt::{Display, Formatter, Result};

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

    pub fn find(&self, target: Option<&T>) -> Option<UGridPoint>
    where
        T: PartialEq,
    {
        self.indexed_iter()
            .find_map(|(r, c, cell)| (cell.as_ref() == target).then(|| UGridPoint::new(r, c)))
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

    pub fn cardinal_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = &T> {
        UGridPoint::new(r, c)
            .cardinal_neighbors()
            .into_iter()
            .filter_map(|p| self.get(p.r, p.c))
    }

    pub fn all_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = &T> {
        UGridPoint::new(r, c)
            .all_neighbors()
            .into_iter()
            .filter_map(|p| self.get(p.r, p.c))
    }

    pub fn indexed_cardinal_neighbors(
        &self,
        r: usize,
        c: usize,
    ) -> impl Iterator<Item = UGridPoint> {
        UGridPoint::new(r, c)
            .cardinal_neighbors()
            .into_iter()
            .filter(|p| self.get(p.r, p.c).is_some())
    }

    pub fn indexed_all_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = UGridPoint> {
        UGridPoint::new(r, c)
            .all_neighbors()
            .into_iter()
            .filter(|p| self.get(p.r, p.c).is_some())
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

impl From<&str> for Static2DGrid<char> {
    fn from(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let rows = lines.len();
        let cols = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let mut grid = Static2DGrid::new(rows, cols);

        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    grid.set(r, c, ch);
                }
            }
        }

        grid
    }
}

impl<T: Display> Display for Static2DGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for r in 0..self.rows() {
            for c in 0..self.columns() {
                match self.get(r, c) {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
