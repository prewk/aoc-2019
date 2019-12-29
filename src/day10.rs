use std::fmt;
use failure::_core::fmt::{Formatter, Error};
use std::collections::{HashSet};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Location {
    Asteroid(Option<u64>),
    Space,
}

pub fn char_to_location(ch: &char) -> Option<Location> {
    match ch {
        '#' => Some(Location::Asteroid(None)),
        '.' => Some(Location::Space),
        _ => None,
    }
}

pub fn calc_radial_coord(pt: (f64, f64)) -> f64 {
    (pt.0 * pt.0 + pt.1 * pt.1).sqrt()
}

pub fn calc_angular_coord(pt: (f64, f64)) -> f64 {
    pt.0.atan2(pt.1)
}

/// ```
/// // . . . p   . . . p
/// // . . . .   . . ./.
/// // . O . .   . . O--
/// // . . . .   . . | .
///
/// use aoc_2019::day10::calc_polar_coord;
/// assert_eq!(calc_polar_coord((1.0, 2.0), (3.0, 0.0)), ((2.0_f64 * 2.0_f64 + 2.0_f64 * 2.0_f64).sqrt(), 135.0));
/// ```
pub fn calc_polar_coord(pole: (f64, f64), point: (f64, f64)) -> (f64, f64) {
    let norm_point = (point.0 - pole.0, point.1 - pole.1);

    (calc_radial_coord(norm_point), (360.0 / (2.0 * std::f64::consts::PI) * calc_angular_coord(norm_point)))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    locations: Vec<Location>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(locations: &Vec<Location>, width: usize, height: usize) -> Map {
        Map {
            locations: locations.clone(),
            width,
            height,
        }
    }

    pub fn get_loc(&self, x: usize, y: usize) -> Option<Location> {
        self.locations.get((self.width * y) + x).map(|loc| *loc)
    }

    pub fn locations(&self) -> Vec<Location> {
        self.locations.clone()
    }

    pub fn process(&self) -> Map {
        let new_locations = self.locations.iter()
            .enumerate()
            .map(|(p_i, pole)| {
                let pole_coord = ((p_i % self.width) as f64, (p_i / self.width) as f64);

                match pole {
                    Location::Asteroid(None) => {
                        let mut others: HashSet<i64> = HashSet::new();

                        for (a_i, &ast) in self.locations.iter().enumerate() {
                            if a_i == p_i {
                                continue;
                            }

                            if let Location::Asteroid(_) = ast {
                                let ast_coord = ((a_i % self.width) as f64, (a_i / self.width) as f64);

                                let (_length, angle) = calc_polar_coord(pole_coord, ast_coord);

                                others.insert((angle * 1000.0) as i64);
                            }
                        }

                        Location::Asteroid(Some(others.len() as u64))
                    },
                    _ => *pole,
                }
            })
            .collect();

        Map::new(&new_locations, self.width, self.height)
    }

    pub fn get_highest_score(&self) -> Option<(i64, i64, u64)> {
        let mut highest: Option<(i64, i64, u64)> = None;

        for (i, loc) in self.locations.iter().enumerate() {
            let x = (i % self.width) as i64;
            let y = (i / self.width) as i64;

            if let Location::Asteroid(Some(c)) = loc {
                if let Some((_coord_x, _coord_y, cnt)) = highest {
                    if *c > cnt {
                        highest = Some((x, y, *c));
                    }
                } else {
                    highest = Some((x, y, *c));
                }
            }
        }

        highest
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut gfx = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let loc = self.get_loc(x, y);

                if let Some(Location::Asteroid(Some(n))) = loc {
                    gfx.push(match n {
                        0 => '0',
                        1 => '1',
                        2 => '2',
                        3 => '3',
                        4 => '4',
                        5 => '5',
                        6 => '6',
                        7 => '7',
                        8 => '8',
                        9 => '9',
                        _ => '+',
                    });
                } else if let Some(Location::Asteroid(None)) = loc {
                    gfx.push('#');
                } else if let Some(Location::Space) = loc {
                    gfx.push('.');
                }
            }

            gfx.push('\n');
        }

        write!(f, "{}", gfx)
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();

    let mut locs: Vec<Location> = vec![];
    let height = lines.len();
    let mut width = 0;
    for line in lines {
        width = line.len();
        for ch in line.chars() {
            locs.push(char_to_location(&ch).expect("Invalid character"));
        }
    }

    Map::new(&locs, width, height)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Map) -> u64 {
    input.process().get_highest_score().unwrap().2
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn test_process_small() {
        let input = ".#..#\n\
                           .....\n\
                           #####\n\
                           ....#\n\
                           ...##";
        let map = input_generator(input).process();

        println!("{}", map);

        assert_eq!(map.get_highest_score().unwrap(), (3, 4, 8));
    }

    #[test]
    fn test_process_large1() {
        let input = "......#.#.\n\
                           #..#.#....\n\
                           ..#######.\n\
                           .#.#.###..\n\
                           .#..#.....\n\
                           ..#....#.#\n\
                           #..#....#.\n\
                           .##.#..###\n\
                           ##...#..#.\n\
                           .#....####";
        let map = input_generator(input).process();

        assert_eq!(map.get_highest_score().unwrap(), (5, 8, 33));
    }

    #[test]
    fn test_process_large2() {
        let input = "#.#...#.#.\n\
                           .###....#.\n\
                           .#....#...\n\
                           ##.#.#.#.#\n\
                           ....#.#.#.\n\
                           .##..###.#\n\
                           ..#...##..\n\
                           ..##....##\n\
                           ......#...\n\
                           .####.###.";
        let map = input_generator(input).process();

        assert_eq!(map.get_highest_score().unwrap(), (1, 2, 35));
    }

    #[test]
    fn test_process_large3() {
        let input = ".#..#..###\n\
                           ####.###.#\n\
                           ....###.#.\n\
                           ..###.##.#\n\
                           ##.##.#.#.\n\
                           ....###..#\n\
                           ..#.#..#.#\n\
                           #..#.#.###\n\
                           .##...##.#\n\
                           .....#.#..";
        let map = input_generator(input).process();

        assert_eq!(map.get_highest_score().unwrap(), (6, 3, 41));
    }

    #[test]
    fn test_process_large4() {
        let input = ".#..##.###...#######\n\
                           ##.############..##.\n\
                           .#.######.########.#\n\
                           .###.#######.####.#.\n\
                           #####.##.#.##.###.##\n\
                           ..#####..#.#########\n\
                           ####################\n\
                           #.####....###.#.#.##\n\
                           ##.#################\n\
                           #####.##.###..####..\n\
                           ..######..##.#######\n\
                           ####.##.####...##..#\n\
                           .#####..#.######.###\n\
                           ##...#.##########...\n\
                           #.##########.#######\n\
                           .####.#.###.###.#.##\n\
                           ....##.##.###..#####\n\
                           .#.#.###########.###\n\
                           #.#.#.#####.####.###\n\
                           ###.##.####.##.#..##";
        let map = input_generator(input).process();

        assert_eq!(map.get_highest_score().unwrap(), (11, 13, 210));

    }
}