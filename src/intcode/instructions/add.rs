use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Add {
    left: (Mode, i64),
    right: (Mode, i64),
    target: (Mode, i64),
}

impl Instruction for Add {
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

        Ok(
            program
                .set_ints(target, left_val + right_val)
                .set_pointer(program.pointer + 4)
        )
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(4)?;
        let opcode = parse_opcode(*ints.get(0).ok_or(ProgramErr::Missing { i: 0 })?)?;
        if !Add::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 1, found: opcode.opcode })
        }

        Ok(Add {
            left: (opcode.a, ints[1]),
            right: (opcode.b, ints[2]),
            target: (opcode.c, ints[3]),
        })
    }

    fn test(val: i64) -> bool {
        val % 10 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let add = Add::new(&Program::new(
            vec![1101, 5, 7, 3],
            0,
            vec![],
            vec![],
            0
        )).unwrap();

        assert_eq!(add.left, (Mode::Immediate, 5));
        assert_eq!(add.right, (Mode::Immediate, 7));
        assert_eq!(add.target, (Mode::Parameter, 3));
    }

    #[test]
    fn test_test() {
        assert!(Add::test(1));
        assert!(!Add::test(2));
    }

    #[test]
    fn test_run() {
        let program = Program::new(
            vec![1, 2, 3, 201, 1, 2, 0],
            3,
            vec![],
            vec![],
            1
        );

        let add = Add::new(&program).unwrap();

        let program = add.run(program).unwrap();

        assert_eq!(program.pointer, 7);
        assert_eq!(program.set_pointer(0).get_ints(7).unwrap()[..], vec![6, 2, 3, 201, 1, 2, 0][..]);
        assert!(program.outputs.is_empty());
    }
}