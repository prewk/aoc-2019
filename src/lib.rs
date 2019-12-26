extern crate aoc_runner;
#[macro_use] extern crate failure;
extern crate geo;
extern crate line_intersection;
extern crate image;
#[macro_use] extern crate aoc_runner_derive;
extern crate voca_rs;

pub mod intcode;
pub mod advent_image;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

aoc_lib!{ year = 2019 }