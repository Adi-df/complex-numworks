use core::{
    f32::consts::{E, PI},
    fmt::{Display, Write},
    iter::IntoIterator,
    ops::{Deref, DerefMut},
    slice,
};

use heapless::{String, Vec};

use crate::complex::{Complex, Exp, Log, Pow, Trig};

pub const FUNCTION_SIZE: usize = 255;
pub const FUNCTION_STRING_SIZE: usize = FUNCTION_SIZE * 8;

#[derive(Clone)]
pub struct Function {
    instructions: Vec<MathInstruction, FUNCTION_SIZE>,
}
#[derive(Clone)]
pub struct FastFunction {
    instructions: Vec<FastMathInstr, FUNCTION_SIZE>,
}
pub type StringFunction = String<FUNCTION_STRING_SIZE>;

#[derive(Clone, Debug)]
pub enum MathInstruction {
    Z,
    Number(f32),

    Imag,
    Pi,
    E,

    Add,
    Sub,
    Mul,
    Div,
    Pow,

    Exp,
    Log,

    Sqrt,

    Sin,
    Cos,
}

#[derive(Clone, Debug)]
pub enum FastMathInstr {
    Z,
    Number(Complex),

    Add(Complex),
    Sub(Complex),
    Mul(Complex),
    Div(Complex),
    Pow(Complex),

    AddR(f32),
    SubR(f32),
    MulR(f32),
    DivR(f32),
    PowR(f32),

    AddZ,
    SubZ,
    MulZ,
    DivZ,
    PowZ,

    AddS,
    SubS,
    MulS,
    DivS,
    PowS,

    ExpZ,
    LogZ,

    Exp,
    Log,

    SinZ,
    CosZ,

    Sin,
    Cos,
}

impl Display for MathInstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MathInstruction::Z => write!(f, "Z"),
            MathInstruction::Number(x) => write!(f, "{}", x),

            MathInstruction::Imag => write!(f, "i"),
            MathInstruction::Pi => write!(f, "pi"),
            MathInstruction::E => write!(f, "e"),

            MathInstruction::Add => write!(f, "+"),
            MathInstruction::Sub => write!(f, "-"),
            MathInstruction::Mul => write!(f, "*"),
            MathInstruction::Div => write!(f, "/"),
            MathInstruction::Pow => write!(f, "^"),

            MathInstruction::Exp => write!(f, "e^"),
            MathInstruction::Log => write!(f, "ln"),

            MathInstruction::Sin => write!(f, "sin"),
            MathInstruction::Cos => write!(f, "cos"),

            MathInstruction::Sqrt => write!(f, "sqrt"),
        }
    }
}

impl Function {
    pub fn from_slice(s: &[MathInstruction]) -> Self {
        Self {
            instructions: Vec::from_slice(s).unwrap(),
        }
    }
}

impl Default for Function {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}
impl Default for FastFunction {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

impl<'a> IntoIterator for &'a Function {
    type Item = &'a MathInstruction;
    type IntoIter = slice::Iter<'a, MathInstruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.iter()
    }
}
impl<'a> IntoIterator for &'a FastFunction {
    type Item = &'a FastMathInstr;
    type IntoIter = slice::Iter<'a, FastMathInstr>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.iter()
    }
}

impl Deref for Function {
    type Target = Vec<MathInstruction, FUNCTION_SIZE>;

    fn deref(&self) -> &Self::Target {
        &self.instructions
    }
}
impl Deref for FastFunction {
    type Target = Vec<FastMathInstr, FUNCTION_SIZE>;

    fn deref(&self) -> &Self::Target {
        &self.instructions
    }
}
impl DerefMut for Function {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instructions
    }
}
impl DerefMut for FastFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instructions
    }
}

pub trait Evaluate {
    fn eval(&self, z: Complex) -> Complex;
}

impl Evaluate for Function {
    fn eval(&self, z: Complex) -> Complex {
        let mut stack: Vec<Complex, 32> = Vec::new();

        for instr in self.iter() {
            match instr {
                MathInstruction::Z => stack.push(z).unwrap(),
                MathInstruction::Number(x) => stack.push(Complex::from_real(*x)).unwrap(),

                MathInstruction::Imag => {
                    let c = stack.pop().unwrap();
                    stack.push(c * Complex::from_imag(1.)).unwrap();
                }
                MathInstruction::Pi => stack.push(Complex::from_real(PI)).unwrap(),
                MathInstruction::E => stack.push(Complex::from_real(E)).unwrap(),

                MathInstruction::Add => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + rhs).unwrap();
                }
                MathInstruction::Sub => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - rhs).unwrap();
                }
                MathInstruction::Mul => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * rhs).unwrap();
                }
                MathInstruction::Div => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / rhs).unwrap();
                }
                MathInstruction::Pow => {
                    let exponent = stack.pop().unwrap();
                    let c = stack.pop().unwrap();
                    stack.push(c.pow(exponent)).unwrap();
                }

                MathInstruction::Exp => {
                    let c = stack.pop().unwrap();
                    stack.push(c.exp()).unwrap();
                }
                MathInstruction::Log => {
                    let c = stack.pop().unwrap();
                    stack.push(c.log()).unwrap();
                }

                MathInstruction::Sin => {
                    let c = stack.pop().unwrap();
                    stack.push(c.sin()).unwrap();
                }
                MathInstruction::Cos => {
                    let c = stack.pop().unwrap();
                    stack.push(c.cos()).unwrap();
                }

                MathInstruction::Sqrt => {
                    let c = stack.pop().unwrap();
                    stack.push(c.pow(0.5)).unwrap();
                }
            }
        }

        stack.pop().unwrap()
    }
}

impl Evaluate for FastFunction {
    fn eval(&self, z: Complex) -> Complex {
        let mut stack: [Complex; 32] = [Complex::ZERO; 32];
        let mut stack_pointer = 0;

        for instr in self.iter() {
            match instr {
                FastMathInstr::Z => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z;
                }
                FastMathInstr::Number(c) => {
                    stack_pointer += 1;
                    stack[stack_pointer] = *c;
                }

                FastMathInstr::Add(c) => {
                    stack[stack_pointer] += *c;
                }
                FastMathInstr::Sub(c) => {
                    stack[stack_pointer] -= *c;
                }
                FastMathInstr::Mul(c) => {
                    stack[stack_pointer] *= *c;
                }
                FastMathInstr::Div(c) => {
                    stack[stack_pointer] /= *c;
                }
                FastMathInstr::Pow(c) => {
                    stack[stack_pointer] = stack[stack_pointer].pow(*c);
                }

                FastMathInstr::AddR(r) => {
                    stack[stack_pointer] += *r;
                }
                FastMathInstr::SubR(r) => {
                    stack[stack_pointer] -= *r;
                }
                FastMathInstr::MulR(r) => {
                    stack[stack_pointer] *= *r;
                }
                FastMathInstr::DivR(r) => {
                    stack[stack_pointer] /= *r;
                }
                FastMathInstr::PowR(r) => {
                    stack[stack_pointer] = stack[stack_pointer].pow(*r);
                }

                FastMathInstr::AddZ => {
                    stack[stack_pointer] += z;
                }
                FastMathInstr::SubZ => {
                    stack[stack_pointer] -= z;
                }
                FastMathInstr::MulZ => {
                    stack[stack_pointer] *= z;
                }
                FastMathInstr::DivZ => {
                    stack[stack_pointer] /= z;
                }
                FastMathInstr::PowZ => {
                    stack[stack_pointer] = stack[stack_pointer].pow(z);
                }

                FastMathInstr::AddS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] += stack[stack_pointer + 1];
                }
                FastMathInstr::SubS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] -= stack[stack_pointer + 1];
                }
                FastMathInstr::MulS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] *= stack[stack_pointer + 1];
                }
                FastMathInstr::DivS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] /= stack[stack_pointer + 1];
                }
                FastMathInstr::PowS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] = stack[stack_pointer].pow(stack[stack_pointer + 1]);
                }

                FastMathInstr::Exp => {
                    stack[stack_pointer] = stack[stack_pointer].exp();
                }
                FastMathInstr::Log => {
                    stack[stack_pointer] = stack[stack_pointer].log();
                }

                FastMathInstr::ExpZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.exp();
                }
                FastMathInstr::LogZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.log();
                }

                FastMathInstr::Sin => {
                    stack[stack_pointer] = stack[stack_pointer].sin();
                }
                FastMathInstr::Cos => {
                    stack[stack_pointer] = stack[stack_pointer].cos();
                }

                FastMathInstr::SinZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.sin();
                }
                FastMathInstr::CosZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.cos();
                }
            }
        }

        stack[1]
    }
}

impl<T: Fn(Complex) -> Complex> Evaluate for T {
    fn eval(&self, z: Complex) -> Complex {
        self(z)
    }
}

impl From<Function> for FastFunction {
    fn from(func: Function) -> Self {
        // MathInstr to FastMathInstr && Simplify Number -> Imag to Number
        let mut fast_instr = {
            let mut iter = func.into_iter().peekable();
            let mut out = FastFunction::default();

            while let Some(instr) = iter.next() {
                out.push(match *instr {
                    MathInstruction::Z => FastMathInstr::Z,
                    MathInstruction::Number(x) => {
                        if let Some(MathInstruction::Imag) = iter.peek() {
                            iter.next().unwrap();
                            FastMathInstr::Number(Complex::from_imag(x))
                        } else {
                            FastMathInstr::Number(Complex::from_real(x))
                        }
                    }

                    MathInstruction::Pi => FastMathInstr::Number(Complex::from_real(PI)),
                    MathInstruction::E => FastMathInstr::Number(Complex::from_real(E)),

                    MathInstruction::Imag => unreachable!(),

                    MathInstruction::Add => FastMathInstr::AddS,
                    MathInstruction::Sub => FastMathInstr::SubS,
                    MathInstruction::Mul => FastMathInstr::MulS,
                    MathInstruction::Div => FastMathInstr::DivS,
                    MathInstruction::Pow => FastMathInstr::PowS,

                    MathInstruction::Exp => FastMathInstr::Exp,
                    MathInstruction::Log => FastMathInstr::Log,

                    MathInstruction::Sin => FastMathInstr::Sin,
                    MathInstruction::Cos => FastMathInstr::Cos,

                    MathInstruction::Sqrt => FastMathInstr::PowR(0.5),
                })
                .unwrap();
            }
            out
        };

        let mut previous_len = fast_instr.len() + 1;

        while fast_instr.len() < previous_len {
            previous_len = fast_instr.len();

            // Z && Number operation simplification
            let op_simplify = {
                let mut iter = fast_instr.into_iter().peekable();
                let mut out = FastFunction::default();

                while let Some(instr) = iter.next() {
                    out.push(match instr.clone() {
                        FastMathInstr::Z => {
                            if let Some(n_instr) = iter.peek() {
                                match n_instr {
                                    FastMathInstr::AddS => {
                                        iter.next().unwrap();
                                        FastMathInstr::AddZ
                                    }
                                    FastMathInstr::SubS => {
                                        iter.next().unwrap();
                                        FastMathInstr::SubZ
                                    }
                                    FastMathInstr::MulS => {
                                        iter.next().unwrap();
                                        FastMathInstr::MulZ
                                    }
                                    FastMathInstr::DivS => {
                                        iter.next().unwrap();
                                        FastMathInstr::DivZ
                                    }
                                    FastMathInstr::PowS => {
                                        iter.next().unwrap();
                                        FastMathInstr::PowZ
                                    }

                                    FastMathInstr::Exp => {
                                        iter.next().unwrap();
                                        FastMathInstr::ExpZ
                                    }
                                    FastMathInstr::Log => {
                                        iter.next().unwrap();
                                        FastMathInstr::LogZ
                                    }

                                    FastMathInstr::Sin => {
                                        iter.next().unwrap();
                                        FastMathInstr::SinZ
                                    }
                                    FastMathInstr::Cos => {
                                        iter.next().unwrap();
                                        FastMathInstr::CosZ
                                    }

                                    _ => FastMathInstr::Z,
                                }
                            } else {
                                FastMathInstr::Z
                            }
                        }
                        FastMathInstr::Number(x) => {
                            if let Some(n_instr) = iter.peek() {
                                match n_instr {
                                    FastMathInstr::AddS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Add(x)
                                    }
                                    FastMathInstr::SubS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Sub(x)
                                    }
                                    FastMathInstr::MulS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Mul(x)
                                    }
                                    FastMathInstr::DivS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Div(x)
                                    }
                                    FastMathInstr::PowS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Pow(x)
                                    }

                                    _ => FastMathInstr::Number(x),
                                }
                            } else {
                                FastMathInstr::Number(x)
                            }
                        }

                        i => i,
                    })
                    .unwrap();
                }
                out
            };

            // Compute number x number operations
            let pre_computed = {
                let mut iter = op_simplify.into_iter().peekable();
                let mut out = FastFunction::default();

                while let Some(instr) = iter.next() {
                    out.push(match instr.clone() {
                        FastMathInstr::Number(c) => {
                            if let Some(n_instr) = iter.peek() {
                                match **n_instr {
                                    FastMathInstr::Add(c2) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c + c2)
                                    }
                                    FastMathInstr::Sub(c2) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c - c2)
                                    }
                                    FastMathInstr::Mul(c2) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c * c2)
                                    }
                                    FastMathInstr::Div(c2) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c / c2)
                                    }
                                    FastMathInstr::Pow(c2) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.pow(c2))
                                    }

                                    FastMathInstr::Exp => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.exp())
                                    }
                                    FastMathInstr::Log => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.log())
                                    }

                                    FastMathInstr::Sin => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.sin())
                                    }
                                    FastMathInstr::Cos => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.cos())
                                    }

                                    _ => FastMathInstr::Number(c),
                                }
                            } else {
                                FastMathInstr::Number(c)
                            }
                        }

                        i => i,
                    })
                    .unwrap();
                }

                out
            };

            fast_instr = pre_computed;
        }

        // Simplify real numbers
        for instr in fast_instr.iter_mut() {
            match instr {
                FastMathInstr::Add(z) if z.is_real() => {
                    *instr = FastMathInstr::AddR(z.real);
                }
                FastMathInstr::Sub(z) if z.is_real() => {
                    *instr = FastMathInstr::SubR(z.real);
                }
                FastMathInstr::Mul(z) if z.is_real() => {
                    *instr = FastMathInstr::MulR(z.real);
                }
                FastMathInstr::Div(z) if z.is_real() => {
                    *instr = FastMathInstr::DivR(z.real);
                }
                FastMathInstr::Pow(z) if z.is_real() => {
                    *instr = FastMathInstr::PowR(z.real);
                }
                _ => {}
            }
        }

        fast_instr
    }
}

impl From<Function> for StringFunction {
    fn from(func: Function) -> Self {
        let mut s = StringFunction::new();

        for instr in &func.instructions {
            write!(&mut s, " {}", instr).unwrap();
        }
        s.push('\0').unwrap();

        s
    }
}
