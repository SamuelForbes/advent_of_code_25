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
    let values: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();

    find_positions(&values)
        .iter()
        .map(|i| values[*i].to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn find_positions(values: &[u32]) -> Vec<usize> {
    let mut positions = vec![];
    let mut highest_digit = *values.iter().max().unwrap();
    let mut window = 0..values.len();

    while positions.len() < 12 {
        println!("{window:?}");
        let potential_positions: Vec<usize> = values
            .iter()
            .enumerate()
            .filter(|(i, _)| window.contains(i))
            .filter(|(_, x)| **x == highest_digit)
            .map(|(i, _)| i)
            .collect();

        for position in potential_positions.iter().rev() {
            if positions.len() < 12 && !positions.contains(position) {
                positions.push(*position);
            }
        }

        let last_position = positions.last().unwrap();
        println!("{},{},{}", values.len(), positions.len(), last_position);
        if values.len() - *last_position > 12 - positions.len() {
            window = *last_position..values.len();
        }

        positions.sort_unstable();
        println!("{positions:?}");

        highest_digit -= 1;
    }

    positions
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

#[test]
fn edge_case() {
    let input = "999999999911897";
    assert_eq!(999_999_999_997, calculate_larger_joltage(input));
}
