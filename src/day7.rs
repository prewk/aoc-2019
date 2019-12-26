use intcode::run_program;
use crate::intcode::instructions::{ProgramErr};
use std::collections::HashMap;
use failure::Error;

pub fn get_best_run(map: &HashMap<String, i64>) -> i64 {
    let mut best = 0;

    for (_, v) in map.to_owned() {
        if v > best {
            best = v;
        }
    }

    best
}

/// ```
/// use advent::day_07::{run_program_with_phase_settings};
///
/// assert_eq!(43210, run_program_with_phase_settings(
///     &vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
///     0,
///     4,
///     3,
///     2,
///     1,
///     0
/// ).unwrap());
///
/// assert_eq!(54321, run_program_with_phase_settings(
///     &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0],
///     0,
///     0,
///     1,
///     2,
///     3,
///     4
/// ).unwrap());
///
/// assert_eq!(65210, run_program_with_phase_settings(
///     &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
///     0,
///     1,
///     0,
///     4,
///     3,
///     2
/// ).unwrap());
/// ```
pub fn run_program_with_phase_settings(program: &Vec<i64>, initial_value: i64, a: i64, b: i64, c: i64, d: i64, e: i64) -> Result<i64, Error> {
    // A
    let a_res = run_program(&program, &vec![a, initial_value])?;
    let a_output = *a_res.outputs().first().ok_or(ProgramErr::Missing { i: 0 })?;
    // B
    let b_res = run_program(&program, &vec![b, a_output])?;
    let b_output = *b_res.outputs().first().ok_or(ProgramErr::Missing { i: 0 })?;
    // C
    let c_res = run_program(&program, &vec![c, b_output])?;
    let c_output = *c_res.outputs().first().ok_or(ProgramErr::Missing { i: 0 })?;
    // D
    let d_res = run_program(&program, &vec![d, c_output])?;
    let d_output = *d_res.outputs().first().ok_or(ProgramErr::Missing { i: 0 })?;
    // E
    let e_res = run_program(&program, &vec![e, d_output])?;

    Ok(*e_res.outputs().first().ok_or(ProgramErr::Missing { i: 0 })?)
}

pub fn run_program_without_feedback(program: &Vec<i64>) -> Result<HashMap<String, i64>, Error> {
    let mut runs = HashMap::new();

    for a in 0..=4 {
        for b in 0..=4 {
            if b == a { continue; }
            for c in 0..=4 {
                if c == b || c == a { continue; }
                for d in 0..=4 {
                    if d == c || d == b || d == a  { continue; }
                    for e in 0..=4 {
                        if e == d || e == c || e == b || e == a { continue; }
                        runs.insert(format!("{}.{}.{}.{}.{}", a, b, c, d, e), run_program_with_phase_settings(&program, 0, a, b, c, d, e)?);
                    }
                }
            }
        }
    }

    Ok(runs)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let runs = run_program_without_feedback(&input.to_vec()).unwrap();
    get_best_run(&runs)
}
