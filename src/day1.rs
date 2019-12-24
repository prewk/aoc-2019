#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<f64> {
    input
        .lines()
        .map(|line| line.parse::<f64>())
        .filter_map(Result::ok)
        .collect()
}

fn mass_to_fuel(mass: f64) -> i64 {
    ((mass / 3.0).floor() - 2.0) as i64
}

fn mass_to_fuel_rec(mass: f64) -> i64 {
    let fuel = ((mass / 3.0).floor() - 2.0) as i64;
    if fuel > 0 {
        fuel as i64 + mass_to_fuel_rec(fuel as f64)
    } else {
        0
    }
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[f64]) -> i64 {
    input.iter().fold(0, |total, mass| {
        total + mass_to_fuel(*mass)
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[f64]) -> i64 {
    input.iter().fold(0, |total, mass| {
        total + mass_to_fuel_rec(*mass)
    })
}

