use core::f64::consts::PI;
use core::fmt::Display;
use core::ops::{Add, Div, Mul, Neg, Sub};

use libm::{atan2, cos, exp, sin};
use libm::{log, sqrt};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Complex {
    pub fn from_real(real: f64) -> Self {
        Complex { real, imag: 0. }
    }
    pub fn from_imag(imag: f64) -> Self {
        Complex { real: 0., imag }
    }

    pub fn squared_modulus(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn modulus(&self) -> f64 {
        sqrt(self.squared_modulus())
    }

    pub fn argument(&self) -> f64 {
        let a = atan2(self.imag, self.real);
        if a < 0. {
            a + 2. * PI
        } else {
            a
        }
    }

    pub fn polar(&self) -> (f64, f64) {
        (self.argument(), self.modulus())
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}
impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}
impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
impl Div<Complex> for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        self * Complex {
            real: rhs.real / (rhs.real * rhs.real + rhs.imag * rhs.imag),
            imag: -rhs.imag / (rhs.real * rhs.real + rhs.imag * rhs.imag),
        }
    }
}

impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Complex {
        Complex {
            real: self.real + rhs,
            imag: self.imag + rhs,
        }
    }
}
impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Complex {
        Complex {
            real: self.real - rhs,
            imag: self.imag - rhs,
        }
    }
}
impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Complex {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}
impl Div<f64> for Complex {
    type Output = Complex;
    fn div(self, rhs: f64) -> Complex {
        Complex {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

pub trait Pow<T> {
    type Output;

    fn pow(self, exp: T) -> Self::Output;
}
pub trait Exp {
    type Output;

    fn exp(self) -> Self::Output;
}
pub trait Log {
    type Output;

    fn log(self) -> Self::Output;
}
pub trait Trig {
    type Output;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
}

impl Exp for Complex {
    type Output = Complex;

    fn exp(self) -> Complex {
        Complex {
            real: cos(self.imag) * exp(self.real),
            imag: sin(self.imag) * exp(self.real),
        }
    }
}

impl Log for Complex {
    type Output = Complex;

    fn log(self) -> Complex {
        Complex {
            real: log(self.modulus()),
            imag: self.argument(),
        }
    }
}

impl Trig for Complex {
    type Output = Complex;

    fn sin(self) -> Complex {
        Complex {
            real: sin(self.real) * (exp(-self.imag) + exp(self.imag)) / 2.,
            imag: -cos(self.real) * (exp(-self.imag) - exp(self.imag)) / 2.,
        }
    }
    fn cos(self) -> Complex {
        Complex {
            real: cos(self.real) * (exp(-self.imag) + exp(self.imag)) / 2.,
            imag: sin(self.real) * (exp(-self.imag) - exp(self.imag)) / 2.,
        }
    }
}

impl Pow<f64> for Complex {
    type Output = Complex;

    fn pow(self, exp: f64) -> Complex {
        (self.log() * exp).exp()
    }
}
impl Pow<Complex> for Complex {
    type Output = Complex;

    fn pow(self, exp: Complex) -> Complex {
        (self.log() * exp).exp()
    }
}
