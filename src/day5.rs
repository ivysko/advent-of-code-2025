use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt;

pub const SMALL_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[derive(Clone, Hash)]
struct Range {
    start: usize,
    end: usize,
}

impl fmt::Display for Range {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Range {}

impl Range {
    pub fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }

    pub fn import(input: &str) -> Range {
        // number dash number
        let split_parts = input
            .split("-")
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>();
        let start = split_parts.first().unwrap();
        let end = split_parts.last().unwrap();

        Range::new(*start, *end)
    }

    pub fn is_in_range(&self, number: usize) -> bool {
        // range's start and end are inclusive
        number <= self.end && number >= self.start
    }

    pub fn is_overlapping(&self, other: &Range) -> bool {
        max(self.start, other.start) <= min(self.end, other.end)
    }

    pub fn overlap_range(&self, other: &Range) -> Range {
        Range::new(other.start.min(self.start), other.end.max(self.end))
    }
}

fn common(input: &str) -> (HashSet<Range>, Vec<usize>) {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    let fresh_ranges = split_input
        .first()
        .unwrap()
        .lines()
        .map(Range::import)
        .collect::<HashSet<Range>>();

    let available_ingredients = split_input
        .last()
        .unwrap()
        .lines()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();

    (fresh_ranges, available_ingredients)
}

pub fn part1(input: &str) -> String {
    let (fresh_ranges, available_ingredients) = common(input);

    let mut fresh_counter = 0;

    for ingredient in available_ingredients {
        for range in &fresh_ranges {
            if range.is_in_range(ingredient) {
                fresh_counter += 1;
                break;
            }
        }
    }

    format!("fresh ingredients: {}", fresh_counter)
}

fn at_least_one_overlap(ranges: HashSet<Range>) -> bool {
    for range_a in &ranges {
        for range_b in &ranges {
            if range_a == range_b {
                continue;
            }
            if range_a.is_overlapping(range_b) || range_b.is_overlapping(range_a) {
                return true;
            }
        }
    }
    false
}

fn calculate_overlaps(ranges: HashSet<Range>) -> HashSet<Range> {
    let mut result = HashSet::new();
    for range_a in &ranges {
        let mut range_a_inserted = false;
        for range_b in &ranges {
            if range_a == range_b {
                continue;
            }
            if range_a.is_overlapping(range_b) {
                result.insert(Range::overlap_range(range_a, range_b));
                range_a_inserted = true;
            }
        }

        if !range_a_inserted {
            result.insert(range_a.clone());
        }
    }
    result
}

pub fn part2(input: &str) -> String {
    let (mut fresh_ranges, _) = common(input);

    // check if a range overlaps with another
    // if so, new range = largest range possible

    while at_least_one_overlap(fresh_ranges.clone()) {
        fresh_ranges = calculate_overlaps(fresh_ranges);
    }

    let mut fresh_ingredients = 0;

    for range in fresh_ranges {
        fresh_ingredients += range.end - range.start + 1;
    }

    format!("fresh ingredients: {}", fresh_ingredients)
}
