use core::{
    f32::consts::{E, PI},
    fmt::{Display, Write},
    iter::IntoIterator,
    ops::{Deref, DerefMut},
    slice,
};

use heapless::{String, Vec};

use crate::complex::{Complex, Conj, Exp, InverseTrig, Log, Pow, Trig};

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
    ZConj,
    Number(f32),

    Conj,

    Imag,
    Pi,
    E,

    Add,
    Sub,
    Mul,
    Div,
    Pow,

    Sqrt,

    Exp,
    Ln,
    Log,

    Sin,
    Cos,
    Tan,

    Arcsin,
    Arccos,
    Arctan,
}

#[derive(Clone, Debug)]
pub enum FastMathInstr {
    Z,
    ZConj,
    Number(Complex),
    Conj,

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
    LnZ,

    Exp,
    Ln,

    Log(Complex),
    LogR(f32),
    LogZ,
    LogS,

    SinZ,
    CosZ,
    TanZ,

    Sin,
    Cos,
    Tan,

    ArcsinZ,
    ArccosZ,
    ArctanZ,

    Arcsin,
    Arccos,
    Arctan,
}

impl Display for MathInstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MathInstruction::Z => write!(f, "Z"),
            MathInstruction::ZConj => write!(f, "Z*"),
            MathInstruction::Number(x) => write!(f, "{}", x),
            MathInstruction::Conj => write!(f, "_"),

            MathInstruction::Imag => write!(f, "i"),
            MathInstruction::Pi => write!(f, "pi"),
            MathInstruction::E => write!(f, "e"),

            MathInstruction::Add => write!(f, "+"),
            MathInstruction::Sub => write!(f, "-"),
            MathInstruction::Mul => write!(f, "*"),
            MathInstruction::Div => write!(f, "/"),
            MathInstruction::Pow => write!(f, "^"),

            MathInstruction::Sqrt => write!(f, "sqrt"),

            MathInstruction::Exp => write!(f, "e^"),
            MathInstruction::Ln => write!(f, "ln"),

            MathInstruction::Log => write!(f, "log"),

            MathInstruction::Sin => write!(f, "sin"),
            MathInstruction::Cos => write!(f, "cos"),
            MathInstruction::Tan => write!(f, "tan"),

            MathInstruction::Arcsin => write!(f, "arcsin"),
            MathInstruction::Arccos => write!(f, "arccos"),
            MathInstruction::Arctan => write!(f, "arctan"),
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
                MathInstruction::ZConj => stack.push(z.conj()).unwrap(),
                MathInstruction::Number(x) => stack.push(Complex::from_real(*x)).unwrap(),
                MathInstruction::Conj => {
                    let c = stack.pop().unwrap();
                    stack.push(c.conj()).unwrap();
                }

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

                MathInstruction::Sqrt => {
                    let c = stack.pop().unwrap();
                    stack.push(c.pow(0.5)).unwrap();
                }

                MathInstruction::Exp => {
                    let c = stack.pop().unwrap();
                    stack.push(c.exp()).unwrap();
                }
                MathInstruction::Ln => {
                    let c = stack.pop().unwrap();
                    stack.push(c.log()).unwrap();
                }

                MathInstruction::Log => {
                    let base = stack.pop().unwrap();
                    let c = stack.pop().unwrap();
                    stack.push(c.log() / base.log()).unwrap();
                }

                MathInstruction::Sin => {
                    let c = stack.pop().unwrap();
                    stack.push(c.sin()).unwrap();
                }
                MathInstruction::Cos => {
                    let c = stack.pop().unwrap();
                    stack.push(c.cos()).unwrap();
                }
                MathInstruction::Tan => {
                    let c = stack.pop().unwrap();
                    stack.push(c.tan()).unwrap();
                }

                MathInstruction::Arcsin => {
                    let c = stack.pop().unwrap();
                    stack.push(c.arcsin()).unwrap();
                }
                MathInstruction::Arccos => {
                    let c = stack.pop().unwrap();
                    stack.push(c.arccos()).unwrap();
                }
                MathInstruction::Arctan => {
                    let c = stack.pop().unwrap();
                    stack.push(c.arctan()).unwrap();
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
                FastMathInstr::ZConj => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.conj();
                }
                FastMathInstr::Number(c) => {
                    stack_pointer += 1;
                    stack[stack_pointer] = *c;
                }
                FastMathInstr::Conj => {
                    stack[stack_pointer] = stack[stack_pointer].conj();
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
                FastMathInstr::Ln => {
                    stack[stack_pointer] = stack[stack_pointer].log();
                }

                FastMathInstr::ExpZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.exp();
                }
                FastMathInstr::LnZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.log();
                }

                FastMathInstr::Log(b) => {
                    stack[stack_pointer] = stack[stack_pointer].log() / *b;
                }
                FastMathInstr::LogR(b) => {
                    stack[stack_pointer] = stack[stack_pointer].log() / *b;
                }
                FastMathInstr::LogZ => {
                    stack[stack_pointer] = stack[stack_pointer].log() / z.log();
                }
                FastMathInstr::LogS => {
                    stack_pointer -= 1;
                    stack[stack_pointer] =
                        stack[stack_pointer].log() / stack[stack_pointer + 1].log();
                }

                FastMathInstr::Sin => {
                    stack[stack_pointer] = stack[stack_pointer].sin();
                }
                FastMathInstr::Cos => {
                    stack[stack_pointer] = stack[stack_pointer].cos();
                }
                FastMathInstr::Tan => {
                    stack[stack_pointer] = stack[stack_pointer].tan();
                }

                FastMathInstr::SinZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.sin();
                }
                FastMathInstr::CosZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.cos();
                }
                FastMathInstr::TanZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.tan();
                }

                FastMathInstr::Arcsin => {
                    stack[stack_pointer] = stack[stack_pointer].arcsin();
                }
                FastMathInstr::Arccos => {
                    stack[stack_pointer] = stack[stack_pointer].arccos();
                }
                FastMathInstr::Arctan => {
                    stack[stack_pointer] = stack[stack_pointer].arctan();
                }

                FastMathInstr::ArcsinZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.arcsin();
                }
                FastMathInstr::ArccosZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.arccos();
                }
                FastMathInstr::ArctanZ => {
                    stack_pointer += 1;
                    stack[stack_pointer] = z.arctan();
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

pub struct SyntaxError {
    pub op_index: usize,
}
pub trait Validate {
    fn validate(&self) -> Result<(), SyntaxError>;
}

impl Validate for Function {
    fn validate(&self) -> Result<(), SyntaxError> {
        let mut stack_size: isize = 0;

        for (op_index, instr) in self.into_iter().enumerate() {
            match instr {
                MathInstruction::Number(_)
                | MathInstruction::Pi
                | MathInstruction::E
                | MathInstruction::Z
                | MathInstruction::ZConj => stack_size += 1,

                MathInstruction::Imag
                | MathInstruction::Conj
                | MathInstruction::Sqrt
                | MathInstruction::Exp
                | MathInstruction::Ln
                | MathInstruction::Sin
                | MathInstruction::Cos
                | MathInstruction::Tan
                | MathInstruction::Arcsin
                | MathInstruction::Arccos
                | MathInstruction::Arctan
                    if stack_size > 0 => {}

                MathInstruction::Add
                | MathInstruction::Sub
                | MathInstruction::Mul
                | MathInstruction::Div
                | MathInstruction::Pow
                | MathInstruction::Log => stack_size -= 1,

                _ => return Err(SyntaxError { op_index }),
            }

            if stack_size <= 0 {
                return Err(SyntaxError { op_index });
            }
        }
        if stack_size != 1 {
            return Err(SyntaxError {
                op_index: usize::MAX,
            });
        }

        Ok(())
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
                    MathInstruction::ZConj => FastMathInstr::ZConj,
                    MathInstruction::Number(x) => {
                        if let Some(MathInstruction::Imag) = iter.peek() {
                            iter.next().unwrap();
                            FastMathInstr::Number(Complex::from_imag(x))
                        } else {
                            FastMathInstr::Number(Complex::from_real(x))
                        }
                    }
                    MathInstruction::Conj => FastMathInstr::Conj,

                    MathInstruction::Pi => FastMathInstr::Number(Complex::from_real(PI)),
                    MathInstruction::E => FastMathInstr::Number(Complex::from_real(E)),

                    MathInstruction::Imag => unreachable!(),

                    MathInstruction::Add => FastMathInstr::AddS,
                    MathInstruction::Sub => FastMathInstr::SubS,
                    MathInstruction::Mul => FastMathInstr::MulS,
                    MathInstruction::Div => FastMathInstr::DivS,
                    MathInstruction::Pow => FastMathInstr::PowS,

                    MathInstruction::Sqrt => FastMathInstr::PowR(0.5),

                    MathInstruction::Exp => FastMathInstr::Exp,
                    MathInstruction::Ln => FastMathInstr::Ln,

                    MathInstruction::Log => FastMathInstr::LogS,

                    MathInstruction::Sin => FastMathInstr::Sin,
                    MathInstruction::Cos => FastMathInstr::Cos,
                    MathInstruction::Tan => FastMathInstr::Tan,

                    MathInstruction::Arcsin => FastMathInstr::Arcsin,
                    MathInstruction::Arccos => FastMathInstr::Arccos,
                    MathInstruction::Arctan => FastMathInstr::Arctan,
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
                                    FastMathInstr::Ln => {
                                        iter.next().unwrap();
                                        FastMathInstr::LnZ
                                    }

                                    FastMathInstr::LogS => {
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
                                    FastMathInstr::Tan => {
                                        iter.next().unwrap();
                                        FastMathInstr::TanZ
                                    }

                                    FastMathInstr::Arcsin => {
                                        iter.next().unwrap();
                                        FastMathInstr::ArcsinZ
                                    }
                                    FastMathInstr::Arccos => {
                                        iter.next().unwrap();
                                        FastMathInstr::ArccosZ
                                    }
                                    FastMathInstr::Arctan => {
                                        iter.next().unwrap();
                                        FastMathInstr::ArctanZ
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

                                    FastMathInstr::LogS => {
                                        iter.next().unwrap();
                                        FastMathInstr::Log(x.log())
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
                                    FastMathInstr::Conj => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.conj())
                                    }

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
                                    FastMathInstr::Ln => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.log())
                                    }

                                    FastMathInstr::Log(b) => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.log() / b)
                                    }

                                    FastMathInstr::Sin => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.sin())
                                    }
                                    FastMathInstr::Cos => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.cos())
                                    }
                                    FastMathInstr::Tan => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.tan())
                                    }

                                    FastMathInstr::Arcsin => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.arcsin())
                                    }
                                    FastMathInstr::Arccos => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.arccos())
                                    }
                                    FastMathInstr::Arctan => {
                                        iter.next().unwrap();
                                        FastMathInstr::Number(c.arctan())
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

                FastMathInstr::Log(z) if z.is_real() => *instr = FastMathInstr::LogR(z.real),
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
