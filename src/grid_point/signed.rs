use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct GridPoint {
    pub r: isize,
    pub c: isize,
}

impl GridPoint {
    pub fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }

    pub fn cardinal_neighbors(&self) -> [GridPoint; 4] {
        let directions = [
            GridPoint::new(-1, 0),
            GridPoint::new(1, 0),
            GridPoint::new(0, -1),
            GridPoint::new(0, 1),
        ];

        directions.map(|dir| *self + dir)
    }

    pub fn all_neighbors(&self) -> [GridPoint; 8] {
        let directions = [
            GridPoint::new(-1, 0),
            GridPoint::new(1, 0),
            GridPoint::new(0, -1),
            GridPoint::new(0, 1),
            GridPoint::new(-1, -1),
            GridPoint::new(-1, 1),
            GridPoint::new(1, -1),
            GridPoint::new(1, 1),
        ];

        directions.map(|dir| *self + dir)
    }
}

impl Add for GridPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.r + other.r, self.c + other.c)
    }
}

impl Sub for GridPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.r - other.r, self.c - other.c)
    }
}

impl Mul for GridPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.r * other.r, self.c * other.c)
    }
}

impl Div for GridPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.r / other.r, self.c / other.c)
    }
}

impl Rem for GridPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.r % other.r, self.c % other.c)
    }
}

impl Add<isize> for GridPoint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self::new(self.r + rhs, self.c + rhs)
    }
}

impl Sub<isize> for GridPoint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self::new(self.r - rhs, self.c - rhs)
    }
}

impl Mul<isize> for GridPoint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.r * rhs, self.c * rhs)
    }
}

impl Div<isize> for GridPoint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.r / rhs, self.c / rhs)
    }
}

impl Rem<isize> for GridPoint {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self::new(self.r % rhs, self.c % rhs)
    }
}

impl Add<GridPoint> for isize {
    type Output = GridPoint;
    fn add(self, rhs: GridPoint) -> Self::Output {
        GridPoint::new(self + rhs.r, self + rhs.c)
    }
}

impl Sub<GridPoint> for isize {
    type Output = GridPoint;
    fn sub(self, rhs: GridPoint) -> Self::Output {
        GridPoint::new(self - rhs.r, self - rhs.c)
    }
}

impl Mul<GridPoint> for isize {
    type Output = GridPoint;
    fn mul(self, rhs: GridPoint) -> Self::Output {
        GridPoint::new(self * rhs.r, self * rhs.c)
    }
}

impl Div<GridPoint> for isize {
    type Output = GridPoint;
    fn div(self, rhs: GridPoint) -> Self::Output {
        GridPoint::new(self / rhs.r, self / rhs.c)
    }
}

impl Rem<GridPoint> for isize {
    type Output = GridPoint;
    fn rem(self, rhs: GridPoint) -> Self::Output {
        GridPoint::new(self % rhs.r, self % rhs.c)
    }
}
