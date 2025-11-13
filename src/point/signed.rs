use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> isize {
        self.x * other.y - self.y * other.x
    }

    pub fn distance(self, other: Self) -> isize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Rem for Point {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl Add<isize> for Point {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub<isize> for Point {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<isize> for Point {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Rem<isize> for Point {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self::new(self.x % rhs, self.y % rhs)
    }
}

impl Add<Point> for isize {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self + rhs.x, self + rhs.y)
    }
}

impl Sub<Point> for isize {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self - rhs.x, self - rhs.y)
    }
}

impl Mul<Point> for isize {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<Point> for isize {
    type Output = Point;
    fn div(self, rhs: Point) -> Self::Output {
        Point::new(self / rhs.x, self / rhs.y)
    }
}

impl Rem<Point> for isize {
    type Output = Point;
    fn rem(self, rhs: Point) -> Self::Output {
        Point::new(self % rhs.x, self % rhs.y)
    }
}
