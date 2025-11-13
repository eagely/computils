use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct HexPoint {
    q: isize,
    r: isize,
}

impl HexPoint {
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }

    pub fn q(&self) -> isize {
        self.q
    }

    pub fn r(&self) -> isize {
        self.r
    }

    pub fn s(&self) -> isize {
        -self.q - self.r
    }

    pub fn length(self) -> isize {
        (self.q.abs() + self.r.abs() + self.s().abs()) / 2
    }

    pub fn distance(self, other: Self) -> isize {
        (self - other).length()
    }

    pub fn neighbors(&self) -> [HexPoint; 6] {
        let directions = [
            HexPoint::new(1, 0),
            HexPoint::new(1, -1),
            HexPoint::new(0, -1),
            HexPoint::new(-1, 0),
            HexPoint::new(-1, 1),
            HexPoint::new(0, 1),
        ];

        directions.map(|dir| *self + dir)
    }
}

impl Add for HexPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.q + other.q, self.r + other.r)
    }
}

impl Sub for HexPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.q - other.q, self.r - other.r)
    }
}

impl Mul for HexPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.q * other.q, self.r * other.r)
    }
}

impl Div for HexPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.q / other.q, self.r / other.r)
    }
}

impl Rem for HexPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.q % other.q, self.r % other.r)
    }
}

impl Add<isize> for HexPoint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self::new(self.q + rhs, self.r + rhs)
    }
}

impl Sub<isize> for HexPoint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self::new(self.q - rhs, self.r - rhs)
    }
}

impl Mul<isize> for HexPoint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.q * rhs, self.r * rhs)
    }
}

impl Div<isize> for HexPoint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.q / rhs, self.r / rhs)
    }
}

impl Rem<isize> for HexPoint {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self::new(self.q % rhs, self.r % rhs)
    }
}

impl Add<HexPoint> for isize {
    type Output = HexPoint;
    fn add(self, rhs: HexPoint) -> Self::Output {
        HexPoint::new(self + rhs.q, self + rhs.r)
    }
}

impl Sub<HexPoint> for isize {
    type Output = HexPoint;
    fn sub(self, rhs: HexPoint) -> Self::Output {
        HexPoint::new(self - rhs.q, self - rhs.r)
    }
}

impl Mul<HexPoint> for isize {
    type Output = HexPoint;
    fn mul(self, rhs: HexPoint) -> Self::Output {
        HexPoint::new(self * rhs.q, self * rhs.r)
    }
}

impl Div<HexPoint> for isize {
    type Output = HexPoint;
    fn div(self, rhs: HexPoint) -> Self::Output {
        HexPoint::new(self / rhs.q, self / rhs.r)
    }
}

impl Rem<HexPoint> for isize {
    type Output = HexPoint;
    fn rem(self, rhs: HexPoint) -> Self::Output {
        HexPoint::new(self % rhs.q, self % rhs.r)
    }
}
