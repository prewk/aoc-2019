use std::collections::HashMap;
use intcode::{get_instruction, InstrType};
use intcode::instructions::{ProgramErr, Program, Instruction};
use failure::_core::fmt::{Formatter, Error};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    Black,
    White,
}

fn color_to_int(c: Color) -> i64 {
    match c {
        Color::Black => 0,
        Color::White => 1,
    }
}

fn int_to_color(i: i64) -> Color {
    match i {
        0 => Color::Black,
        1 => Color::White,
        _ => panic!("Invalid color"),
    }
}

type Coords = (i64, i64);

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Angle {
    Left,
    Right,
}

fn int_to_angle(i: i64) -> Angle {
    match i {
        0 => Angle::Left,
        1 => Angle::Right,
        _ => panic!("Invalid angle state"),
    }
}

fn int_to_direction(prev: Direction, i: i64) -> Direction {
    match (prev, int_to_angle(i)) {
        (Direction::Up, Angle::Left) => Direction::Left,
        (Direction::Up, Angle::Right) => Direction::Right,
        (Direction::Right, Angle::Left) => Direction::Up,
        (Direction::Right, Angle::Right) => Direction::Down,
        (Direction::Down, Angle::Left) => Direction::Right,
        (Direction::Down, Angle::Right) => Direction::Left,
        (Direction::Left, Angle::Left) => Direction::Down,
        (Direction::Left, Angle::Right) => Direction::Up,
    }
}

fn move_robot(current: Coords, dir: Direction) -> Coords {
    match dir {
        Direction::Up => (current.0, current.1 + 1),
        Direction::Right => (current.0 + 1, current.1),
        Direction::Down => (current.0, current.1 - 1),
        Direction::Left => (current.0 - 1, current.1),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ship {
    panels: HashMap<Coords, Color>,
    robot_dir: Direction,
    robot_pos: Coords,
}

impl Ship {
    pub fn new(panels: &HashMap<Coords, Color>, robot_dir: Direction, robot_pos: Coords) -> Ship {
        Ship {
            panels: panels.clone(),
            robot_dir,
            robot_pos,
        }
    }

    pub fn get_robot_pos(&self) -> Coords { self.robot_pos }
    pub fn get_robot_dir(&self) -> Direction { self.robot_dir }
    pub fn get_painted_panel_cnt(&self) -> usize { self.panels.len() }

    fn paint_panel_and_set_direction(&self, color: Color, dir: Direction) -> Ship {
        let mut panels = self.panels.clone();
        panels.insert(self.robot_pos, color);

        Ship {
            panels,
            robot_dir: dir,
            robot_pos: move_robot(self.robot_pos, dir),
        }
    }

    pub fn get_color(&self, coords: Coords) -> Color {
        *self.panels.get(&coords).unwrap_or(&Color::Black)
    }
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut min_x: Option<i64> = None;
        let mut max_x: Option<i64> = None;
        let mut min_y: Option<i64> = None;
        let mut max_y: Option<i64> = None;

        for ((x, y), _color) in &self.panels {
            if let None = min_x {
                min_x = Some(*x);
            } else if let Some(cmp_x) = min_x {
                if *x < cmp_x { min_x = Some(*x); }
            }

            if let None = max_x {
                max_x = Some(*x);
            } else if let Some(cmp_x) = max_x {
                if *x > cmp_x { max_x = Some(*x); }
            }

            if let None = min_y {
                min_y = Some(*y);
            } else if let Some(cmp_y) = min_y {
                if *y < cmp_y { min_y = Some(*y); }
            }

            if let None = max_y {
                max_y = Some(*y)
            } else if let Some(cmp_y) = max_y {
                if *y > cmp_y { max_y = Some(*y); }
            }
        }

        let mut gfx = String::new();

        for y in min_y.unwrap()..max_y.unwrap() {
            for x in min_x.unwrap()..max_x.unwrap() {
                gfx.push(match self.panels.get(&(x, y)) {
                    None => '.',
                    Some(Color::Black) => '.',
                    Some(Color::White) => '#',
                });
            }
            gfx.push('\n');
        }

        write!(f, "{}", gfx)
    }
}

pub fn run(program_ints: &Vec<i64>) -> Result<Ship, ProgramErr> {
    let mut ship = Ship::new(&HashMap::new(), Direction::Up, (0, 0));

    let mut program = Program::new(
        program_ints.clone(),
        0,
        vec![],
        vec![color_to_int(ship.get_color((0, 0)))],
        0
    );

    let mut col_dir: (Option<i64>, Option<i64>) = (None, None);

    loop {
        program = match get_instruction(&program)? {
            InstrType::Add(instr) => instr.run(program)?,
            InstrType::Mul(instr) => instr.run(program)?,
            InstrType::Input(instr) => instr.run(program)?,
            InstrType::Output(instr) => {
                let mut next_program = instr.run(program)?;
                let out = next_program.outputs();
                let last = out.last().expect("Expected output missing");

                col_dir = match col_dir {
                    (None, None) => (Some(*last), None),
                    (Some(c), None) => (Some(c), Some(*last)),
                    (Some(_c), Some(_d)) => (Some(*last), None),
                    _ => panic!("Invalid state"),
                };

                if let (Some(c), Some(d)) = col_dir {
                    ship = ship.paint_panel_and_set_direction(
                        int_to_color(c),
                        int_to_direction(ship.get_robot_dir(), d)
                    );

                    next_program = next_program.push_input(
                        color_to_int(
                            ship.get_color(ship.robot_pos)
                        )
                    );
                }

                next_program
            },
            InstrType::JmpIfFalse(instr) => instr.run(program)?,
            InstrType::JmpIfTrue(instr) => instr.run(program)?,
            InstrType::LessThan(instr) => instr.run(program)?,
            InstrType::Equals(instr) => instr.run(program)?,
            InstrType::Exit(instr) => instr.run(program)?,
            InstrType::RelBaseOffset(instr) => instr.run(program)?,
        };

        if program.has_exited() {
            return Ok(ship)
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let ship = run(&input.to_vec()).unwrap();

    ship.get_painted_panel_cnt()
}
