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

#[derive(PartialEq, Debug)]
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

    fn base_value(&self) -> u64 {
        match self {
            Action::Add => 0,
            Action::Multiply => 1,
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

fn part_two(input: &str) -> u64 {
    let grid: Vec<Vec<Vec<char>>> = input.lines().map(|line| line.split_ascii_whitespace().map(|value| value.chars().collect()).collect()).collect();

    let mut total = 0;

    for i in 0..grid[0].len() {
        let action = Action::from(grid.last().unwrap()[i][0]);

        let mut calculation_total = action.base_value();
        let maximum_instruction_length = find_maximum_instruction_length(&grid, i);
        println!("{maximum_instruction_length}");

        for j in (0..(maximum_instruction_length - 1)).rev() {
            println!("{j}");
            let mut value_parts = Vec::new();
            for k in 0..grid.len() - 1 {
                println!("{:?}", grid[k][i]);
                if j < grid[k][i].len(){
                    value_parts.push(grid[k][i][j]);
                }
            }
            let value: u64 = value_parts.iter().collect::<String>().parse().unwrap();
            if action == Action::Add { calculation_total += value } else { calculation_total *= value};
        }

        total += calculation_total;
    }

    total
}

fn find_maximum_instruction_length(grid: &Vec<Vec<Vec<char>>>, i: usize) -> usize {
    let mut maximum_instruction_length: usize = 0;

    for j in 0..grid.len() - 1 {
        let instruction_length = grid[j][i].len();
        if instruction_length > maximum_instruction_length {
            maximum_instruction_length = instruction_length;
        }
    }

    maximum_instruction_length
}


#[test]
fn small_input(){
    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

    assert_eq!(4_277_556, part_one(input));
    assert_eq!(3_263_827, part_two(input));
}

