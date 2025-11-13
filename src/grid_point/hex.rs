use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct HexGridPoint {
    q: isize,
    r: isize,
}

impl HexGridPoint {
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

    pub fn neighbors(&self) -> [HexGridPoint; 6] {
        let directions = [
            HexGridPoint::new(1, 0),
            HexGridPoint::new(1, -1),
            HexGridPoint::new(0, -1),
            HexGridPoint::new(-1, 0),
            HexGridPoint::new(-1, 1),
            HexGridPoint::new(0, 1),
        ];

        directions.map(|dir| *self + dir)
    }
}

impl Add for HexGridPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.q + other.q, self.r + other.r)
    }
}

impl Sub for HexGridPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.q - other.q, self.r - other.r)
    }
}

impl Mul for HexGridPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.q * other.q, self.r * other.r)
    }
}

impl Div for HexGridPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.q / other.q, self.r / other.r)
    }
}

impl Rem for HexGridPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.q % other.q, self.r % other.r)
    }
}

impl Add<isize> for HexGridPoint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self::new(self.q + rhs, self.r + rhs)
    }
}

impl Sub<isize> for HexGridPoint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self::new(self.q - rhs, self.r - rhs)
    }
}

impl Mul<isize> for HexGridPoint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.q * rhs, self.r * rhs)
    }
}

impl Div<isize> for HexGridPoint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.q / rhs, self.r / rhs)
    }
}

impl Rem<isize> for HexGridPoint {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self::new(self.q % rhs, self.r % rhs)
    }
}

impl Add<HexGridPoint> for isize {
    type Output = HexGridPoint;
    fn add(self, rhs: HexGridPoint) -> Self::Output {
        HexGridPoint::new(self + rhs.q, self + rhs.r)
    }
}

impl Sub<HexGridPoint> for isize {
    type Output = HexGridPoint;
    fn sub(self, rhs: HexGridPoint) -> Self::Output {
        HexGridPoint::new(self - rhs.q, self - rhs.r)
    }
}

impl Mul<HexGridPoint> for isize {
    type Output = HexGridPoint;
    fn mul(self, rhs: HexGridPoint) -> Self::Output {
        HexGridPoint::new(self * rhs.q, self * rhs.r)
    }
}

impl Div<HexGridPoint> for isize {
    type Output = HexGridPoint;
    fn div(self, rhs: HexGridPoint) -> Self::Output {
        HexGridPoint::new(self / rhs.q, self / rhs.r)
    }
}

impl Rem<HexGridPoint> for isize {
    type Output = HexGridPoint;
    fn rem(self, rhs: HexGridPoint) -> Self::Output {
        HexGridPoint::new(self % rhs.q, self % rhs.r)
    }
}
