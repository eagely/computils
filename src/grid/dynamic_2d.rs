use crate::grid_point::signed::GridPoint;
use std::{
    collections::{
        HashMap,
        hash_map::{IntoValues, Values, ValuesMut},
    },
    fmt::{Display, Formatter, Result},
};

#[derive(Clone)]
pub struct Dynamic2DGrid<T> {
    pub data: HashMap<GridPoint, T>,
}

impl<T> Dynamic2DGrid<T> {
    pub fn new(data: HashMap<GridPoint, T>) -> Self {
        Self { data }
    }

    pub fn rows(&self) -> Option<isize> {
        self.bounds().map(|(min, max)| max.r - min.r + 1)
    }

    pub fn columns(&self) -> Option<isize> {
        self.bounds().map(|(min, max)| max.c - min.c + 1)
    }

    pub fn get(&self, p: &GridPoint) -> Option<&T> {
        self.data.get(p)
    }

    pub fn get_mut(&mut self, p: &GridPoint) -> Option<&mut T> {
        self.data.get_mut(p)
    }

    pub fn set(&mut self, p: GridPoint, v: T) -> Option<T> {
        self.data.insert(p, v)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_blank(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.values_mut()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (GridPoint, &T)> {
        self.data.iter().map(|(p, v)| (*p, v))
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (GridPoint, &mut T)> {
        self.data.iter_mut().map(|(p, v)| (*p, v))
    }

    pub fn into_indexed_iter(self) -> impl Iterator<Item = (GridPoint, T)> {
        self.data.into_iter()
    }

    pub fn all(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().all(|v| f(Some(v))) && f(None)
    }

    pub fn indexed_all(&self, f: impl Fn(GridPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().all(|(p, v)| f(*p, Some(v)))
    }

    pub fn any(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().any(|v| f(Some(v))) || f(None)
    }

    pub fn indexed_any(&self, f: impl Fn(GridPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().any(|(p, v)| f(*p, Some(v)))
    }

    pub fn filter(&self, f: impl Fn(Option<&T>) -> bool) -> impl Iterator<Item = Option<&T>> {
        self.data.values().filter(move |v| f(Some(v))).map(Some)
    }

    pub fn indexed_filter(
        &self,
        f: impl Fn(GridPoint, Option<&T>) -> bool,
    ) -> impl Iterator<Item = (GridPoint, Option<&T>)> {
        self.data
            .iter()
            .filter(move |(p, v)| f(**p, Some(v)))
            .map(|(p, v)| (*p, Some(v)))
    }

    pub fn retain(&mut self, f: impl Fn(Option<&T>) -> bool) {
        self.data.retain(|_, v| f(Some(v)));
    }

    pub fn indexed_retain(&mut self, f: impl Fn(GridPoint, Option<&T>) -> bool) {
        self.data.retain(|p, v| f(*p, Some(v)));
    }

    pub fn map<U>(&self, f: impl Fn(Option<&T>) -> Option<U>) -> impl Iterator<Item = Option<U>> {
        self.data.values().map(move |v| f(Some(v)))
    }

    pub fn indexed_map<U>(
        &self,
        f: impl Fn(GridPoint, Option<&T>) -> Option<U>,
    ) -> impl Iterator<Item = (GridPoint, Option<U>)> {
        self.data.iter().map(move |(p, v)| (*p, f(*p, Some(v))))
    }

    pub fn update(&mut self, f: impl Fn(Option<&T>) -> Option<T>) {
        let mut updates = Vec::new();
        for (p, v) in &self.data {
            if let Some(new_val) = f(Some(v)) {
                updates.push((*p, new_val));
            }
        }
        for (p, new_val) in updates {
            self.data.insert(p, new_val);
        }
    }

    pub fn indexed_update(&mut self, f: impl Fn(GridPoint, Option<&T>) -> Option<T>) {
        let mut updates = Vec::new();
        for (p, v) in &self.data {
            if let Some(new_val) = f(*p, Some(v)) {
                updates.push((*p, new_val));
            }
        }
        for (p, new_val) in updates {
            self.data.insert(p, new_val);
        }
    }

    pub fn cardinal_neighbors(&self, p: GridPoint) -> impl Iterator<Item = &T> {
        p.cardinal_neighbors()
            .into_iter()
            .filter_map(|p| self.get(&p))
    }

    pub fn indexed_cardinal_neighbors(&self, p: GridPoint) -> impl Iterator<Item = GridPoint> {
        p.cardinal_neighbors()
            .into_iter()
            .filter(|p| self.get(p).is_some())
    }

    pub fn all_neighbors(&self, p: GridPoint) -> impl Iterator<Item = &T> {
        p.all_neighbors().into_iter().filter_map(|p| self.get(&p))
    }

    pub fn indexed_all_neighbors(&self, p: GridPoint) -> impl Iterator<Item = GridPoint> {
        p.all_neighbors()
            .into_iter()
            .filter(|p| self.get(p).is_some())
    }

    pub fn bounds(&self) -> Option<(GridPoint, GridPoint)> {
        if self.data.is_empty() {
            return None;
        }
        let (mut min_r, mut min_c, mut max_r, mut max_c) =
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
        for p in self.data.keys() {
            if p.r < min_r {
                min_r = p.r;
            }
            if p.c < min_c {
                min_c = p.c;
            }
            if p.r > max_r {
                max_r = p.r;
            }
            if p.c > max_c {
                max_c = p.c;
            }
        }
        Some((GridPoint::new(min_r, min_c), GridPoint::new(max_r, max_c)))
    }
}

impl<T> Default for Dynamic2DGrid<T> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Dynamic2DGrid<T> {
    type Item = &'a T;
    type IntoIter = Values<'a, GridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values()
    }
}

impl<'a, T> IntoIterator for &'a mut Dynamic2DGrid<T> {
    type Item = &'a mut T;
    type IntoIter = ValuesMut<'a, GridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values_mut()
    }
}

impl<T> IntoIterator for Dynamic2DGrid<T> {
    type Item = T;
    type IntoIter = IntoValues<GridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}

impl From<&str> for Dynamic2DGrid<char> {
    fn from(s: &str) -> Self {
        let mut data = HashMap::new();
        for (r, line) in s.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    data.insert(GridPoint::new(r as isize, c as isize), ch);
                }
            }
        }
        Dynamic2DGrid { data }
    }
}

impl Display for Dynamic2DGrid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.data.is_empty() {
            return Ok(());
        }

        let (min, max) = self.bounds().unwrap();
        for r in min.r..=max.r {
            for c in min.c..=max.c {
                let ch = self.get(&GridPoint::new(r, c));
                match ch {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
