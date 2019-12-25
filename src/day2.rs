use intcode::run_program;
use intcode::instructions::Program;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut mut_input = input.to_vec();

    std::mem::replace(&mut mut_input[1], 12);
    std::mem::replace(&mut mut_input[2], 2);

    let res = run_program(&mut_input, &vec![]).unwrap();

    *res.ints().get(0).unwrap()
}

pub fn replace_and_process(program: &Program, noun: i64, verb: i64) -> i64 {
    let nouned = program.set_ints(1, noun);
    let verbed = nouned.set_ints(2, verb);

    let res = run_program(&verbed.ints(), &vec![]).unwrap();

    res
        .ints()
        .first()
        .map(|i| *i)
        .unwrap()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let program = Program::new(
        input.to_vec(),
        0,
        vec![],
        vec![],
        0
    );

    for noun in 0..98 {
        for verb in 0..98 {
            let val = replace_and_process(&program, noun, verb);
            if val == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Couldn't reach 19690720");
}
