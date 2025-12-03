mod day1;
mod day2;

use std::fs;
use crate::day2::{part1, part2, SMALL_INPUT};

fn main() {
    println!("Advent of Code 2025\n\n");

    let use_small_input = false;

    if use_small_input {
        println!("Use small input.");

        println!("part1: {}", part1(SMALL_INPUT));
        println!("part2: {}", part2(SMALL_INPUT));
    }
    else {
        let content = fs::read_to_string("input/input-day2.txt").expect("Should have been able to read the file").trim().to_string();

        println!("part1: {}", part1(&content));
        println!("part2: {}", part2(&content));
    }





}
