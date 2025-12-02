use std::fmt::format;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());

     // start = Instant::now();
    // println!("{} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u64 {
    input.split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split(['-','\n']).collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .fold(0, add_double_values)
}

fn add_double_values(mut accumulator: u64, (range_start, range_end): (u64, u64)) -> u64 {
    let range_start_string = range_start.to_string();
    let index_limit = if range_start_string.len() > 1 { range_start_string.len() / 2 } else { 1 };
    let mut start_first_half =  range_start_string[0..index_limit].parse::<u64>().unwrap();

    loop {
        let double = format!("{start_first_half}{start_first_half}").parse::<u64>().unwrap();
        println!("{},{},{}, {}", range_start, range_end, double, double >= range_start && double <= range_end);

        if double >= range_start && double <= range_end {
            accumulator += double;

        } else if double > range_end {
            break;
        }

        start_first_half+= 1;
    }

    accumulator
}

#[test]
fn small_input(){
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(part_one(input), 1_227_775_554);
}