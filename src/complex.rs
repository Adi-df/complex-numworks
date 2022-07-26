use core::f64::consts::PI;
use core::fmt::Display;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
    pub const ZERO: Complex = Complex { real: 0., imag: 0. };

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

    pub fn is_real(&self) -> bool {
        self.imag == 0.
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

impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
impl SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}
impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        self.real = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
    }
}
impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        *self *= Complex {
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

impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        self.real += rhs;
        self.imag += rhs;
    }
}
impl SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        self.real -= rhs;
        self.imag -= rhs;
    }
}
impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        self.real *= rhs;
        self.imag *= rhs;
    }
}
impl DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        self.real /= rhs;
        self.imag /= rhs;
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
    type Output: Div<Self::Output>;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
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

impl Exp for Complex {
    type Output = Complex;

    fn exp(self) -> Complex {
        Complex {
            real: cos(self.imag) * exp(self.real),
            imag: sin(self.imag) * exp(self.real),
        }
    }
}
impl Exp for f64 {
    type Output = f64;
    fn exp(self) -> f64 {
        exp(self)
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
impl Log for f64 {
    type Output = f64;
    fn log(self) -> f64 {
        log(self)
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
impl Trig for f64 {
    type Output = f64;

    fn sin(self) -> f64 {
        sin(self)
    }
    fn cos(self) -> f64 {
        cos(self)
    }
}
