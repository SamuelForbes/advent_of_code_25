use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut start = Instant::now();
    println!("{} - {:?}", part_one(&input), start.elapsed());

    start = Instant::now();
    println!("{} - {:?}", part_two(&input), start.elapsed());
}
fn part_one(input: &str) -> u32{
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut counter = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' && has_more_than_four_neighbours((x, y), &grid){
                counter += 1;
            }
        }
    }

    counter
}

fn part_two(input: &str) -> u32{
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut counter = 0;

    loop {
        let mut round_counter = 0;

        for (y, row) in grid.clone().iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' && has_more_than_four_neighbours((x, y), &grid){
                    round_counter += 1;
                    grid[y][x] = 'x';
                }
            }
        }

        if round_counter == 0 {
            break;
        }

        counter += round_counter;
    }

    counter
}

fn has_more_than_four_neighbours((x, y): (usize, usize), grid: &[Vec<char>]) -> bool {
     [(0,1), (0, -1), (-1, 0), (1, 0), (-1, -1), (1, 1), (-1, 1), (1, -1)].iter()
        .filter(|(dx, dy)| {
            let (inspect_x, inspect_y) = (x as isize + dx, y as isize + dy);
            if inspect_x >= 0 && inspect_x < grid[0].len() as isize && inspect_y >= 0 && inspect_y < grid.len() as isize {
                return grid[inspect_y as usize][inspect_x as usize] == '@'
            }

            false
        }).count() < 4
}

#[test]
fn small_input(){
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    assert_eq!(13, part_one(input));
    assert_eq!(43, part_two(input));
}