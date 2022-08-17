use core::f32::consts::PI;
use core::fmt::Display;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use libm::{acosf, asinf, atan2f, atanf, cosf, expf, fabsf, logf, sinf, sqrtf, tanf};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

impl Display for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Complex {
    pub const ZERO: Complex = Complex { real: 0., imag: 0. };
    pub const I: Complex = Complex { real: 0., imag: 1. };

    pub fn from_real(real: f32) -> Self {
        Complex { real, imag: 0. }
    }
    pub fn from_imag(imag: f32) -> Self {
        Complex { real: 0., imag }
    }

    pub fn squared_modulus(&self) -> f32 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn modulus(&self) -> f32 {
        sqrtf(self.squared_modulus())
    }

    pub fn argument(&self) -> f32 {
        atan2f(self.imag, self.real)
    }

    pub fn polar(&self) -> (f32, f32) {
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
        *self = *self * rhs;
    }
}
impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        *self = *self / rhs;
    }
}

impl Add<f32> for Complex {
    type Output = Complex;
    fn add(self, rhs: f32) -> Complex {
        Complex {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}
impl Sub<f32> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f32) -> Complex {
        Complex {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}
impl Mul<f32> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f32) -> Complex {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}
impl Div<f32> for Complex {
    type Output = Complex;
    fn div(self, rhs: f32) -> Complex {
        Complex {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl AddAssign<f32> for Complex {
    fn add_assign(&mut self, rhs: f32) {
        self.real += rhs;
    }
}
impl SubAssign<f32> for Complex {
    fn sub_assign(&mut self, rhs: f32) {
        self.real -= rhs;
    }
}
impl MulAssign<f32> for Complex {
    fn mul_assign(&mut self, rhs: f32) {
        self.real *= rhs;
        self.imag *= rhs;
    }
}
impl DivAssign<f32> for Complex {
    fn div_assign(&mut self, rhs: f32) {
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
    type Output;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn tan(self) -> Self::Output;
}
pub trait InverseTrig {
    type Output;

    fn arcsin(self) -> Self::Output;
    fn arccos(self) -> Self::Output;
    fn arctan(self) -> Self::Output;
}
pub trait Conj {
    type Output;

    fn conj(self) -> Self::Output;
}

impl Pow<f32> for Complex {
    type Output = Complex;

    fn pow(self, exp: f32) -> Complex {
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
            real: cosf(self.imag) * expf(self.real),
            imag: sinf(self.imag) * expf(self.real),
        }
    }
}
impl Exp for f32 {
    type Output = f32;
    fn exp(self) -> f32 {
        expf(self)
    }
}

impl Log for Complex {
    type Output = Complex;

    fn log(self) -> Complex {
        Complex {
            real: logf(self.modulus()),
            imag: self.argument(),
        }
    }
}
impl Log for f32 {
    type Output = Complex;
    fn log(self) -> Complex {
        Complex {
            imag: if self < 0. { PI } else { 0. },
            real: logf(fabsf(self)),
        }
    }
}

impl Trig for Complex {
    type Output = Complex;

    fn sin(self) -> Complex {
        Complex {
            real: sinf(self.real) * (expf(-self.imag) + expf(self.imag)) / 2.,
            imag: -cosf(self.real) * (expf(-self.imag) - expf(self.imag)) / 2.,
        }
    }
    fn cos(self) -> Complex {
        Complex {
            real: cosf(self.real) * (expf(-self.imag) + expf(self.imag)) / 2.,
            imag: sinf(self.real) * (expf(-self.imag) - expf(self.imag)) / 2.,
        }
    }
    fn tan(self) -> Complex {
        let eiz = (Complex::I * self).exp();
        let emiz = (-Complex::I * self).exp();
        -Complex::I * (eiz - emiz) / (eiz + emiz)
    }
}
impl Trig for f32 {
    type Output = f32;

    fn sin(self) -> f32 {
        sinf(self)
    }
    fn cos(self) -> f32 {
        cosf(self)
    }
    fn tan(self) -> f32 {
        tanf(self)
    }
}

impl InverseTrig for Complex {
    type Output = Complex;

    fn arcsin(self) -> Complex {
        -Complex::I * ((Complex::from_real(1.) - self.pow(2.)).pow(0.5) + Complex::I * self).log()
    }
    fn arccos(self) -> Complex {
        -Complex::I * (Complex::I * (Complex::from_real(1.) - self.pow(2.)).pow(0.5)).log()
    }
    fn arctan(self) -> Complex {
        -Complex::I / 2.
            * ((Complex::from_real(1.) + Complex::I * self)
                / (Complex::from_real(1.) - Complex::I * self))
                .log()
    }
}
impl InverseTrig for f32 {
    type Output = f32;

    fn arcsin(self) -> f32 {
        asinf(self)
    }
    fn arccos(self) -> f32 {
        acosf(self)
    }
    fn arctan(self) -> f32 {
        atanf(self)
    }
}

impl Conj for Complex {
    type Output = Complex;

    fn conj(self) -> Self::Output {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl Conj for f32 {
    type Output = f32;

    fn conj(self) -> Self::Output {
        self
    }
}
