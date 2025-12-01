use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(character: char) -> Direction {
        match character {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        Instruction {
            direction: Direction::from(line.chars().next().unwrap()),
            value: line[1..].parse().unwrap(),
        }
    }

    fn apply_to(&self, value: i32) -> i32 {
        match self.direction {
            Direction::Left => value - self.value,
            Direction::Right => value + self.value,
        }
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());

    start = Instant::now();
    println!("{} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(Instruction::from)
        .fold((0, 50), |(mut total, mut position), instruction| {
            total += count_times_at_zero(&mut position, &instruction);
            (total, position)
        })
        .0
}

fn count_times_at_zero(position: &mut i32, instruction: &Instruction) -> i32 {
    *position = instruction.apply_to(*position);
    i32::from(*position % 100 == 0)
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(Instruction::from)
        .fold((0, 50), |(mut total, mut position), instruction| {
            total += count_times_past_zero(&mut position, &instruction);
            (total, position)
        })
        .0
}

fn count_times_past_zero(position: &mut i32, instruction: &Instruction) -> i32 {
    let new_position = instruction.apply_to(*position);
    let full_cycles = (new_position / 100).abs();

    let crossed_zero =  i32::from(new_position > 0 && *position < 0 || new_position < 0 && *position > 0 || new_position == 0);
    let remainder = new_position % 100;

    *position = if remainder < 0 {100 + remainder} else {remainder};

    full_cycles + crossed_zero
}

#[test]
fn small_input() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    assert_eq!(part_one(input), 3);
    assert_eq!(part_two(input), 6);
}
