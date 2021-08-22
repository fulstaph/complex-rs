use num_traits::{One, Pow, Zero};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Complex {
    real: f64,
    imag: f64
}

impl Complex {
    pub fn real(self: &Self) -> f64 {
        self.real
    }

    pub fn imag(self: &Self) -> f64 {
        self.imag
    }

    pub fn set_real(&mut self, real: f64) {
        self.real = real;
    }

    pub fn set_imag(&mut self, imag: f64) {
        self.imag = imag;
    }

    pub fn abs(&self) -> f64 {
        ((self.real.pow(2) + self.imag.pow(2)) as f64).sqrt()
    }

    pub fn abs_sq(&self) -> f64 {
        (self.real.pow(2) + self.imag.pow(2)) as f64
    }

    pub fn conjugate(&self) -> Complex {
        Complex::new(self.real, -self.imag)
    }

    pub fn new(real: f64, imag: f64) -> Complex {
        Complex{real, imag}
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Self) -> Self {
        Complex::new(self.real + other.real,self.imag + other.imag)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Self) -> Self {
        Complex::new(self.real - other.real,self.imag - other.imag)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(self.real * rhs.real - self.imag * rhs.imag, self.real * rhs.imag + self.imag * rhs.real)
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let denominator = rhs.abs_sq(); 
        Complex::new((self.real * rhs.real + self.imag * rhs.imag) / denominator, (self.imag * rhs.real - self.real * rhs.imag) / denominator)
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex::default()
    }

    fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imag == 0.0
    }

    fn set_zero(&mut self) {
        self.real = 0.0;
        self.imag = 0.0;   
    }
}

impl One for Complex {
    fn one() -> Self {
        Complex::new(1.0, 0.0)
    }

    fn is_one(&self) -> bool
    where
            Self: PartialEq, {
        // TODO: rhs allocates memory so use default comparison kekw
        self == &Self::one()
    }

    fn set_one(&mut self) {
        self.real = 1.0;
        self.imag = 0.0;
    }
}

mod tests {
    #[cfg(test)]    
    use super::*;

    #[test]
    fn getters() {
        let c = Complex::new(1.0, 2.0);
        
        assert_eq!(c.real(), 1.0);

        assert_eq!(c.imag(), 2.0);
    }

    #[test]
    fn multiplication() {
        let i = Complex::new(0.0, 1.0);
        assert_eq!((i * i).real(), -1.0);
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
}
