use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub r: isize,
    pub c: isize,
}

impl Point {
    pub fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }

    pub fn cardinal_neighbors(&self) -> [Point; 4] {
        let directions = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];

        directions.map(|dir| *self + dir)
    }

    pub fn all_neighbors(&self) -> [Point; 8] {
        let directions = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(-1, -1),
            Point::new(-1, 1),
            Point::new(1, -1),
            Point::new(1, 1),
        ];

        directions.map(|dir| *self + dir)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.r + other.r, self.c + other.c)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.r - other.r, self.c - other.c)
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.r * other.r, self.c * other.c)
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.r / other.r, self.c / other.c)
    }
}

impl Rem for Point {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.r % other.r, self.c % other.c)
    }
}

impl Add<isize> for Point {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self::new(self.r + rhs, self.c + rhs)
    }
}

impl Sub<isize> for Point {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self::new(self.r - rhs, self.c - rhs)
    }
}

impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.r * rhs, self.c * rhs)
    }
}

impl Div<isize> for Point {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.r / rhs, self.c / rhs)
    }
}

impl Rem<isize> for Point {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self::new(self.r % rhs, self.c % rhs)
    }
}

impl Add<Point> for isize {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self + rhs.r, self + rhs.c)
    }
}

impl Sub<Point> for isize {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self - rhs.r, self - rhs.c)
    }
}

impl Mul<Point> for isize {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.r, self * rhs.c)
    }
}

impl Div<Point> for isize {
    type Output = Point;
    fn div(self, rhs: Point) -> Self::Output {
        Point::new(self / rhs.r, self / rhs.c)
    }
}

impl Rem<Point> for isize {
    type Output = Point;
    fn rem(self, rhs: Point) -> Self::Output {
        Point::new(self % rhs.r, self % rhs.c)
    }
}
