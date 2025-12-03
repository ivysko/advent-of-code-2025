use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    clicks: i32,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dir: {:?}, clicks: {}", self.dir, self.clicks)
    }
}

pub const SMALL_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn common(input: &str) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation> = Vec::new();

    for line in input.lines() {
        let mut rotation: Rotation = Rotation {
            dir: Direction::Right,
            clicks: 0,
        };
        let mut nb_string = "".to_owned();

        for character in line.chars() {
            if !character.is_digit(10) {
                if character.eq(&'R') {
                    rotation.dir = Direction::Right;
                } else if character.eq(&'L') {
                    rotation.dir = Direction::Left;
                }
            } else {
                nb_string.push(character);
            }
        }

        let number = nb_string.parse::<i32>().unwrap();
        rotation.clicks = number;

        rotations.push(rotation);
    }

    rotations
}

pub fn part1(input: &str) -> String {
    // value starts at 50
    // cpt starts at 0
    // each line:
    //      - Ldigits = remove digits to value
    //      - Rdigits = add digits to value
    //      if negative: note for later
    //      value = value % 100
    //      if negative: value = 100-value
    //      if value==0: cpt += 1

    let rotations = common(input);

    let mut cpt: usize = 0;
    let mut value: i32 = 50;

    for rotation in rotations {
        if rotation.dir == Direction::Left {
            value -= rotation.clicks;
        } else {
            value += rotation.clicks;
        }

        if value.is_negative() {
            value = (100 - (value.abs() % 100)) % 100;
        } else {
            value = value % 100;
        }

        if value == 0 {
            cpt += 1;
        }
    }

    format!("cpt: {}", cpt)
}

pub fn part2(input: &str) -> String {
    let rotations = common(input);

    format!("cpt: {}", part2_loops(rotations))
}

// fixed with the help of github.com/maneatingape code
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day01.rs
fn part2_maths(rotations: Vec<Rotation>) -> i32 {
    let mut cpt: i32 = 0;
    let mut dial_value: i32 = 50;

    for rotation in rotations {
        if rotation.dir == Direction::Left {
            let reversed = (100 - dial_value) % 100;
            cpt += (reversed + rotation.clicks).div_euclid(100);

            dial_value -= rotation.clicks;
        } else {
            cpt += (dial_value + rotation.clicks).div_euclid(100);

            dial_value += rotation.clicks;
        }

        dial_value = dial_value.rem_euclid(100);
    }

    cpt
}

fn part2_loops(rotations: Vec<Rotation>) -> usize {
    let mut cpt: usize = 0;
    let mut value: i32 = 50;

    for rotation in rotations {
        let mut direction = 1;
        if rotation.dir == Direction::Left {
            direction *= -1;
        }

        for _i in 0..rotation.clicks {
            value += 1 * direction;

            if value == 100 {
                value = 0;
            } else if value == -1 {
                value = 99;
            }

            if value == 0 {
                cpt += 1;
            }
        }
    }

    cpt
}
