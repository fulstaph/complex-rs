use num_traits::{One, Pow, Zero};
use std::{fmt, ops::{Add, Div, Mul, Sub}};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn re(self: &Self) -> f64 {
        self.re
    }

    pub fn im(self: &Self) -> f64 {
        self.im
    }

    pub fn arg(&self) -> f64 {
        f64::atan2(self.im, self.re)
    }

    pub fn set_re(&mut self, re: f64) {
        self.re = re;
    }

    pub fn set_im(&mut self, im: f64) {
        self.im = im;
    }

    pub fn abs(&self) -> f64 {
        ((self.re.pow(2) + self.im.pow(2)) as f64).sqrt()
    }

    pub fn abs_sq(&self) -> f64 {
        (self.re.pow(2) + self.im.pow(2)) as f64
    }

    pub fn conjugate(&self) -> Complex {
        Complex::new(self.re, -self.im)
    }

    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn i() -> Complex {
        Complex {re: 0.0, im: 1.0}
    }

    pub fn i_ref() -> &'static Complex {
        &Complex {re: 0.0, im: 1.0}
    }
} 

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Complex { re, im: 0.0 } => write!(f, "{}", re),
            Complex { re: 0.0, im} => write!(f, "{}i", im),
            Complex { re, im} => write!(f, "{} + {}i", re, im)
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Self) -> Self {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let denominator = rhs.abs_sq();
        Complex::new(
            (self.re * rhs.re + self.im * rhs.im) / denominator,
            (self.im * rhs.re - self.re * rhs.im) / denominator,
        )
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex::default()
    }

    fn is_zero(&self) -> bool {
        self.re == 0.0 && self.im == 0.0
    }

    fn set_zero(&mut self) {
        self.re = 0.0;
        self.im = 0.0;
    }
}

impl One for Complex {
    fn one() -> Self {
        Complex::new(1.0, 0.0)
    }

    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        // TODO: rhs allocates memory so use default comparison kekw
        self == &Self::one()
    }

    fn set_one(&mut self) {
        self.re = 1.0;
        self.im = 0.0;
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn getters() {
        let c = Complex::new(1.0, 2.0);

        assert_eq!(c.re(), 1.0);

        assert_eq!(c.im(), 2.0);
    }

    #[test]
    fn multiplication() {
        let i = Complex::new(0.0, 1.0);
        assert_eq!((i * i).re(), -1.0);
    }

    #[test]
    fn check_addition_identity() {
        let c = Complex::zero();

        assert_eq!(c.is_zero(), true);
    }

    #[test]
    fn check_multiplication_identity() {
        let c = Complex::one();

        assert_eq!(c.is_one(), true);
    }

    #[test]
    fn check_abs() {
        assert_eq!(Complex::new(3.0, 4.0).abs(), 5.0)
    }

    #[test]
    fn check_set_zero() {
        let mut c = Complex::new(1.23, 9.324);
        c.set_zero();

        assert_eq!(c.is_zero(), true)
    }

    #[test]
    fn check_set_one() {
        let mut c = Complex::new(1.23, 9.324);
        c.set_one();

        assert_eq!(c.is_one(), true)
    }

    #[test]
    fn check_string_respresentation() {
        let c1 = Complex::new(1.03, 2.94);

        assert_eq!(format!("{}", c1), "1.03 + 2.94i");
    }
}
