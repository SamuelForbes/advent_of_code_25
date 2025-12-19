fn main() {
    println!("Hello, world!");
}

struct JunctionBox {
    location: (usize, usize, usize),
    distances_to_others: Vec<(usize, f32)>
}

impl JunctionBox {
    fn new(location: (usize, usize, usize)) -> JunctionBox {
        JunctionBox {
            location,
            distances_to_others: Vec::new()
        }
    }

    fn get_nearest_neighbour(&self) -> &(usize, f32) {
        self.distances_to_others.iter().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap()
    }
}

fn part_one(input: &str) -> u32 {
    let mut locations = parse_input(input);
    calculate_distances(&mut locations);

    0
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input.lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            JunctionBox::new((parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap()))
        })
        .collect()
}

fn calculate_distances(locations: &mut [JunctionBox]) {
    for i in  0..locations.len() {
        for j in 0..locations.len() {
            if i == j {
                continue;
            }

            locations[i].distances_to_others.push((j, calculate_distance(locations[i].location, locations[j].location)));
        }
    }
}

fn calculate_distance(base: (usize, usize, usize), target: (usize, usize, usize)) -> f32{
    ((base.0 as f32 - target.0 as f32) + (base.1 as f32 - target.1 as f32) + (base.2 as f32 - target.2 as f32)).abs().sqrt()
}

#[test]
fn small_input() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(40, part_one(input));
}