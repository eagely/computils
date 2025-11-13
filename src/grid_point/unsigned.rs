use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct UGridPoint {
    pub r: usize,
    pub c: usize,
}

impl UGridPoint {
    pub fn new(r: usize, c: usize) -> Self {
        Self { r, c }
    }

    pub fn cardinal_neighbors(&self) -> [UGridPoint; 4] {
        let directions = [
            UGridPoint::new(usize::MAX, 0),
            UGridPoint::new(1, 0),
            UGridPoint::new(0, usize::MAX),
            UGridPoint::new(0, 1),
        ];
        directions.map(|dir| *self + dir)
    }

    pub fn all_neighbors(&self) -> [UGridPoint; 8] {
        let directions = [
            UGridPoint::new(usize::MAX, 0),
            UGridPoint::new(1, 0),
            UGridPoint::new(0, usize::MAX),
            UGridPoint::new(0, 1),
            UGridPoint::new(usize::MAX, usize::MAX),
            UGridPoint::new(usize::MAX, 1),
            UGridPoint::new(1, usize::MAX),
            UGridPoint::new(1, 1),
        ];
        directions.map(|dir| *self + dir)
    }
}

impl Add for UGridPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_add(other.r), self.c.wrapping_add(other.c))
    }
}

impl Sub for UGridPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_sub(other.r), self.c.wrapping_sub(other.c))
    }
}

impl Mul for UGridPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_mul(other.r), self.c.wrapping_mul(other.c))
    }
}

impl Div for UGridPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.r / other.r, self.c / other.c)
    }
}

impl Rem for UGridPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.r % other.r, self.c % other.c)
    }
}

impl Add<usize> for UGridPoint {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_add(rhs), self.c.wrapping_add(rhs))
    }
}

impl Sub<usize> for UGridPoint {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_sub(rhs), self.c.wrapping_sub(rhs))
    }
}

impl Mul<usize> for UGridPoint {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_mul(rhs), self.c.wrapping_mul(rhs))
    }
}

impl Div<usize> for UGridPoint {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::new(self.r / rhs, self.c / rhs)
    }
}

impl Rem<usize> for UGridPoint {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self::new(self.r % rhs, self.c % rhs)
    }
}

impl Add<UGridPoint> for usize {
    type Output = UGridPoint;
    fn add(self, rhs: UGridPoint) -> Self::Output {
        UGridPoint::new(self.wrapping_add(rhs.r), self.wrapping_add(rhs.c))
    }
}

impl Sub<UGridPoint> for usize {
    type Output = UGridPoint;
    fn sub(self, rhs: UGridPoint) -> Self::Output {
        UGridPoint::new(self.wrapping_sub(rhs.r), self.wrapping_sub(rhs.c))
    }
}

impl Mul<UGridPoint> for usize {
    type Output = UGridPoint;
    fn mul(self, rhs: UGridPoint) -> Self::Output {
        UGridPoint::new(self.wrapping_mul(rhs.r), self.wrapping_mul(rhs.c))
    }
}

impl Div<UGridPoint> for usize {
    type Output = UGridPoint;
    fn div(self, rhs: UGridPoint) -> Self::Output {
        UGridPoint::new(self / rhs.r, self / rhs.c)
    }
}

impl Rem<UGridPoint> for usize {
    type Output = UGridPoint;
    fn rem(self, rhs: UGridPoint) -> Self::Output {
        UGridPoint::new(self % rhs.r, self % rhs.c)
    }
}
