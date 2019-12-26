use intcode::run_program;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let res = run_program(&input.to_vec(), &vec![1]).unwrap();

    *res.outputs().last().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let res = run_program(&input.to_vec(), &vec![5]).unwrap();

    *res.outputs().last().unwrap()
}
