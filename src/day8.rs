use crate::advent_image::{ImageErr, Image};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).ok_or(ImageErr::ParseError))
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let image = Image::new_from_ints(25, 6, &input.to_vec()).unwrap();

    image.day08a_challenge().ok_or(ImageErr::ParseError).unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[u32]) -> String {
    let image = Image::new_from_ints(25, 6, &input.to_vec()).unwrap();

    let res = image.day08b_challenge().unwrap();
    let mut out = String::with_capacity(25 * 6 + 1);

    for row in res.iter(){
        out.push('\n');
        for col in row.iter(){
            match col {
                crate::advent_image::Pixel::Black => {
                    out.push(' ');
                },
                crate::advent_image::Pixel::Transparent => {
                    out.push('.');
                },
                crate::advent_image::Pixel::White => {
                    out.push('#');
                },
            };
        }
    }

    out.to_string()
}