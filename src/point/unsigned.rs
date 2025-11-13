use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct UPoint {
    pub r: usize,
    pub c: usize,
}

impl UPoint {
    pub fn new(r: usize, c: usize) -> Self {
        Self { r, c }
    }

    pub fn cardinal_neighbors(&self) -> [UPoint; 4] {
        let directions = [
            UPoint::new(usize::MAX, 0),
            UPoint::new(1, 0),
            UPoint::new(0, usize::MAX),
            UPoint::new(0, 1),
        ];
        directions.map(|dir| *self + dir)
    }

    pub fn all_neighbors(&self) -> [UPoint; 8] {
        let directions = [
            UPoint::new(usize::MAX, 0),
            UPoint::new(1, 0),
            UPoint::new(0, usize::MAX),
            UPoint::new(0, 1),
            UPoint::new(usize::MAX, usize::MAX),
            UPoint::new(usize::MAX, 1),
            UPoint::new(1, usize::MAX),
            UPoint::new(1, 1),
        ];
        directions.map(|dir| *self + dir)
    }
}

impl Add for UPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_add(other.r), self.c.wrapping_add(other.c))
    }
}

impl Sub for UPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_sub(other.r), self.c.wrapping_sub(other.c))
    }
}

impl Mul for UPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.r.wrapping_mul(other.r), self.c.wrapping_mul(other.c))
    }
}

impl Div for UPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::new(self.r / other.r, self.c / other.c)
    }
}

impl Rem for UPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new(self.r % other.r, self.c % other.c)
    }
}

impl Add<usize> for UPoint {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_add(rhs), self.c.wrapping_add(rhs))
    }
}

impl Sub<usize> for UPoint {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_sub(rhs), self.c.wrapping_sub(rhs))
    }
}

impl Mul<usize> for UPoint {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.r.wrapping_mul(rhs), self.c.wrapping_mul(rhs))
    }
}

impl Div<usize> for UPoint {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::new(self.r / rhs, self.c / rhs)
    }
}

impl Rem<usize> for UPoint {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self::new(self.r % rhs, self.c % rhs)
    }
}

impl Add<UPoint> for usize {
    type Output = UPoint;
    fn add(self, rhs: UPoint) -> Self::Output {
        UPoint::new(self.wrapping_add(rhs.r), self.wrapping_add(rhs.c))
    }
}

impl Sub<UPoint> for usize {
    type Output = UPoint;
    fn sub(self, rhs: UPoint) -> Self::Output {
        UPoint::new(self.wrapping_sub(rhs.r), self.wrapping_sub(rhs.c))
    }
}

impl Mul<UPoint> for usize {
    type Output = UPoint;
    fn mul(self, rhs: UPoint) -> Self::Output {
        UPoint::new(self.wrapping_mul(rhs.r), self.wrapping_mul(rhs.c))
    }
}

impl Div<UPoint> for usize {
    type Output = UPoint;
    fn div(self, rhs: UPoint) -> Self::Output {
        UPoint::new(self / rhs.r, self / rhs.c)
    }
}

impl Rem<UPoint> for usize {
    type Output = UPoint;
    fn rem(self, rhs: UPoint) -> Self::Output {
        UPoint::new(self % rhs.r, self % rhs.c)
    }
}
