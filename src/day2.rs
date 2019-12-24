#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split(',')
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect()
}

