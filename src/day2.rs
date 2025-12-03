use std::cmp::PartialEq;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

pub const SMALL_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn common(input: &str) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();

    for line in input.split(",") {
        let numbers = line.split("-").collect::<Vec<&str>>();

        let start_number = numbers[0].parse::<usize>().unwrap();
        let end_number = numbers[1].parse::<usize>().unwrap();

        let range = Range {
            start: start_number,
            end: end_number,
        };
        ranges.push(range);
    }

    ranges
}

pub fn part1(input: &str) -> String {
    let ranges = common(input);

    let mut invalid_ids: Vec<usize> = Vec::new();

    for range in ranges.iter() {
        invalid_ids.append(&mut list_invalid_ids_p1(range));
    }

    let sum: usize = invalid_ids.iter().sum();

    format!("sum: {sum}")
}

fn list_invalid_ids_p1(range: &Range) -> Vec<usize> {
    let mut invalid_ids: Vec<usize> = Vec::new();

    for i in range.start..range.end + 1 {
        if is_invalid_id_p1(i) {
            invalid_ids.push(i);
        }
    }

    invalid_ids
}

fn is_invalid_id_p1(id: usize) -> bool {
    let string_id = id.to_string();

    if string_id.len() % 2 != 0 {
        false
    } else {
        let halves = string_id.split_at(string_id.len() / 2);

        halves.0.eq(halves.1)
    }
}

pub fn part2(input: &str) -> String {
    let ranges = common(input);

    let mut invalid_ids: Vec<usize> = Vec::new();

    for range in ranges.iter() {
        invalid_ids.append(&mut list_invalid_ids_p2(range));
    }

    let sum: usize = invalid_ids.iter().sum();

    format!("sum: {sum}")
}

fn list_invalid_ids_p2(range: &Range) -> Vec<usize> {
    let mut invalid_ids: Vec<usize> = Vec::new();

    for i in range.start..range.end + 1 {
        if is_invalid_id_p2(i) {
            invalid_ids.push(i);
        }
    }

    invalid_ids
}

fn is_invalid_id_p2(id: usize) -> bool {
    let string_id = id.to_string();

    // create a vector of substrings: 0 to 0, 0 to 1, 0 to 2, …, 0 to x; x being len(string_id)/2
    // check if string_id is made of these substrings repetitively

    let mut substrings: Vec<&str> = Vec::new();

    for i in 0..(string_id.len() / 2) {
        substrings.push(&string_id[0..i + 1]);
    }

    for substring in substrings {
        let substring_size = substring.len();
        let string_id_size = &string_id.len();

        let supposed_string_id = substring.repeat(string_id_size / substring_size);

        if supposed_string_id.eq(&string_id) {
            return true;
        }
    }

    false
}
