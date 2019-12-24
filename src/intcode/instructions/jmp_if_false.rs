use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct JmpIfFalse {
    val: (Mode, i64),
    target: (Mode, i64),
}

impl Instruction for JmpIfFalse {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let val = match self.val.0 {
            Mode::Parameter => *program.ints.get(self.val.1 as usize).ok_or(ProgramErr::Missing { i: self.val.1 as usize })?,
            Mode::Immediate => self.val.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.val.1) as usize).ok_or(ProgramErr::Missing { i: self.val.1 as usize })?,
        };
        let target = match self.target.0 {
            Mode::Parameter => *program.ints.get(self.target.1 as usize).ok_or(ProgramErr::Missing { i: self.target.1 as usize })?,
            Mode::Immediate => self.target.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.target.1) as usize).ok_or(ProgramErr::Missing { i: self.target.1 as usize })?,
        } as usize;

        if val == 0 {
            Ok(program.set_pointer(target))
        } else {
            Ok(program.set_pointer(program.pointer + 3))
        }
    }

    fn test(val: i64) -> bool {
        val % 10 == 6
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(3)?;
        let opcode = parse_opcode(ints[0])?;
        if !JmpIfFalse::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 6, found: opcode.opcode });
        }

        Ok(JmpIfFalse {
            val: (opcode.a, ints[1]),
            target: (opcode.b, ints[2]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let jump_if_true = JmpIfFalse::new(&Program::new(
            vec![1106, 0, 3, 666],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(jump_if_true.val, (Mode::Immediate, 0));
        assert_eq!(jump_if_true.target, (Mode::Immediate, 3));
    }

    #[test]
    fn test_test() {
        assert!(!JmpIfFalse::test(5));
        assert!(JmpIfFalse::test(6));
    }

    #[test]
    fn test_run_with_jump() {
        let program = Program::new(
            vec![1106, 0, 3, 666],
            0,
            vec![],
            vec![],
            0
        );

        let jump_if_true = JmpIfFalse::new(&program).unwrap();

        let program = jump_if_true.run(program).unwrap();

        assert_eq!(program.ints[..], vec![1106, 0, 3, 666][..]);
        assert_eq!(program.pointer, 3);
        assert!(program.outputs.is_empty());
    }
}