use std::fmt;
use failure::_core::fmt::{Formatter, Error, Display};
use std::collections::{HashSet, HashMap};

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

    pub fn process(&self) -> Map {
        let next_locations = self.locations
            .iter()
            .enumerate()
            .map(|(i, l)| {
                if let Location::Asteroid(None) = l {
                    let x = (i % self.width) as i64;
                    let y = (i / self.width) as i64;
                    let mut hits = 0;
                    let mut blacklist = vec![];

                    for r_int in 1..self.width {
                        let r = r_int as f64 - 0.4;
                        let c = Circle::new_from_radius(r, &blacklist);

                        for (c_x, c_y) in c.get_coords() {
                            let t_x = x + c_x;
                            let t_y = y + c_y;

                            if t_x < 0 || t_y < 0 {
                                continue;
                            }

                            if let Some(Location::Asteroid(a)) = self.get_loc(t_x as usize, t_y as usize) {
                                hits += 1;

                                if t_x == x && t_y == y {
                                    continue;
                                }

                                if let Some(v) = c.get_angles(c_x, c_y) {
                                    for a in v {
                                        blacklist.push(a);
                                    }
                                }
                            }
                        }
                    }
                    Location::Asteroid(Some(hits))
                } else {
                    *l
                }
            })
            .collect();

        Map::new(&next_locations, self.width, self.height)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut gfx = String::new();

        for y in 0..self.height {
            let mut line: Vec<&str> = vec![];
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
                        _ => 'x',
                    });
                } else if let Some(Location::Asteroid(None)) = loc {
                    gfx.push('o');
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
    123
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    coords: HashMap<(i64, i64), Vec<i64>>,
    radius: f64,
}

impl Circle {
    pub fn new_from_radius(radius: f64, blacklist: &Vec<i64>) -> Circle {
        let mut unit = vec![];

        // Get unit circle coords
        for a in 0..360_i64 {
            if blacklist.contains(&a) {
                continue;
            }

            let rads = (a as f64 * 2.0 * std::f64::consts::PI) / 360.0;

            let x = rads.cos();
            let y = rads.sin();

            unit.push((x, y, a));
        }

        // Adapt to radius
        let mut adapted: Vec<(f64, f64, i64)> = unit
            .iter()
            .map(|(x, y, a)| (x * radius, y * radius, *a))
            .collect();

        // Check which integer coords we hit
        let mut hits: HashMap<(i64, i64), Vec<i64>> = HashMap::new();

        for (x, y, a) in adapted {
            let x_i = x.floor();
            let y_i = y.floor();
            let key = (x_i as i64, y_i as i64);

            let default = &vec![];
            let mut angles = hits.get(&key).unwrap_or(default).clone();
            angles.push(a);

            hits.insert(key, angles.to_vec());
        }

        Circle { coords: hits, radius }
    }

    pub fn get_coords(&self) -> HashSet<(i64, i64)> {
        let mut out = HashSet::new();
        for ((x, y), _) in &self.coords {
            out.insert((*x, *y));
        }
        out
    }

    pub fn get_angles(&self, x: i64, y: i64) -> Option<Vec<i64>> {
        self.coords.get(&(x, y)).map(|v| v.clone())
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut gfx = String::new();
        let transpose_to = (self.radius.floor() * 2.0) as i64;
        let to = (self.radius * 2.0).ceil() as i64;
        let from = to * -1;

        for y in from..=to {
            for x in from..=to {
                if let Some(n) = self.coords.get(&(x, y)) {
                    gfx.push('#');
                } else {
                    gfx.push(' ');
                }
            }
            gfx.push('\n');
        }

        write!(f, "{}", gfx)
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    use std::collections::HashSet;

    #[test]
    fn test_that_it_processes_asteroids() {
        let m1 = Map::new(&vec![
            Location::Space, Location::Asteroid(None), Location::Space, Location::Space, Location::Asteroid(None),
            Location::Space, Location::Space, Location::Space, Location::Space, Location::Space,
            Location::Asteroid(None), Location::Asteroid(None), Location::Asteroid(None), Location::Asteroid(None), Location::Asteroid(None),
            Location::Space, Location::Space, Location::Space, Location::Space, Location::Asteroid(None),
            Location::Space, Location::Space, Location::Space, Location::Asteroid(None), Location::Asteroid(None),
        ], 5, 5);

        println!("{}", m1);

        let m2 = m1.process();

        println!("{}", m2);

        assert!(false);
    }
}
