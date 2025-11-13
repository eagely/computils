use crate::point::n_dimensional::NDPoint;
use std::collections::{
    HashMap,
    hash_map::{IntoValues, Values, ValuesMut},
};

pub struct NDGrid<T> {
    pub data: HashMap<NDPoint, T>,
}

impl<T> NDGrid<T> {
    pub fn new(data: HashMap<NDPoint, T>) -> Self {
        Self { data }
    }

    pub fn get(&self, p: &NDPoint) -> Option<&T> {
        self.data.get(p)
    }

    pub fn get_mut(&mut self, p: &NDPoint) -> Option<&mut T> {
        self.data.get_mut(p)
    }

    pub fn set(&mut self, p: NDPoint, v: T) -> Option<T> {
        self.data.insert(p, v)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.values_mut()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (NDPoint, &T)> + '_ {
        self.data.iter().map(|(p, v)| (p.clone(), v))
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (NDPoint, &mut T)> + '_ {
        self.data.iter_mut().map(|(p, v)| (p.clone(), v))
    }

    pub fn into_indexed_iter(self) -> impl Iterator<Item = (NDPoint, T)> {
        self.data.into_iter()
    }

    pub fn all(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().all(|v| f(Some(v))) && f(None)
    }

    pub fn indexed_all(&self, f: impl Fn(NDPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().all(|(p, v)| f(p.clone(), Some(v)))
    }

    pub fn any(&self, f: impl Fn(Option<&T>) -> bool) -> bool {
        self.data.values().any(|v| f(Some(v))) || f(None)
    }

    pub fn indexed_any(&self, f: impl Fn(NDPoint, Option<&T>) -> bool) -> bool {
        self.data.iter().any(|(p, v)| f(p.clone(), Some(v)))
    }

    pub fn retain(&mut self, f: impl Fn(Option<&T>) -> bool) {
        self.data.retain(|_, v| f(Some(v)));
    }

    pub fn indexed_retain(&mut self, f: impl Fn(NDPoint, Option<&T>) -> bool) {
        self.data.retain(|p, v| f(p.clone(), Some(v)));
    }

    pub fn map<U>(&self, f: impl Fn(Option<&T>) -> Option<U>) -> impl Iterator<Item = Option<U>> {
        self.data.values().map(move |v| f(Some(v)))
    }

    pub fn indexed_map<U>(
        &self,
        f: impl Fn(NDPoint, Option<&T>) -> Option<U>,
    ) -> impl Iterator<Item = (NDPoint, Option<U>)> {
        self.data
            .iter()
            .map(move |(p, v)| (p.clone(), f(p.clone(), Some(v))))
    }

    pub fn update(&mut self, f: impl Fn(Option<&T>) -> Option<T>) {
        let mut updates = Vec::new();
        for (p, v) in &self.data {
            if let Some(new_val) = f(Some(v)) {
                updates.push((p.clone(), new_val));
            }
        }
        for (p, new_val) in updates {
            self.data.insert(p, new_val);
        }
    }

    pub fn indexed_update(&mut self, f: impl Fn(NDPoint, Option<&T>) -> Option<T>) {
        let mut updates = Vec::new();
        for (p, v) in &self.data {
            if let Some(new_val) = f(p.clone(), Some(v)) {
                updates.push((p.clone(), new_val));
            }
        }
        for (p, new_val) in updates {
            self.data.insert(p, new_val);
        }
    }

    pub fn neighbors(&self, p: NDPoint) -> impl Iterator<Item = &T> {
        p.neighbors().into_iter().filter_map(|p| self.get(&p))
    }

    pub fn indexed_neighbors(&self, p: NDPoint) -> impl Iterator<Item = NDPoint> {
        p.neighbors().into_iter().filter(|p| self.get(p).is_some())
    }
}

impl<T> Default for NDGrid<T> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<'a, T> IntoIterator for &'a NDGrid<T> {
    type Item = &'a T;
    type IntoIter = Values<'a, NDPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values()
    }
}

impl<'a, T> IntoIterator for &'a mut NDGrid<T> {
    type Item = &'a mut T;
    type IntoIter = ValuesMut<'a, NDPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.values_mut()
    }
}

impl<T> IntoIterator for NDGrid<T> {
    type Item = T;
    type IntoIter = IntoValues<NDPoint, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}
