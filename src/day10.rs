use std::fmt;
use failure::_core::fmt::{Formatter, Error};
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CoordToOther {
    x: f64,
    y: f64,
    radial: f64,
    angular: i64,
}

fn laser_hit(grouped_by_angle: &mut HashMap<i64, Vec<CoordToOther>>, angle: i64) -> Option<CoordToOther> {
    let i_love_clones = grouped_by_angle.clone();
    let targets = i_love_clones.get(&angle);

    if targets.is_some() {
        let v = targets.unwrap();
        if let None = v.get(0) {
            return None;
        }

        let first = v.get(0).unwrap();

        let mut new_v: Vec<CoordToOther> = vec![];
        for c in v.iter().skip(1) {
            new_v.push(*c);
        }

        grouped_by_angle.insert(angle, new_v);
        return Some(*first);
    }

    return None;
}

pub fn sort_coords_by_laser_hit(coords: &Vec<CoordToOther>) -> Option<Vec<CoordToOther>> {
    let mut all_angles = HashSet::new();
    for &coord in coords {
        all_angles.insert(coord.angular);
    }

    let mut grouped_by_angle = HashMap::new();
    for angle in &all_angles {
        let mut coords_for_angle: Vec<CoordToOther> = coords.clone()
            .iter()
            .filter(|c| c.angular == *angle)
            .map(|c| *c)
            .collect();

        coords_for_angle.sort_by(|a, b| {
            a.radial.partial_cmp(&b.radial).unwrap_or(Ordering::Equal)
        });

        grouped_by_angle.insert(*angle, coords_for_angle);
    }

    let mut hits: Vec<CoordToOther> = vec![];

    laser_hit(&mut grouped_by_angle, 0)
        .map(|c| {
            hits.push(c);
//            println!("{}/{}", hits.len(), coords.len());
        });

    while hits.len() < coords.len() {
        for i in (0..36000).rev() {
            laser_hit(&mut grouped_by_angle, i)
                .map(|c| {
                    hits.push(c);
//                    println!("{}/{}", hits.len(), coords.len());
                });
        }
    }

    Some(hits)
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Location {
    Asteroid(Option<usize>),
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
    angle_lookup: HashMap<(i64, i64), Vec<CoordToOther>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(locations: &Vec<Location>, width: usize, height: usize, angle_lookup: &HashMap<(i64, i64), Vec<CoordToOther>>) -> Map {
        Map {
            locations: locations.clone(),
            width,
            height,
            angle_lookup: angle_lookup.clone(),
        }
    }

    pub fn get_loc(&self, x: usize, y: usize) -> Option<Location> {
        self.locations.get((self.width * y) + x).map(|loc| *loc)
    }

    pub fn locations(&self) -> Vec<Location> {
        self.locations.clone()
    }

    pub fn lookup_angles(&self, x: i64, y: i64) -> Option<Vec<CoordToOther>> {
        self.angle_lookup.get(&(x, y))
            .map(|v| {
                v.iter().map(|c| {
                    let mut norm = (c.angular - 18000) % 36000;
                    if norm < 0 {
                        norm = 36000 + norm;
                    }

                    CoordToOther {
                        x: c.x,
                        y: c.y,
                        radial: c.radial,
                        angular: norm,
                    }
                }).collect()
            })
    }

    pub fn process(&self) -> Map {
        let mut angle_lookup: HashMap<(i64, i64), Vec<CoordToOther>> = HashMap::new();

        let new_locations = self.locations.iter()
            .enumerate()
            .map(|(p_i, pole)| {
                let pole_coord = ((p_i % self.width) as f64, (p_i / self.width) as f64);

                match pole {
                    Location::Asteroid(None) => {
                        let mut others: HashSet<i64> = HashSet::new();
                        let mut coords: Vec<CoordToOther> = vec![];

                        for (a_i, &ast) in self.locations.iter().enumerate() {
                            if a_i == p_i {
                                continue;
                            }

                            if let Location::Asteroid(_) = ast {
                                let ast_coord = ((a_i % self.width) as f64, (a_i / self.width) as f64);

                                let (length, angle) = calc_polar_coord(pole_coord, ast_coord);

                                let key = (angle * 100.0) as i64;
                                others.insert(key);

                                coords.push(CoordToOther {
                                    x: ast_coord.0,
                                    y: ast_coord.1,
                                    radial: length,
                                    angular: key,
                                });
                            }
                        }

                        angle_lookup.insert((pole_coord.0 as i64, pole_coord.1 as i64), coords);

                        Location::Asteroid(Some(others.len()))
                    },
                    _ => *pole,
                }
            })
            .collect();

        Map::new(&new_locations, self.width, self.height, &angle_lookup)
    }

    pub fn get_highest_score(&self) -> Option<(i64, i64, usize)> {
        let mut highest: Option<(i64, i64, usize)> = None;

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

    pub fn lasers(&self) -> Option<Vec<CoordToOther>> {
        let (x, y, _score) = self.get_highest_score()?;
        let coords = self.lookup_angles(x, y)?;

        let ordered_hits = sort_coords_by_laser_hit(&coords);

        ordered_hits
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut gfx = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let loc = self.get_loc(x, y);

                if let Some(Location::Asteroid(Some(h))) = loc {
                    gfx.push(match h {
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

    Map::new(&locs, width, height, &HashMap::new())
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Map) -> usize {
    input.process().get_highest_score().unwrap().2
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Map) -> i64 {
    let hits = input.process().lasers().unwrap();

    let two_hundredth = hits.get(199).unwrap();

    return ((two_hundredth.x * 100.0) + two_hundredth.y) as i64
}

#[cfg(test)]
mod tests {
    use crate::day10::*;
    use std::collections::HashMap;

    #[test]
    fn test_process_small() {
        let input = ".#..#\n\
                           .....\n\
                           #####\n\
                           ....#\n\
                           ...##";
        let map = input_generator(input).process();

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

        let hits = map.lasers().unwrap();

        for h in &hits {
            println!("{:?}", h);
        }

        let hit_1 = hits.get(0).unwrap();
        assert_eq!((hit_1.x as u64, hit_1.y as u64), (11, 12));

        let hit_2 = hits.get(1).unwrap();
        assert_eq!((hit_2.x as u64, hit_2.y as u64), (12, 1));

        let hit_3 = hits.get(2).unwrap();
        assert_eq!((hit_3.x as u64, hit_3.y as u64), (12, 2));

        let hit_10 = hits.get(9).unwrap();
        assert_eq!((hit_10.x as u64, hit_10.y as u64), (12, 8));

        let hit_20 = hits.get(19).unwrap();
        assert_eq!((hit_20.x as u64, hit_20.y as u64), (16, 0));

        let hit_50 = hits.get(49).unwrap();
        assert_eq!((hit_50.x as u64, hit_50.y as u64), (16, 9));

        let hit_100 = hits.get(99).unwrap();
        assert_eq!((hit_100.x as u64, hit_100.y as u64), (10, 16));

        let hit_199 = hits.get(198).unwrap();
        assert_eq!((hit_199.x as u64, hit_199.y as u64), (9, 6));

        let hit_200 = hits.get(199).unwrap();
        assert_eq!((hit_200.x as u64, hit_200.y as u64), (8, 2));

        let hit_201 = hits.get(200).unwrap();
        assert_eq!((hit_201.x as u64, hit_201.y as u64), (10, 9));

        let hit_299 = hits.get(298).unwrap();
        assert_eq!((hit_299.x as u64, hit_299.y as u64), (11, 1));
    }
}