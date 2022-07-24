use core::{
    fmt::{Display, Write},
    iter::IntoIterator,
    ops::{Deref, DerefMut},
    slice,
};

use heapless::{String, Vec};

use crate::complex::{Complex, Exp, Log, Pow};

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
    Number(f64),
    Imag,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Exp,
    Log,
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
}

impl Display for MathInstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MathInstruction::Z => write!(f, "Z"),
            MathInstruction::Number(x) => write!(f, "{}", x),
            MathInstruction::Imag => write!(f, "i"),

            MathInstruction::Add => write!(f, "+"),
            MathInstruction::Sub => write!(f, "-"),
            MathInstruction::Mul => write!(f, "*"),
            MathInstruction::Div => write!(f, "/"),
            MathInstruction::Pow => write!(f, "^"),

            MathInstruction::Exp => write!(f, "e"),
            MathInstruction::Log => write!(f, "ln"),
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
            }
        }

        stack.pop().unwrap()
    }
}

impl Evaluate for FastFunction {
    fn eval(&self, z: Complex) -> Complex {
        let mut stack: Vec<Complex, 32> = Vec::new();

        for instr in self.iter() {
            match instr {
                FastMathInstr::Z => stack.push(z).unwrap(),
                FastMathInstr::Number(z) => stack.push(*z).unwrap(),

                FastMathInstr::Exp => {
                    let c = stack.pop().unwrap();
                    stack.push(c.exp()).unwrap();
                }
                FastMathInstr::Log => {
                    let c = stack.pop().unwrap();
                    stack.push(c.log()).unwrap();
                }

                FastMathInstr::ExpZ => stack.push(z.exp()).unwrap(),
                FastMathInstr::LogZ => stack.push(z.log()).unwrap(),

                FastMathInstr::Add(z) => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + *z).unwrap();
                }
                FastMathInstr::Sub(z) => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - *z).unwrap();
                }
                FastMathInstr::Mul(z) => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * *z).unwrap();
                }
                FastMathInstr::Div(z) => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / *z).unwrap();
                }
                FastMathInstr::Pow(z) => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs.pow(*z)).unwrap();
                }

                FastMathInstr::AddZ => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + z).unwrap();
                }
                FastMathInstr::SubZ => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - z).unwrap();
                }
                FastMathInstr::MulZ => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * z).unwrap();
                }
                FastMathInstr::DivZ => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / z).unwrap();
                }
                FastMathInstr::PowZ => {
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs.pow(z)).unwrap();
                }

                FastMathInstr::AddS => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + rhs).unwrap();
                }
                FastMathInstr::SubS => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - rhs).unwrap();
                }
                FastMathInstr::MulS => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * rhs).unwrap();
                }
                FastMathInstr::DivS => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / rhs).unwrap();
                }
                FastMathInstr::PowS => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs.pow(rhs)).unwrap();
                }
            }
        }

        stack.pop().unwrap()
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
                    MathInstruction::Imag => unreachable!(),

                    MathInstruction::Add => FastMathInstr::AddS,
                    MathInstruction::Sub => FastMathInstr::SubS,
                    MathInstruction::Mul => FastMathInstr::MulS,
                    MathInstruction::Div => FastMathInstr::DivS,
                    MathInstruction::Pow => FastMathInstr::PowS,

                    MathInstruction::Exp => FastMathInstr::Exp,
                    MathInstruction::Log => FastMathInstr::Log,
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
