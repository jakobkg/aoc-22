use std::{collections::HashSet, fs, process::exit};

type Obstacle = (u32, u32);

struct Cave {
    obstacles: HashSet<Obstacle>,
    depth: u32,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            obstacles: HashSet::new(),
            depth: 2,
        }
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.insert(obstacle);
    }

    pub fn check_coordinates(&self, x: u32, y: u32) -> bool {
        self.obstacles.contains(&(x, y)) || y >= self.depth
    }
}

fn parse_input(input: &str) -> Cave {
    let mut cave = Cave::new();

    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|pos| {
                let nums = pos
                    .split(",")
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                (nums[0], nums[1])
            })
            .collect::<Vec<(u32, u32)>>()
            .windows(2)
            .for_each(|window| {
                let from = window[0];
                let to = window[1];

                let from_x = from.0.min(to.0);
                let to_x = from.0.max(to.0);

                let from_y = from.1.min(to.1);
                let to_y = from.1.max(to.1);

                for x in from_x..=to_x {
                    for y in from_y..=to_y {
                        cave.add_obstacle((x, y));

                        if y > cave.depth - 2 {
                            cave.depth = y + 2;
                        }
                    }
                }
            })
    });

    cave
}

mod part1 {
    use crate::{parse_input, Obstacle};

    pub fn solve(input: &str) -> usize {
        let mut cave = parse_input(input);
        let mut sand_counter = 0;
        let mut sand: Obstacle;

        'outer: loop {
            sand = (500, 0);

            loop {
                if !cave.check_coordinates(sand.0, sand.1 + 1) {
                    sand = (sand.0, sand.1 + 1);
                } else if !cave.check_coordinates(sand.0 - 1, sand.1 + 1) {
                    sand = (sand.0 - 1, sand.1 + 1);
                } else if !cave.check_coordinates(sand.0 + 1, sand.1 + 1) {
                    sand = (sand.0 + 1, sand.1 + 1);
                } else {
                    cave.add_obstacle(sand);
                    sand_counter += 1;
                    break;
                }

                if sand.1 + 1 == cave.depth {
                    break 'outer;
                }
            }
        }

        sand_counter
    }
}

mod part2 {
    use crate::{parse_input, Obstacle};

    pub fn solve(input: &str) -> usize {
        let mut cave = parse_input(input);
        let mut sand_counter = 0;
        let mut sand: Obstacle;

        while !cave.check_coordinates(500, 0) {
            sand = (500, 0);

            loop {
                if !cave.check_coordinates(sand.0, sand.1 + 1) {
                    sand = (sand.0, sand.1 + 1);
                } else if !cave.check_coordinates(sand.0 - 1, sand.1 + 1) {
                    sand = (sand.0 - 1, sand.1 + 1);
                } else if !cave.check_coordinates(sand.0 + 1, sand.1 + 1) {
                    sand = (sand.0 + 1, sand.1 + 1);
                } else {
                    cave.add_obstacle(sand);
                    sand_counter += 1;
                    break;
                }
            }
        }

        sand_counter
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 24);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 93);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day14/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!(
        "Part 1: {} units of sand were able to settle before sand fell into the void",
        part1::solve(&input)
    );
    println!(
        "Part 2: {} units of sand were able to settle before the source was blocked",
        part2::solve(&input)
    );
}
