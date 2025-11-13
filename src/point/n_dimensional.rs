use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NDPoint {
    pub coords: Vec<isize>,
}

impl NDPoint {
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

    pub fn neighbors(&self) -> Vec<NDPoint> {
        let ndim = self.ndim();
        let mut result = Vec::new();
        let mut deltas = vec![0; ndim];

        fn generate(
            deltas: &mut Vec<isize>,
            idx: usize,
            point: &NDPoint,
            result: &mut Vec<NDPoint>,
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

impl Add for NDPoint {
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

impl Sub for NDPoint {
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

impl Mul for NDPoint {
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

impl Div for NDPoint {
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

impl Rem for NDPoint {
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

impl Add<isize> for NDPoint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a + rhs).collect(),
        }
    }
}

impl Sub<isize> for NDPoint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a - rhs).collect(),
        }
    }
}

impl Mul<isize> for NDPoint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a * rhs).collect(),
        }
    }
}

impl Div<isize> for NDPoint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a / rhs).collect(),
        }
    }
}

impl Rem<isize> for NDPoint {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self {
            coords: self.coords.iter().map(|a| a % rhs).collect(),
        }
    }
}

impl Add<NDPoint> for isize {
    type Output = NDPoint;
    fn add(self, rhs: NDPoint) -> Self::Output {
        NDPoint {
            coords: rhs.coords.iter().map(|a| self + a).collect(),
        }
    }
}

impl Sub<NDPoint> for isize {
    type Output = NDPoint;
    fn sub(self, rhs: NDPoint) -> Self::Output {
        NDPoint {
            coords: rhs.coords.iter().map(|a| self - a).collect(),
        }
    }
}

impl Mul<NDPoint> for isize {
    type Output = NDPoint;
    fn mul(self, rhs: NDPoint) -> Self::Output {
        NDPoint {
            coords: rhs.coords.iter().map(|a| self * a).collect(),
        }
    }
}

impl Div<NDPoint> for isize {
    type Output = NDPoint;
    fn div(self, rhs: NDPoint) -> Self::Output {
        NDPoint {
            coords: rhs.coords.iter().map(|a| self / a).collect(),
        }
    }
}

impl Rem<NDPoint> for isize {
    type Output = NDPoint;
    fn rem(self, rhs: NDPoint) -> Self::Output {
        NDPoint {
            coords: rhs.coords.iter().map(|a| self % a).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::n_dimensional::NDPoint;

    #[test]
    fn test() {
        let p = NDPoint::new(vec![1, 2]);
        for n in p.neighbors() {
            println!("{:?}", n.coords);
        }
    }
}
