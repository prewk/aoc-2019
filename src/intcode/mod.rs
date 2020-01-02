use crate::intcode::instructions::{Program, Instruction, ProgramErr};
use crate::intcode::instructions::add::Add;
use crate::intcode::instructions::mul::Mul;
use crate::intcode::instructions::input::Input;
use crate::intcode::instructions::output::Output;
use crate::intcode::instructions::jmp_if_false::JmpIfFalse;
use crate::intcode::instructions::jmp_if_true::JmpIfTrue;
use crate::intcode::instructions::lt::LessThan;
use crate::intcode::instructions::eq::Equals;
use crate::intcode::instructions::exit::Exit;
use crate::intcode::instructions::offset::RelBaseOffset;

pub mod instructions;

#[derive(Debug)]
pub enum InstrType {
    Add(Add),
    Mul(Mul),
    Input(Input),
    Output(Output),
    JmpIfFalse(JmpIfFalse),
    JmpIfTrue(JmpIfTrue),
    LessThan(LessThan),
    Equals(Equals),
    RelBaseOffset(RelBaseOffset),
    Exit(Exit),
}

pub fn get_instruction(program: &Program) -> Result<InstrType, ProgramErr> {
    let code = *program.peek().ok_or(ProgramErr::Missing { i: program.get_pointer() })?;

    if Add::test(code) {
        Ok(InstrType::Add(Add::new(program)?))
    } else if Mul::test(code) {
        Ok(InstrType::Mul(Mul::new(program)?))
    } else if Input::test(code) {
        Ok(InstrType::Input(Input::new(program)?))
    } else if Output::test(code) {
        Ok(InstrType::Output(Output::new(program)?))
    } else if JmpIfFalse::test(code) {
        Ok(InstrType::JmpIfFalse(JmpIfFalse::new(program)?))
    } else if JmpIfTrue::test(code) {
        Ok(InstrType::JmpIfTrue(JmpIfTrue::new(program)?))
    } else if LessThan::test(code) {
        Ok(InstrType::LessThan(LessThan::new(program)?))
    } else if Equals::test(code) {
        Ok(InstrType::Equals(Equals::new(program)?))
    } else if Exit::test(code) {
        Ok(InstrType::Exit(Exit::new(program)?))
    } else if RelBaseOffset::test(code) {
        Ok(InstrType::RelBaseOffset(RelBaseOffset::new(program)?))
    } else {
        Err(ProgramErr::InvalidInstruction { instr: code })
    }
}


pub fn run_program(program_ints: &Vec<i64>, inputs: &Vec<i64>) -> Result<Program, ProgramErr> {
    let mut program = Program::new(program_ints.clone(), 0, vec![], inputs.clone(), 0);

    loop {
        program = match get_instruction(&program)? {
            InstrType::Add(instr) => instr.run(program)?,
            InstrType::Mul(instr) => instr.run(program)?,
            InstrType::Input(instr) => instr.run(program)?,
            InstrType::Output(instr) => instr.run(program)?,
            InstrType::JmpIfFalse(instr) => instr.run(program)?,
            InstrType::JmpIfTrue(instr) => instr.run(program)?,
            InstrType::LessThan(instr) => instr.run(program)?,
            InstrType::Equals(instr) => instr.run(program)?,
            InstrType::Exit(instr) => instr.run(program)?,
            InstrType::RelBaseOffset(instr) => instr.run(program)?,
        };

        if program.has_exited() {
            return Ok(program)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(&vec![1,0,0,0,99], &vec![]).unwrap().set_pointer(0).get_ints(5).unwrap()[..],
            vec![2,0,0,0,99][..]
        );
        assert_eq!(
            run_program(&vec![2,3,0,3,99], &vec![]).unwrap().set_pointer(0).get_ints(5).unwrap()[..],
            vec![2,3,0,6,99][..]
        );
        assert_eq!(
            run_program(&vec![2,4,4,5,99,0], &vec![]).unwrap().set_pointer(0).get_ints(6).unwrap()[..],
            vec![2,4,4,5,99,9801][..]
        );
        assert_eq!(
            run_program(&vec![1,1,1,4,99,5,6,0,99], &vec![]).unwrap().set_pointer(0).get_ints(9).unwrap()[..],
            vec![30,1,1,4,2,5,6,0,99][..]
        );
        assert_eq!(
            run_program(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![999]).unwrap().outputs()[..],
            vec![1][..]
        );
        assert_eq!(
            run_program(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![0]).unwrap().outputs()[..],
            vec![0][..]
        );
        assert_eq!(
            run_program(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &vec![999]).unwrap().outputs()[..],
            vec![1][..]
        );
        assert_eq!(
            run_program(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![5]).unwrap().outputs()[..],
            vec![999][..]
        );
        assert_eq!(
            run_program(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![8]).unwrap().outputs()[..],
            vec![1000][..]
        );
        assert_eq!(
            run_program(&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![10]).unwrap().outputs()[..],
            vec![1001][..]
        );
    }
}