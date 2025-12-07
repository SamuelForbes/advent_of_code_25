use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());

    start = Instant::now();
    println!("{} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u32 {
    let (ranges, values) = parse_input(input);
    let mut counter = 0;
    for value in values {
        'ranges: for range in &ranges {
            if range.contains(&value) {
                counter += 1;
                break 'ranges;
            }
        }
    }

    counter
}

fn part_two(input: &str) -> u64 {
    let mut ranges = parse_input(input).0;
    let mut unique_ranges: Vec<RangeInclusive<u64>> = vec![ranges[0].clone()];

    'outer: for i in 0..ranges.len() {
        let mut range = ranges[i].clone();
        let mut j = 0;

        while j < unique_ranges.len() {
            if range_contained(&unique_ranges[j], &ranges[i]) {
                continue 'outer;
            } else if range_intersects(&unique_ranges[j], &ranges[i]) {
                let start = if unique_ranges[j].start() < range.start() {
                    unique_ranges[j].start()
                } else {
                    range.start()
                };

                let end = if unique_ranges[j].end() > range.end() {
                    unique_ranges[j].end()
                } else {
                    range.end()
                };

                range = *start..=*end;
                unique_ranges.remove(j);
            }
            j += 1;
        }

        unique_ranges.push(range);
    }

    unique_ranges
        .iter()
        .map(|range| range.end() - range.start())
        .sum()
}

fn range_contained(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> bool {
    range1.start() > range2.start() && range1.end() < range2.end()
}

fn range_intersects(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> bool {
    range1.start() <= range2.start() && range1.end() >= range2.start() ||
        range1.start() <= range2.end() && range1.end() >= range2.end()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let parts = input.split_once("\n\n").unwrap();

    let ranges = parts
        .0
        .lines()
        .map(|line| {
            let line_parts = line.split_once('-').unwrap();
            return line_parts.0.parse().unwrap()..=line_parts.1.parse().unwrap();
        })
        .collect();

    let values = parts
        .1
        .lines()
        .map(|value| value.parse().unwrap())
        .collect();

    (ranges, values)
}

#[test]
fn small_input() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(3, part_one(input));
    assert_eq!(14, part_two(input));
    assert!(range_intersects(&(2..=10), &(5..=15)));
    assert!(range_intersects(&(7..=16), &(5..=15)));
    assert!(range_intersects(&(2..=16), &(5..=15)));
}
