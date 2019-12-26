use intcode::run_program;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut program = input.to_vec();

    for _ in 0..99999 {
        program.push(0);
    }

    let res = run_program(&program, &vec![1]).unwrap();

    *res.outputs().last().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut program = input.to_vec();

    for _ in 0..99999 {
        program.push(0);
    }

    let res = run_program(&program, &vec![2]).unwrap();

    *res.outputs().last().unwrap()
}