use crate::grid_point::hex::HexGridPoint;
use std::collections::{
    HashMap,
    hash_map::{IntoValues, Values, ValuesMut},
};

#[derive(Clone)]
pub struct HexGrid<T> {
    pub data: HashMap<HexGridPoint, T>,
}

impl<T> HexGrid<T> {
    pub fn new(data: HashMap<HexGridPoint, T>) -> Self {
        Self { data }
    }

    pub fn rows(&self) -> Option<isize> {
        self.bounds().map(|(min, max)| max.r() - min.r() + 1)
    }

    pub fn columns(&self) -> Option<isize> {
        self.bounds().map(|(min, max)| max.q() - min.q() + 1)
    }

    pub fn get(&self, p: &HexGridPoint) -> Option<&T> {
        self.data.get(p)
    }

    pub fn get_mut(&mut self, p: &HexGridPoint) -> Option<&mut T> {
        self.data.get_mut(p)
    }

    pub fn set(&mut self, p: HexGridPoint, v: T) -> Option<T> {
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

    pub fn indexed_iter(&self) -> impl Iterator<Item = (HexGridPoint, &T)> {
        self.data.iter().map(|(p, v)| (*p, v))
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (HexGridPoint, &mut T)> {
        self.data.iter_mut().map(|(p, v)| (*p, v))
    }

    pub fn into_indexed_iter(self) -> impl Iterator<Item = (HexGridPoint, T)> {
        self.data.into_iter()
    }

    pub fn all(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().all(|v| f(Some(v))) && f(None)
    }

    pub fn indexed_all(&self, f: impl Fn(HexGridPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().all(|(p, v)| f(*p, Some(v)))
    }

    pub fn any(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().any(|v| f(Some(v))) || f(None)
    }

    pub fn indexed_any(&self, f: impl Fn(HexGridPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().any(|(p, v)| f(*p, Some(v)))
    }

    pub fn filter(&self, f: impl Fn(Option<&T>) -> bool) -> impl Iterator<Item = Option<&T>> {
        self.data.values().filter(move |v| f(Some(v))).map(Some)
    }

    pub fn indexed_filter(
        &self,
        f: impl Fn(HexGridPoint, Option<&T>) -> bool,
    ) -> impl Iterator<Item = (HexGridPoint, Option<&T>)> {
        self.data
            .iter()
            .filter(move |(p, v)| f(**p, Some(v)))
            .map(|(p, v)| (*p, Some(v)))
    }

    pub fn retain(&mut self, f: impl Fn(Option<&T>) -> bool) {
        self.data.retain(|_, v| f(Some(v)));
    }

    pub fn indexed_retain(&mut self, f: impl Fn(HexGridPoint, Option<&T>) -> bool) {
        self.data.retain(|p, v| f(*p, Some(v)));
    }

    pub fn map<U>(&self, f: impl Fn(Option<&T>) -> Option<U>) -> impl Iterator<Item = Option<U>> {
        self.data.values().map(move |v| f(Some(v)))
    }

    pub fn indexed_map<U>(
        &self,
        f: impl Fn(HexGridPoint, Option<&T>) -> Option<U>,
    ) -> impl Iterator<Item = (HexGridPoint, Option<U>)> {
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

    pub fn indexed_update(&mut self, f: impl Fn(HexGridPoint, Option<&T>) -> Option<T>) {
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

    pub fn neighbors(&self, p: HexGridPoint) -> impl Iterator<Item = &T> {
        p.neighbors()
            .into_iter()
            .filter_map(move |n| self.data.get(&n))
    }

    pub fn indexed_neighbors(&self, p: HexGridPoint) -> impl Iterator<Item = HexGridPoint> {
        p.neighbors()
            .into_iter()
            .filter(move |n| self.data.contains_key(n))
    }

    pub fn bounds(&self) -> Option<(HexGridPoint, HexGridPoint)> {
        if self.data.is_empty() {
            return None;
        }
        let (mut min_q, mut min_r, mut max_q, mut max_r) =
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
        for p in self.data.keys() {
            if p.q() < min_q {
                min_q = p.q();
            }
            if p.r() < min_r {
                min_r = p.r();
            }
            if p.q() > max_q {
                max_q = p.q();
            }
            if p.r() > max_r {
                max_r = p.r();
            }
        }
        Some((
            HexGridPoint::new(min_q, min_r),
            HexGridPoint::new(max_q, max_r),
        ))
    }
}

impl<T> Default for HexGrid<T> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<'a, T> IntoIterator for &'a HexGrid<T> {
    type Item = &'a T;
    type IntoIter = Values<'a, HexGridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values()
    }
}

impl<'a, T> IntoIterator for &'a mut HexGrid<T> {
    type Item = &'a mut T;
    type IntoIter = ValuesMut<'a, HexGridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values_mut()
    }
}

impl<T> IntoIterator for HexGrid<T> {
    type Item = T;
    type IntoIter = IntoValues<HexGridPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}
