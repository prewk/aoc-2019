use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    target: (Mode, i64),
}

impl Instruction for Input {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let val = *program.inputs().get(0).ok_or(ProgramErr::ExpectedInput)?;

        let target = match self.target.0 {
            Mode::Parameter => self.target.1,
            Mode::Immediate => { return Err(ProgramErr::NeverImmediate); },
            Mode::Relative => program.rel_base() + self.target.1,
        } as usize;

        Ok(program
            .set_ints(target, val)
            .consume_input()
            .set_pointer(program.pointer + 2))
    }

    fn test(val: i64) -> bool {
        val % 10 == 3
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(2)?;

        let opcode = parse_opcode(ints[0])?;
        if !Input::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 3, found: opcode.opcode })
        }

        Ok(Input {
            target: (opcode.a, ints[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = Input::new(&Program::new(
            vec![1103, 123],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(input.target, (Mode::Immediate, 123));
    }

    #[test]
    fn test_test() {
        assert!(!Input::test(2));
        assert!(Input::test(3));
    }

    #[test]
    fn test_run_with_input() {
        let program = Program::new(
            vec![1, 2, 3, 0, 666],
            2,
            vec![],
            vec![123],
            0
        );

        let input = Input::new(&program).unwrap();

        let program = input.run(program).unwrap();

        assert_eq!(program.ints[..], vec![123, 2, 3, 0, 666][..]);
        assert_eq!(program.pointer, 4);
        assert!(program.outputs.is_empty());
    }

    #[test]
    fn test_run_without_input() {
        let program = Program::new(
            vec![1, 2, 3, 0, 666],
            2,
            vec![],
            vec![],
            0
        );

        let input = Input::new(&program).unwrap();

        assert!(input.run(program).is_err());
    }
}