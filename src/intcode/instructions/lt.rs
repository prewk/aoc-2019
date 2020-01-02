use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LessThan {
    left: (Mode, i64),
    right: (Mode, i64),
    target: (Mode, i64),
}

impl Instruction for LessThan {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let left_val = match self.left.0 {
            Mode::Parameter => program.get_int(self.left.1),
            Mode::Immediate => self.left.1,
            Mode::Relative => program.get_rel_int(self.left.1),
        };
        let right_val = match self.right.0 {
            Mode::Parameter => program.get_int(self.right.1),
            Mode::Immediate => self.right.1,
            Mode::Relative => program.get_rel_int(self.right.1),
        };
        let target = match self.target.0 {
            Mode::Parameter => self.target.1,
            Mode::Immediate => { return Err(ProgramErr::NeverImmediate); },
            Mode::Relative => program.rel_base() + self.target.1,
        };

        if left_val < right_val {
            Ok(
                program
                    .set_ints(target, 1)
                    .set_pointer(program.pointer + 4)
            )
        } else {
            Ok(
                program
                    .set_ints(target, 0)
                    .set_pointer(program.pointer + 4)
            )
        }
    }

    fn test(val: i64) -> bool {
        val % 10 == 7
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(4)?;
        let opcode = parse_opcode(*ints.get(0).ok_or(ProgramErr::Missing { i: 0 })?)?;
        if !LessThan::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 7, found: opcode.opcode })
        }

        Ok(LessThan {
            left: (opcode.a, ints[1]),
            right: (opcode.b, ints[2]),
            target: (opcode.c, ints[3]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lt = LessThan::new(&Program::new(
            vec![1107, 5, 7, 3],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(lt.left, (Mode::Immediate, 5));
        assert_eq!(lt.right, (Mode::Immediate, 7));
        assert_eq!(lt.target, (Mode::Parameter, 3));
    }

    #[test]
    fn test_test() {
        assert!(!LessThan::test(6));
        assert!(LessThan::test(7));
    }

    #[test]
    fn test_run() {
        let program = Program::new(
            vec![1, 2, 7, 0, 1, 6, 0],
            2,
            vec![],
            vec![],
            0
        );

        let lt = LessThan::new(&program).unwrap();

        let program = lt.run(program).unwrap();

        assert_eq!(program.pointer, 6);
        assert_eq!(program.set_pointer(0).get_ints(7).unwrap()[..], vec![1, 2, 7, 0, 1, 6, 1][..]);
        assert!(program.outputs.is_empty());
    }
}