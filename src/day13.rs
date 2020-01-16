use intcode::instructions::{Program, ProgramErr, Instruction};
use intcode::{InstrType, get_instruction};
use std::collections::HashMap;
use std::fmt;
use failure::_core::fmt::{Formatter, Error};
use std::io::{Write, stdout};
use termion::cursor;
use termion::raw::IntoRawMode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Block {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub fn int_to_block(i: i64) -> Option<Block> {
    match i {
        0 => Some(Block::Empty),
        1 => Some(Block::Wall),
        2 => Some(Block::Block),
        3 => Some(Block::Paddle),
        4 => Some(Block::Ball),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Screen {
    pixels: HashMap<(usize, usize), Block>,
}

pub fn print_screen(scr: &Screen) -> String {
    let mut gfx = String::new();
    gfx.push('\n');

    let px = scr.pixels();

    let max_x = px.iter().fold(0, |max, ((x, _), _)| {
        if *x > max { *x } else { max }
    });

    let max_y = px.iter().fold(0, |max, ((_, y), _)| {
        if *y > max { *y } else { max }
    });

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            gfx.push(match px.get(&(x, y)) {
                Some(Block::Empty) => ' ',
                Some(Block::Wall) => '#',
                Some(Block::Block) => ':',
                Some(Block::Paddle) => '=',
                Some(Block::Ball) => 'o',
                None => '?',
            });
        }
        gfx.push('\n');
    }

    gfx
}

pub fn print_screen_with_termion(scr: &Screen) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let px = scr.pixels();

    let max_x = px.iter().fold(0, |max, ((x, _), _)| {
        if *x > max { *x } else { max }
    });

    let max_y = px.iter().fold(0, |max, ((_, y), _)| {
        if *y > max { *y } else { max }
    });

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            let char = match px.get(&(x, y)) {
                Some(Block::Empty) => ' ',
                Some(Block::Wall) => '#',
                Some(Block::Block) => ':',
                Some(Block::Paddle) => '=',
                Some(Block::Ball) => 'o',
                None => '?',
            };

            write!(stdout, "{}{}", termion::cursor::Goto(x as u16, y as u16), char).unwrap();
        }
    }
}

impl Screen {
    pub fn new() -> Screen {
        Screen { pixels: HashMap::new() }
    }

    pub fn pixels(&self) -> &HashMap<(usize, usize), Block> { &self.pixels }

    pub fn set_block(&self, x: usize, y: usize, b: Block) -> Screen {
        let mut pixels = self.pixels.clone();

        pixels.insert((x, y), b);

        Screen { pixels }
    }

    pub fn num_of(&self, b: Block) -> usize {
        self.pixels.iter()
            .filter(|(_, &i)| i == b)
            .count()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let gfx = print_screen(&self);

        write!(f, "{}", gfx)
    }
}

pub fn run(program_ints: &Vec<i64>) -> Result<Screen, ProgramErr> {
    let mut program = Program::new(program_ints.clone(), 0, vec![], vec![], 0);
    let mut screen = Screen::new();
    let mut block_instr: (Option<usize>, Option<usize>, Option<Block>) = (None, None, None);

    loop {
        program = match get_instruction(&program)? {
            InstrType::Add(instr) => instr.run(program)?,
            InstrType::Mul(instr) => instr.run(program)?,
            InstrType::Input(instr) => instr.run(program)?,
            InstrType::Output(instr) => {
                let next = instr.run(program)?;
                let output = *next.outputs().last().expect("No last");

                block_instr = match block_instr {
                    (None, None, None) => (Some(output as usize), None, None),
                    (Some(_), Some(_), Some(_)) => (Some(output as usize), None, None),
                    (Some(x), None, None) => (Some(x), Some(output as usize), None),
                    (Some(x), Some(y), None) => (Some(x), Some(y), Some(int_to_block(output).expect("Invalid block"))),
                    _ => panic!("Invalid state"),
                };

                if let (Some(x), Some(y), Some(b)) = block_instr {
                    screen = screen.set_block(x, y, b);
//                    print_screen_with_termion(&screen);
                }

                next
            },
            InstrType::JmpIfFalse(instr) => instr.run(program)?,
            InstrType::JmpIfTrue(instr) => instr.run(program)?,
            InstrType::LessThan(instr) => instr.run(program)?,
            InstrType::Equals(instr) => instr.run(program)?,
            InstrType::Exit(instr) => instr.run(program)?,
            InstrType::RelBaseOffset(instr) => instr.run(program)?,
        };

        if program.has_exited() {
            return Ok(screen)
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let screen = run(&input.to_vec()).unwrap();

    screen.num_of(Block::Block)
}