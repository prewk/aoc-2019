pub mod add;
pub mod mul;
pub mod input;
pub mod output;
pub mod jmp_if_true;
pub mod jmp_if_false;
pub mod lt;
pub mod eq;
pub mod exit;
pub mod offset;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    ints: Vec<i64>,
    pointer: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    has_exited: bool,
    rel_base: i64,
}

#[derive(Debug, Fail)]
pub enum ProgramErr {
    #[fail(display = "Can't construct {} from {:?}", kind, ints)]
    Unconstructable { kind: String, ints: Vec<i64> },
    #[fail(display = "Missing item {}", i)]
    Missing { i: usize },
    #[fail(display = "Couldn't construct modes from {:?}", v)]
    InvalidMode { v: Vec<i64> },
    #[fail(display = "Failed parsing instruction: {:?}", instr)]
    InvalidInstruction { instr: i64 },
    #[fail(display = "Intcode out of bounds: {}", i)]
    IntOutOfBounds { i: usize },
    #[fail(display = "Expected input, found None")]
    ExpectedInput,
    #[fail(display = "Expected output, found None")]
    ExpectedOutput,
    #[fail(display = "Encountered opcode {} doesn't match expected {}", found, expected)]
    OpcodeMismatch { expected: i64, found: i64 },
    #[fail(display = "Infinite loop detected")]
    InfiniteLoop,
    #[fail(display = "Parameters that an instruction writes to will never be in immediate mode")]
    NeverImmediate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Parameter,
    Immediate,
    Relative,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Opcode {
    pub opcode: i64,
    pub a: Mode,
    pub b: Mode,
    pub c: Mode,
}

pub trait Instruction {
    fn run(&self, program: Program) -> Result<Program, ProgramErr>;
    fn test(val: i64) -> bool;
    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized;
}

impl Program {
    pub fn new (ints: Vec<i64>, pointer: usize, outputs: Vec<i64>, inputs: Vec<i64>, rel_base: i64) -> Program {
        Program { ints, pointer, outputs, inputs, has_exited: false, rel_base }
    }

    fn exit(&self) -> Program {
        Program { ints: self.ints.clone(), pointer: self.pointer, outputs: self.outputs.clone(), inputs: self.inputs.clone(), has_exited: true, rel_base: self.rel_base }
    }

    pub fn has_exited(&self) -> bool {
        self.has_exited
    }

    pub fn ints(&self) -> Vec<i64> {
        self.ints.clone()
    }

    pub fn outputs(&self) -> Vec<i64> { self.outputs.clone() }

    pub fn inputs(&self) -> Vec<i64> { self.inputs.clone() }

    pub fn push_input(&self, val: i64) -> Program {
        let mut inputs = self.inputs.clone();
        inputs.push(val);

        Program { ints: self.ints.clone(), pointer: self.pointer, outputs: self.outputs.clone(), inputs, has_exited: false, rel_base: self.rel_base }
    }

    pub fn consume_input(&self) -> Program {
        let mut less_inputs = vec![];
        for (i, val) in self.inputs.iter().enumerate() {
            if i > 0 {
                less_inputs.push(*val);
            }
        }

        Program { ints: self.ints.clone(), pointer: self.pointer, outputs: self.outputs.clone(), inputs: less_inputs, has_exited: false, rel_base: self.rel_base }
    }

    pub fn pointer(&self) -> usize { self.pointer }

    pub fn set_ints(&self, index: usize, val: i64) -> Program {
        let mut ints: Vec<i64> = vec![];
        for (i, v) in self.ints.iter().enumerate() {
            if i == index {
                ints.push(val);
            } else {
                ints.push(v.clone());
            }
        }

        Program::new(ints, self.pointer, self.outputs.clone(), self.inputs.clone(), self.rel_base)
    }

    fn set_rel_base(&self, rel_base: i64) -> Program {
        Program::new(self.ints.clone(), self.pointer, self.outputs.clone(), self.inputs.clone(), rel_base)
    }

    pub fn rel_base(&self) -> i64 { self.rel_base }

    fn set_pointer(&self, pointer: usize) -> Program {
        Program::new(self.ints.clone(), pointer, self.outputs.clone(), self.inputs.clone(), self.rel_base)
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    fn push_output(&self, output: i64) -> Program {
        let mut outputs = self.outputs.clone();
        outputs.push(output);

        Program::new(self.ints.clone(), self.pointer, outputs, self.inputs.clone(), self.rel_base)
    }

    fn get_ints(&self, size: usize) -> Result<&[i64], ProgramErr> {
        if self.pointer + size > self.ints.len() {
            Err(ProgramErr::IntOutOfBounds { i: self.pointer + size })
        } else {
            Ok(&self.ints[self.pointer..(self.pointer + size)])
        }
    }

    pub fn peek(&self) -> Option<&i64> {
        self.ints.get(self.pointer)
    }
}

fn int_to_mode(int: i64) -> Result<Mode, ProgramErr> {
    match int {
        0 => Ok(Mode::Parameter),
        1 => Ok(Mode::Immediate),
        2 => Ok(Mode::Relative),
        _ => Err(ProgramErr::InvalidMode { v: vec![int] })
    }
}

/// ABCDE
/// |||||
/// ||||+- Instruction LSB
/// |||+-- Instruction MSB
/// ||+--- Mode Param A
/// |+---- Mode Param B
/// +----- Mode Param C
///
/// ```
/// use aoc_2019::intcode::instructions::*;
///
/// assert_eq!(
///     parse_opcode(11101).unwrap(),
///     Opcode { opcode: 1, a: Mode::Immediate, b: Mode::Immediate, c: Mode::Immediate }
/// );
/// assert_eq!(
///     parse_opcode(120).unwrap(),
///     Opcode { opcode: 20, a: Mode::Immediate, b: Mode::Parameter, c: Mode::Parameter }
/// );
/// assert_eq!(
///     parse_opcode(21233).unwrap(),
///     Opcode { opcode: 33, a: Mode::Relative, b: Mode::Immediate, c: Mode::Relative }
/// );
/// ```
pub fn parse_opcode(raw: i64) -> Result<Opcode, ProgramErr> {
    let mut opt_opcode: Option<i64> = None;
    let mut a_int = 0i64;
    let mut b_int = 0i64;
    let mut c_int = 0i64;

    if raw > 9999 {
        c_int = (raw / 10000) % 10;
        b_int = (raw / 1000) % 10;
        a_int = (raw / 100) % 10;
        opt_opcode = Some(raw % 100);
    } else if raw > 999 {
        b_int = (raw / 1000) % 10;
        a_int = (raw / 100) % 10;
        opt_opcode = Some(raw % 100);
    } else if raw > 99 {
        a_int = (raw / 100) % 10;
        opt_opcode = Some(raw % 100);
    } else if raw > 9 {
        opt_opcode = Some(raw);
    } else if raw > 0 {
        opt_opcode = Some(raw);
    }

    let a = int_to_mode(a_int)?;
    let b = int_to_mode(b_int)?;
    let c = int_to_mode(c_int)?;

    opt_opcode
        .ok_or(ProgramErr::InvalidInstruction { instr: raw })
        .map(|opcode| {
            Opcode {
                opcode,
                a,
                b,
                c,
            }
        })
}