use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Equals {
    left: (Mode, i64),
    right: (Mode, i64),
    target: (Mode, i64),
}

impl Instruction for Equals {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let left_val = match self.left.0 {
            Mode::Parameter => *program.ints.get(self.left.1 as usize).ok_or(ProgramErr::Missing { i: self.left.1 as usize })?,
            Mode::Immediate => self.left.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.left.1) as usize).ok_or(ProgramErr::Missing { i: self.left.1 as usize })?,
        };
        let right_val = match self.right.0 {
            Mode::Parameter => *program.ints.get(self.right.1 as usize).ok_or(ProgramErr::Missing { i: self.right.1 as usize })?,
            Mode::Immediate => self.right.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.right.1) as usize).ok_or(ProgramErr::Missing { i: self.right.1 as usize })?,
        };
        let target = match self.target.0 {
            Mode::Parameter => self.target.1,
            Mode::Immediate => { return Err(ProgramErr::NeverImmediate); },
            Mode::Relative => program.rel_base() + self.target.1,
        } as usize;

        if left_val == right_val {
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
        val % 10 == 8
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(4)?;
        let opcode = parse_opcode(ints[0])?;
        if !Equals::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 8, found: opcode.opcode })
        }

        Ok(Equals {
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
        let eq = Equals::new(&Program::new(
            vec![1108, 5, 7, 3],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(eq.left, (Mode::Immediate, 5));
        assert_eq!(eq.right, (Mode::Immediate, 7));
        assert_eq!(eq.target, (Mode::Parameter, 3));
    }

    #[test]
    fn test_test() {
        assert!(!Equals::test(7));
        assert!(Equals::test(8));
    }

    #[test]
    fn test_run() {
        let program = Program::new(
            vec![555, 555, 8, 0, 1, 6, 0],
            2,
            vec![],
            vec![],
            0
        );

        let eq = Equals::new(&program).unwrap();

        let program = eq.run(program).unwrap();

        assert_eq!(program.ints[..], vec![555, 555, 8, 0, 1, 6, 1][..]);
        assert_eq!(program.pointer, 6);
        assert!(program.outputs.is_empty());
    }
}