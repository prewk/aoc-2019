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
//            Mode::Parameter => *program.ints.get(self.left.1 as usize).ok_or(ProgramErr::Missing { i: self.left.1 as usize })?,
            Mode::Parameter => *program.ints.get(self.left.1 as usize).unwrap_or(&0),
            Mode::Immediate => self.left.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.left.1) as usize).ok_or(ProgramErr::Missing { i: self.left.1 as usize })?,
        };
        let right_val = match self.right.0 {
//            Mode::Parameter => *program.ints.get(self.right.1 as usize).ok_or(ProgramErr::Missing { i: self.right.1 as usize })?,
            Mode::Parameter => *program.ints.get(self.right.1 as usize).unwrap_or(&0),
            Mode::Immediate => self.right.1,
            Mode::Relative => *program.ints.get((program.rel_base() + self.right.1) as usize).ok_or(ProgramErr::Missing { i: self.right.1 as usize })?,
        };
        let target = match self.target.0 {
            Mode::Parameter => self.target.1,
            Mode::Immediate => { return Err(ProgramErr::NeverImmediate); },
            Mode::Relative => program.rel_base() + self.target.1,
        } as usize;

        Ok(
            program
                .set_ints(target, left_val + right_val)
                .set_pointer(program.pointer + 4)
        )
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(4)?;
        let opcode = parse_opcode(ints[0])?;
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

        assert_eq!(program.ints[..], vec![6, 2, 3, 201, 1, 2, 0][..]);
        assert_eq!(program.pointer, 7);
        assert!(program.outputs.is_empty());
    }
}