use crate::{math, point::signed::Point};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    pub a: isize,
    pub b: isize,
    pub c: isize,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        assert!(p1 != p2, "Line requires distinct points");
        let a = p1.y - p2.y;
        let b = p2.x - p1.x;
        let c = p1.x * p2.y - p2.x * p1.y;
        Self { a, b, c }.normalized()
    }

    pub fn normalized(self) -> Self {
        let g = math::gcd(math::gcd(self.a.abs(), self.b.abs()), self.c.abs()).max(1);
        let mut a = self.a / g;
        let mut b = self.b / g;
        let mut c = self.c / g;
        if a < 0 || (a == 0 && b < 0) {
            a = -a;
            b = -b;
            c = -c;
        }
        Self { a, b, c }
    }

    pub fn contains(&self, p: Point) -> bool {
        self.a * p.x + self.b * p.y + self.c == 0
    }

    pub fn signed_distance_num(&self, p: Point) -> isize {
        self.a * p.x + self.b * p.y + self.c
    }

    pub fn parallel(&self, other: &Self) -> bool {
        self.a * other.b == self.b * other.a
    }

    pub fn same(&self, other: &Self) -> bool {
        self.parallel(other)
            && self.a * other.c == self.c * other.a
            && self.b * other.c == self.c * other.b
    }

    pub fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let det = self.a * other.b - other.a * self.b;
        if det == 0 {
            return None;
        }
        let x = (self.b * other.c - other.b * self.c) as f64 / det as f64;
        let y = (self.c * other.a - other.c * self.a) as f64 / det as f64;
        Some((x, y))
    }

    pub fn direction(&self) -> Point {
        Point::new(self.b, -self.a)
    }

    pub fn normal(&self) -> Point {
        Point::new(self.a, self.b)
    }

    pub fn side(&self, p: Point) -> i8 {
        let v = self.signed_distance_num(p);
        if v == 0 {
            0
        } else if v > 0 {
            1
        } else {
            -1
        }
    }
}
