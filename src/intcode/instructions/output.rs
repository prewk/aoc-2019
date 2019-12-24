use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    val: (Mode, i64),
}

impl Instruction for Output {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let val = match self.val.0 {
            Mode::Parameter => *program.ints.get(self.val.1 as usize).ok_or(ProgramErr::Missing { i: self.val.1 as usize })?,
            Mode::Immediate => self.val.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.val.1) as usize).ok_or(ProgramErr::Missing { i: self.val.1 as usize })?,
        };

        Ok(
            program
                .push_output(val)
                .set_pointer(program.pointer + 2)
        )
    }

    fn test(val: i64) -> bool {
        val % 10 == 4
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(2)?;
        let opcode = parse_opcode(ints[0])?;
        if !Output::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 4, found: opcode.opcode })
        }

        Ok(Output {
            val: (opcode.a, ints[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let output = Output::new(&Program::new(
            vec![4, 3, 0, 666],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(output.val, (Mode::Parameter, 3));
    }

    #[test]
    fn test_test() {
        assert!(!Output::test(3));
        assert!(Output::test(4));
    }

    #[test]
    fn test_run() {
        let program = Program::new(
            vec![1, 2, 3, 4, 5, 666, 123],
            3,
            vec![],
            vec![],
            0
        );

        let output = Output::new(&program).unwrap();

        let program = output.run(program).unwrap();

        assert_eq!(program.ints[..], vec![1, 2, 3, 4, 5, 666, 123][..]);
        assert_eq!(program.pointer, 5);
        assert_eq!(program.outputs[..], vec![666][..]);
    }
}