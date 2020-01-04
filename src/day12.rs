use voca_rs::*;
use std::cmp::Ordering;
use std::collections::{HashMap};

type Position = (i64, i64, i64);
type Velocity = (i64, i64, i64);

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Moon {
    pos: Position,
    vel: Velocity,
}

/// Is a's position less/greater/equal to b's?
///
/// ```
/// use aoc_2019::day12::{cmp_gravity, Moon};
/// use std::cmp::Ordering;
///
/// let ord = cmp_gravity(
///     &Moon::new(0, 0, 0),
///     &Moon::new(1, -1, 0),
/// );
///
/// assert_eq!(ord, (Ordering::Less, Ordering::Greater, Ordering::Equal));
/// ```
pub fn cmp_gravity(a: &Moon, b: &Moon) -> (Ordering, Ordering, Ordering) {
    let a_pos = a.position();
    let b_pos = b.position();

    (
        a_pos.0.partial_cmp(&b_pos.0).unwrap_or(Ordering::Equal),
        a_pos.1.partial_cmp(&b_pos.1).unwrap_or(Ordering::Equal),
        a_pos.2.partial_cmp(&b_pos.2).unwrap_or(Ordering::Equal),
    )
}


/// ```
/// use aoc_2019::day12::str_coord_pair_to_int;
///
/// let coords = vec![
///     "x= 123".to_string(),
///     " y=-123".to_string(),
///     " z= 0 ".to_string(),
/// ];
///
/// let x = str_coord_pair_to_int(&coords, "x");
/// let y = str_coord_pair_to_int(&coords, "y");
/// let z = str_coord_pair_to_int(&coords, "z");
///
/// assert_eq!((x, y, z), (123, -123, 0));
/// ```
pub fn str_coord_pair_to_int(coords: &Vec<String>, expected: &str) -> i64 {
    coords.iter()
        .find(|coord|
            manipulate::trim(coord, "").starts_with(expected)
        )
        .expect(format!("Missing {} coord", expected).as_ref())
        .split('=')
        .map(|part| manipulate::trim(part, ""))
        .collect::<Vec<String>>()
        .get(1)
        .expect(format!("Missing {} coord value", expected).as_ref())
        .parse::<i64>()
        .expect(format!("Invalid {} coord value", expected).as_ref())
}

pub fn cmp_moons(input: Vec<Moon>) -> Vec<Moon> {
    let mut lookup: HashMap<usize, Moon> = HashMap::new();
    for (i, m) in input.iter().enumerate() {
        lookup.insert(i, m.clone());
    }

    for a in 0..lookup.len() {
        for b in 0..lookup.len() {
            if a == b {
                continue;
            }

            let a_m = lookup.get(&a).unwrap();
            let b_m = lookup.get(&b).unwrap();

            let gravvied = a_m.apply_gravity(b_m);

            lookup.insert(a, gravvied);
        }
    }

    let mut velocited = vec![];

    for i in 0..lookup.len() {
        let m = lookup.get(&i).unwrap();
        velocited.push(m.apply_velocity());
    }

    velocited
}

pub fn calc_total_energy(moons: &Vec<Moon>) -> i64 {
    moons.iter().fold(0, |total, moon| { total + moon.energy() })
}

impl Moon {
    pub fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon { pos: (x, y, z), vel: (0, 0, 0) }
    }

    pub fn position(&self) -> Position { self.pos }

    pub fn velocity(&self) -> Velocity { self.vel }

    pub fn apply_gravity(&self, against: &Moon) -> Moon {
        let (x, y, z) = cmp_gravity(self, against);

        Moon { pos: self.pos, vel: (
            match x {
                Ordering::Less => self.vel.0 + 1,
                Ordering::Equal => self.vel.0,
                Ordering::Greater => self.vel.0 - 1,
            },
            match y {
                Ordering::Less => self.vel.1 + 1,
                Ordering::Equal => self.vel.1,
                Ordering::Greater => self.vel.1 - 1,
            },
            match z {
                Ordering::Less => self.vel.2 + 1,
                Ordering::Equal => self.vel.2,
                Ordering::Greater => self.vel.2 - 1,
            },
        ) }
    }

    pub fn apply_velocity(&self) -> Moon {
        Moon {
            pos: (
                self.pos.0 + self.vel.0,
                self.pos.1 + self.vel.1,
                self.pos.2 + self.vel.2,
            ),
            vel: self.vel,
        }
    }

    pub fn energy(&self) -> i64 {
        (self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()) *
        (self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs())
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let coords = manipulate::trim(&line, "<>")
                .split(',')
                .map(|coord| manipulate::trim(coord, ""))
                .collect::<Vec<String>>();

            Moon::new(
                str_coord_pair_to_int(&coords, "x"),
                str_coord_pair_to_int(&coords, "y"),
                str_coord_pair_to_int(&coords, "z")
            )
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();

    for _ in 0..1000 {
        moons = cmp_moons(moons);
    }

    calc_total_energy(&moons)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_moons() {
        let mut moons = vec![
             Moon::new(-1, 0, 2),
             Moon::new(2, -10, -7),
             Moon::new(4, -8, 8),
             Moon::new(3, 5, -1),
        ];
        let mut turns = vec![];

        for i in 0..10 {
            moons = cmp_moons(moons);
            turns.push(moons.clone());
        }

        let turn_1 = (
            *turns.get(0).unwrap().get(0).unwrap(),
            *turns.get(0).unwrap().get(1).unwrap(),
            *turns.get(0).unwrap().get(2).unwrap(),
            *turns.get(0).unwrap().get(3).unwrap(),
        );

        assert_eq!((turn_1.0.position(), turn_1.0.velocity()), ((2, -1, 1), (3, -1, -1)));
        assert_eq!((turn_1.1.position(), turn_1.1.velocity()), ((3, -7, -4), (1, 3, 3)));
        assert_eq!((turn_1.2.position(), turn_1.2.velocity()), ((1, -7, 5), (-3, 1, -3)));
        assert_eq!((turn_1.3.position(), turn_1.3.velocity()), ((2, 2, 0), (-1, -3, 1)));

        let turn_10 = (
            *turns.get(9).unwrap().get(0).unwrap(),
            *turns.get(9).unwrap().get(1).unwrap(),
            *turns.get(9).unwrap().get(2).unwrap(),
            *turns.get(9).unwrap().get(3).unwrap(),
        );

        assert_eq!((turn_10.0.position(), turn_10.0.velocity()), ((2, 1, -3), (-3, -2, 1)));
        assert_eq!((turn_10.1.position(), turn_10.1.velocity()), ((1, -8, 0), (-1, 1, 3)));
        assert_eq!((turn_10.2.position(), turn_10.2.velocity()), ((3, -6, 1), (3, 2, -3)));
        assert_eq!((turn_10.3.position(), turn_10.3.velocity()), ((2, 0, 4), (1, -1, -1)));
    }
}