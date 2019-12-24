use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Exit {}

impl Instruction for Exit {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        Ok(program.exit())
    }

    fn test(val: i64) -> bool {
        val % 100 == 99
    }

    fn new(_: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        Ok(Exit {})
    }
}