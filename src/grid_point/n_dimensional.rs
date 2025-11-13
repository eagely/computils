use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NDGridPoint {
    pub coords: Vec<isize>,
}

impl NDGridPoint {
    pub fn new(coords: Vec<isize>) -> Self {
        Self { coords }
    }

    pub fn ndim(&self) -> usize {
        self.coords.len()
    }

    pub fn offset(&self, deltas: &[isize]) -> Self {
        assert_eq!(self.coords.len(), deltas.len());
        Self {
            coords: self.coords.iter().zip(deltas).map(|(a, b)| a + b).collect(),
        }
    }

    pub fn neighbors(&self) -> Vec<NDGridPoint> {
        let ndim = self.ndim();
        let mut result = Vec::new();
        let mut deltas = vec![0; ndim];

        fn generate(
            deltas: &mut Vec<isize>,
            idx: usize,
            point: &NDGridPoint,
            result: &mut Vec<NDGridPoint>,
        ) {
            if idx == deltas.len() {
                if deltas.iter().any(|&x| x != 0) {
                    result.push(point.offset(deltas));
                }
                return;
            }
            for &d in &[-1, 0, 1] {
                deltas[idx] = d;
                generate(deltas, idx + 1, point, result);
            }
        }

        generate(&mut deltas, 0, self, &mut result);
        result
    }
}

impl Add for NDGridPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.ndim(), other.ndim());
        Self {
            coords: self
                .coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl Sub for NDGridPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.ndim(), other.ndim());
        Self {
            coords: self
                .coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl Mul for NDGridPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.ndim(), other.ndim());
        Self {
            coords: self
                .coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }
}

impl Div for NDGridPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        assert_eq!(self.ndim(), other.ndim());
        Self {
            coords: self
                .coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| a / b)
                .collect(),
        }
    }
}

impl Rem for NDGridPoint {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        assert_eq!(self.ndim(), other.ndim());
        Self {
            coords: self
                .coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| a % b)
                .collect(),
        }
    }
}

impl Add<isize> for NDGridPoint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a + rhs).collect(),
        }
    }
}

impl Sub<isize> for NDGridPoint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a - rhs).collect(),
        }
    }
}

impl Mul<isize> for NDGridPoint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a * rhs).collect(),
        }
    }
}

impl Div<isize> for NDGridPoint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a / rhs).collect(),
        }
    }
}

impl Rem<isize> for NDGridPoint {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a % rhs).collect(),
        }
    }
}

impl Add<NDGridPoint> for isize {
    type Output = NDGridPoint;
    fn add(self, rhs: NDGridPoint) -> Self::Output {
        NDGridPoint {
            coords: rhs.coords.iter().map(|a| self + a).collect(),
        }
    }
}

impl Sub<NDGridPoint> for isize {
    type Output = NDGridPoint;
    fn sub(self, rhs: NDGridPoint) -> Self::Output {
        NDGridPoint {
            coords: rhs.coords.iter().map(|a| self - a).collect(),
        }
    }
}

impl Mul<NDGridPoint> for isize {
    type Output = NDGridPoint;
    fn mul(self, rhs: NDGridPoint) -> Self::Output {
        NDGridPoint {
            coords: rhs.coords.iter().map(|a| self * a).collect(),
        }
    }
}

impl Div<NDGridPoint> for isize {
    type Output = NDGridPoint;
    fn div(self, rhs: NDGridPoint) -> Self::Output {
        NDGridPoint {
            coords: rhs.coords.iter().map(|a| self / a).collect(),
        }
    }
}

impl Rem<NDGridPoint> for isize {
    type Output = NDGridPoint;
    fn rem(self, rhs: NDGridPoint) -> Self::Output {
        NDGridPoint {
            coords: rhs.coords.iter().map(|a| self % a).collect(),
        }
    }
}
