pub const SMALL_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";


pub fn part1(input: &str) -> String {
    let mut largest_joltages: Vec<u32> = Vec::new();
    for bank in input.lines() {
        let mut batteries = bank.chars();

        // remove last one
        let last = batteries.next_back();

        let batteries_but_last_vec = batteries.collect::<Vec<char>>();
        let batteries_but_last_vec_uint = batteries_but_last_vec.iter().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let mut complete_batteries = batteries_but_last_vec_uint.clone();

        // put the last one back
        let last_uint = last.unwrap().to_digit(10).unwrap();
        complete_batteries.push(last_uint);


        let (max, index) = get_first_max_with_index(&batteries_but_last_vec_uint);

        let second_max = complete_batteries[index+1..].iter().max().unwrap();

        let largest_joltage = max*10+second_max;
        largest_joltages.push(largest_joltage);
    }

    format!("sum: {}", largest_joltages.iter().sum::<u32>())
}

pub fn part2(input: &str) -> String {
    let mut largest_joltages: Vec<usize> = Vec::new();
    for bank in input.lines() {
        let batteries = bank.chars();

        let batteries_vec = batteries.collect::<Vec<char>>();
        let batteries_vec_uint = batteries_vec.iter().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let mut start_pos = 0;
        let mut largest_joltage = "".to_string();

        for i in (0..12).rev() {
            let substring = batteries_vec_uint.get(start_pos..(batteries_vec_uint.len()-i)).unwrap();

            let (max, index) = get_first_max_with_index(&substring.to_vec());

            start_pos = start_pos+index+1;

            largest_joltage.push(max.to_string().parse().unwrap());
        }

        largest_joltages.push(largest_joltage.parse::<usize>().unwrap());
    }

    format!("sum: {}", largest_joltages.iter().sum::<usize>())
}

fn get_first_max_with_index(nums: &Vec<u32>) -> (u32, usize) {
    let max = nums
            .iter()
            .max()
            .unwrap();

    // get the index of the first max value
    let first_max_index = nums
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value.eq(max))
        .next()
        .map(|(index, _)| index)
        .unwrap();

    (*max, first_max_index)
}
