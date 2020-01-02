use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RelBaseOffset {
    diff: (Mode, i64)
}

impl Instruction for RelBaseOffset {
    fn run(&self, program: Program) -> Result<Program, ProgramErr> {
        let diff = match self.diff.0 {
            Mode::Parameter => program.get_int(self.diff.1),
            Mode::Immediate => self.diff.1,
            Mode::Relative => program.get_rel_int(self.diff.1),
        };

        Ok(
            program
                .set_rel_base(program.rel_base() + diff)
                .set_pointer(program.pointer + 2)
        )
    }

    fn test(val: i64) -> bool {
        val % 10 == 9
    }

    fn new(program: &Program) -> Result<Self, ProgramErr> where Self: std::marker::Sized {
        let ints = program.get_ints(2)?;
        let opcode = parse_opcode(*ints.get(0).ok_or(ProgramErr::Missing { i: 0 })?)?;
        if !RelBaseOffset::test(opcode.opcode) {
            return Err(ProgramErr::OpcodeMismatch { expected: 9, found: opcode.opcode });
        }

        Ok(RelBaseOffset {
            diff: (opcode.a, ints[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = RelBaseOffset::new(&Program::new(
            vec![109, 50, 1, 2, 3],
            0,
            vec![],
            vec![],
            1
        )).unwrap();

        assert_eq!(offset.diff, (Mode::Immediate, 50));
    }

    #[test]
    fn test_test() {
        assert!(!RelBaseOffset::test(8));
        assert!(RelBaseOffset::test(9));
    }

    #[test]
    fn test_run() {
        let program = Program::new(
            vec![109, 50, 1, 2, 3],
            0,
            vec![],
            vec![],
            1
        );

        let offset = RelBaseOffset::new(&program).unwrap();

        let program = offset.run(program).unwrap();

        assert_eq!(program.pointer, 2);
        assert_eq!(program.set_pointer(0).get_ints(5).unwrap()[..], vec![109, 50, 1, 2, 3][..]);
        assert_eq!(program.rel_base, 51);
        assert!(program.outputs.is_empty());
    }
}

