use std::collections::HashMap;

#[derive(Debug, Fail)]
pub enum CodeError {
    #[fail(display = "Encountered a streak error")]
    StreakError,
    #[fail(display = "Encountered a parse error")]
    ParseError,
}

fn number_to_vec(n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn digits_in_a_row(digits: &Vec<u64>) -> Result<HashMap<u64, u64>, CodeError> {
    let mut digit_streaks: HashMap<u64, u64> = HashMap::new();

    for d in digits {
        let mut streak: u64 = 0;
        if digit_streaks.contains_key(d) {
            streak = *digit_streaks.get(d).ok_or(CodeError::StreakError)?;
        }

        streak += 1;
        digit_streaks.insert(*d, streak);
    }

    Ok(digit_streaks.to_owned())
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .split('-')
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect()
}

fn day_04a(bounds: &Vec<u64>) -> Result<Vec<u64>, CodeError> {
    let range = (*bounds.get(0).ok_or(CodeError::ParseError)?, *bounds.get(1).ok_or(CodeError::ParseError)?);

    let mut valids: Vec<u64> = vec![];
    for code in range.0..range.1 {
        let digits = number_to_vec(code);
        let digit_1 = digits.get(0).ok_or(CodeError::ParseError)?;
        let digit_2 = digits.get(1).ok_or(CodeError::ParseError)?;
        let digit_3 = digits.get(2).ok_or(CodeError::ParseError)?;
        let digit_4 = digits.get(3).ok_or(CodeError::ParseError)?;
        let digit_5 = digits.get(4).ok_or(CodeError::ParseError)?;
        let digit_6 = digits.get(5).ok_or(CodeError::ParseError)?;

        if *digit_1 == *digit_2 || *digit_2 == *digit_3 || *digit_3 == *digit_4 || *digit_4 == *digit_5 || *digit_5 == *digit_6 {

            if *digit_1 <= *digit_2 && *digit_2 <= *digit_3 && *digit_3 <= *digit_4 && *digit_4 <= *digit_5 && *digit_5 <= *digit_6 {
                valids.push(*digit_1 * 100_000 + *digit_2 * 10_000 + *digit_3 * 1_000 + *digit_4 * 100 + *digit_5 * 10  + *digit_6);
            }
        }
    }

    Ok(valids.to_owned())
}

fn day_04b(bounds: &Vec<u64>) -> Result<Vec<u64>, CodeError> {
    let range = (*bounds.get(0).ok_or(CodeError::ParseError)?, *bounds.get(1).ok_or(CodeError::ParseError)?);

    let mut valids: Vec<u64> = vec![];
    for code in range.0..range.1 {
        let digits = number_to_vec(code);
        let digit_1 = digits.get(0).ok_or(CodeError::ParseError)?;
        let digit_2 = digits.get(1).ok_or(CodeError::ParseError)?;
        let digit_3 = digits.get(2).ok_or(CodeError::ParseError)?;
        let digit_4 = digits.get(3).ok_or(CodeError::ParseError)?;
        let digit_5 = digits.get(4).ok_or(CodeError::ParseError)?;
        let digit_6 = digits.get(5).ok_or(CodeError::ParseError)?;

        if *digit_1 == *digit_2 || *digit_2 == *digit_3 || *digit_3 == *digit_4 || *digit_4 == *digit_5 || *digit_5 == *digit_6 {

            if *digit_1 <= *digit_2 && *digit_2 <= *digit_3 && *digit_3 <= *digit_4 && *digit_4 <= *digit_5 && *digit_5 <= *digit_6 {
                let map = digits_in_a_row(&digits).unwrap();

                for (_, v) in map {
                    if v == 2 {
                        valids.push(*digit_1 * 100_000 + *digit_2 * 10_000 + *digit_3 * 1_000 + *digit_4 * 100 + *digit_5 * 10  + *digit_6);
                        break;
                    }
                }
            }
        }
    }

    Ok(valids.to_owned())
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[u64]) -> usize {
    day_04a(&input.to_vec()).unwrap().len()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[u64]) -> usize {
    day_04b(&input.to_vec()).unwrap().len()
}
