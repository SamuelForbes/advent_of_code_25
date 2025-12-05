use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());

    start = Instant::now();
    println!("{} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u32 {
    input.lines().map(calculate_joltage).sum()
}

fn calculate_joltage(input: &str) -> u32 {
    let values: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let first_max = values[0..values.len() - 1].iter().max().unwrap();
    let max_position = values.iter().position(|x| x == first_max).unwrap();
    let second_max = values[max_position + 1..].iter().max().unwrap();

    format!("{first_max}{second_max}").parse().unwrap()
}

fn part_two(input: &str) -> u64 {
    input.lines().map(calculate_larger_joltage).sum()
}

fn calculate_larger_joltage(input: &str) -> u64 {
    let mut values: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let lead_digit = values[0..values.len() - 12].iter().max().unwrap();
    let lead_digit_position = values.iter().position(|x| x == lead_digit).unwrap();

    while values.len() > 12 {
        let min = values.iter().min().unwrap();
        let min_position = values.iter().position(|x| x == min).unwrap();
        values.remove(min_position);
    }

    println!("{input}, {lead_digit_position}, {values:?}");

    values
        .iter()
        .map(ToString::to_string)
        .collect::<String>()
        .parse()
        .unwrap()
}

#[test]
fn small_input() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";

    assert_eq!(357, part_one(input));
    assert_eq!(3_121_910_778_619, part_two(input));
}
