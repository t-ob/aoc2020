use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CyclotomicInteger(pub i64, pub i64);

impl CyclotomicInteger {
    pub fn new(u: i64, v: i64) -> CyclotomicInteger {
        CyclotomicInteger(u, v)
    }
}

impl Add for CyclotomicInteger {
    type Output = CyclotomicInteger;

    fn add(self, rhs: Self) -> Self::Output {
        CyclotomicInteger(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for CyclotomicInteger {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul for CyclotomicInteger {
    type Output = CyclotomicInteger;

    fn mul(self, rhs: Self) -> Self::Output {
        CyclotomicInteger(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0 + self.1 * rhs.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::CyclotomicInteger;

    #[test]
    pub fn test_multiplication() {
        let zeta = CyclotomicInteger(0, 1);

        assert_eq!(zeta * zeta, CyclotomicInteger(-1, 1));
        assert_eq!(zeta * zeta * zeta, CyclotomicInteger(-1, 0));
        assert_eq!(zeta * zeta * zeta * zeta, CyclotomicInteger(0, -1));
        assert_eq!(zeta * zeta * zeta * zeta * zeta, CyclotomicInteger(1, -1));
        assert_eq!(
            zeta * zeta * zeta * zeta * zeta * zeta,
            CyclotomicInteger(1, 0)
        );
    }
}
