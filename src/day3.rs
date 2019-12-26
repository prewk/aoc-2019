use voca_rs::*;
use line_intersection::{LineInterval};
use geo::{Line};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line
            .split(',')
            .map(|s| s.to_string())
            .collect()
        )
        .collect()
}

#[derive(Debug, Fail)]
pub enum CoordError {
    #[fail(display = "Couldn't parse direction")]
    ParseError,
    #[fail(display = "LineError: ({}, {})", x, y)]
    LineError {
        x: i64,
        y: i64,
    },
    #[fail(display = "Encountered a streak error")]
    StreakError,
    #[fail(display = "Missing manhattan distance")]
    MissingDistance,
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    let x_diff = a.0 - b.0;
    let y_diff = a.1 - b.1;

    x_diff.abs() + y_diff.abs()
}

enum Direction {
    Up(i64),
    Right(i64),
    Down(i64),
    Left(i64),
}

fn directions_to_coords(directions: &Vec<String>) -> Result<Vec<(i64, i64, i64)>, CoordError> {
    let mut dirs: Vec<Direction> = vec![];

    for dir_str in directions {
        let symbol = chop::first(dir_str, 1);
        let length = chop::slice(dir_str, 1, 0).parse::<i64>().map_err(|_| CoordError::ParseError)?;

        let dir = match symbol.as_ref() {
            "U" => Direction::Up(length),
            "R" => Direction::Right(length),
            "D" => Direction::Down(length),
            "L" => Direction::Left(length),
            _ => { return Err(CoordError::ParseError); }
        };

        dirs.push(dir);
    }

    let mut coords: Vec<(i64, i64, i64)> = vec![];
    let mut last_coord: (i64, i64, i64) = (0, 0, 0);

    for dir in dirs {
        let length: i64;
        match dir {
            Direction::Up(v) => {
                last_coord.1 += v;
                length = v;
            }
            Direction::Right(v) => {
                last_coord.0 += v;
                length = v;
            }
            Direction::Down(v) => {
                last_coord.1 -= v;
                length = v;
            }
            Direction::Left(v) => {
                last_coord.0 -= v;
                length = v;
            }
        }
        coords.push((last_coord.0, last_coord.1, length));
    }

    Ok(coords)
}

fn get_distance(cords: Vec<Vec<String>>) -> Result<i64, CoordError> {
    let cord1 = directions_to_coords(cords.get(0).ok_or(CoordError::ParseError)?)?;
    let cord2 = directions_to_coords(cords.get(1).ok_or(CoordError::ParseError)?)?;

    let mut manhattan_distances = vec![];
    let mut _combined_length: i64 = 0;
    let mut i1 = 0;
    for (x1_start, y1_start, l1_start) in &cord1 {
        if i1 == 0 {
            _combined_length += *l1_start;
        }

        if cord1.len() <= (i1 + 1) {
            break;
        }
        let (x1_end, y1_end, l1_end) = cord1.get(i1 + 1).ok_or(CoordError::LineError { x: *x1_start, y: *y1_start })?;

        let line1 = LineInterval::line_segment(Line {
            start: (*x1_start as f64, *y1_start as f64).into(),
            end: (*x1_end as f64, *y1_end as f64).into(),
        });

        _combined_length += *l1_end;

        let mut i2 = 0;

        for (x2_start, y2_start, l2_start) in &cord2 {
            if i2 == 0 {
                _combined_length += *l2_start;
            }

            if cord2.len() <= (i2 + 1) {
                break;
            }

            let (x2_end, y2_end, l2_end) = cord2.get(i2 + 1).ok_or(CoordError::LineError { x: *x2_start, y: *y2_start })?;

            let line2 = LineInterval::line_segment(Line {
                start: (*x2_start as f64, *y2_start as f64).into(),
                end: (*x2_end as f64, *y2_end as f64).into(),
            });

            _combined_length += *l2_end;

            let intersection = line1.relate(&line2).unique_intersection();

            match intersection {
                Some(v) => {
                    let distance = manhattan_distance((0, 0), (v.0.x as i64, v.0.y as i64));

                    manhattan_distances.push(distance);
                },
                None => {},
            }

            i2 += 1;
        }

        i1 += 1;
    }

    let mut shortest: Option<i64> = None;

    for distance in manhattan_distances {
        if let None = shortest {
            shortest = Some(distance);
        } else if let Some(s) = shortest {
            if distance < s {
                shortest = Some(distance);
            }
        }
    }

    shortest.ok_or(CoordError::MissingDistance)
}

#[aoc(day3, part1)]
pub fn solve_part1(cords: &[Vec<String>]) -> i64 {
    get_distance(cords.to_vec()).unwrap()
}
