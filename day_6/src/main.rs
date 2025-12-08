use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());
    //
    // start = Instant::now();
    // println!("{} - {:?}", part_two(&input), start.elapsed());
}

#[derive(PartialEq)]
enum Action {
    Add,
    Multiply
}

impl Action {
    fn from(c: char) -> Action {
        match c {
            '+' => Action::Add,
            '*' => Action::Multiply,
            _ => unreachable!("Unknown action {c}")
        }
    }
}
fn part_one(input: &str) -> u64 {
    let grid: Vec<Vec<&str>> = input.lines().map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>()).collect();
    let mut total = 0;

    for i in 0..grid[0].len() {
        let action = Action::from(grid.last().unwrap()[i].chars().next().unwrap());
        let mut column_total: u64 = grid[0][i].parse().unwrap();

        for j in 1..grid.len() - 1 {
            let value: u64 = grid[j][i].parse().unwrap();
            if action == Action::Add { column_total += value } else { column_total *= value};
        }

        total += column_total;
    }

    total
}


#[test]
fn small_input(){
    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

    assert_eq!(4277556, part_one(input));
}

