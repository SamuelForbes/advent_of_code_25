use std::collections::HashSet;
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
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(grid[0].iter().position(|c| *c == 'S').unwrap());

    let mut split_count = 0;

    for row in grid {
        let mut new_beams = HashSet::new();
        for beam in &mut beams.iter() {
            if row[*beam] == '^' {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                split_count += 1;
            } else {
                new_beams.insert(*beam);
            }
        }
        beams = new_beams;
    }

    split_count
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut beams = vec![grid[0].iter().position(|c| *c == 'S').unwrap()];
    
    for row in grid {
        let mut new_beams = vec![];
        for beam in &mut beams.iter() {
            if row[*beam] == '^' {
                new_beams.push(beam - 1);
                new_beams.push(beam + 1);
            } else {
                new_beams.push(*beam);
            }
        }
        beams = new_beams;
    }

    beams.len()
}

#[test]
fn small_input() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(21, part_one(input));
    assert_eq!(40, part_two(input));
}
