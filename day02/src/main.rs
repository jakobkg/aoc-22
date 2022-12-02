use std::{fs, process::exit};

mod part1 {
    pub fn solve(input: &str) -> usize {
        let mut score = 0usize;

        for line in input.lines() {
            let split: Vec<&str> = line.split(" ").collect();
            let opponent = split[0];
            let me = split[1];

            match me {
                "X" => match opponent {
                    "A" => score += 4,
                    "B" => score += 1,
                    "C" => score += 7,
                    _ => {}
                },

                "Y" => match opponent {
                    "A" => score += 8,
                    "B" => score += 5,
                    "C" => score += 2,
                    _ => {}
                },

                "Z" => match opponent {
                    "A" => score += 3,
                    "B" => score += 9,
                    "C" => score += 6,
                    _ => {}
                },
                _ => {}
            }
        }

        score
    }
}

mod part2 {
    pub fn solve(input: &str) -> usize {
        let mut score = 0usize;

        for line in input.lines() {
            let split: Vec<&str> = line.split(" ").collect();
            let opponent = split[0];
            let result = split[1];

            match opponent {
                "A" => match result {
                    "X" => score += 3,
                    "Y" => score += 4,
                    "Z" => score += 5,
                    _ => {}
                },
                "B" => match result {
                    "X" => score += 1,
                    "Y" => score += 5,
                    "Z" => score += 9,
                    _ => {}
                },
                "C" => match result {
                    "X" => score += 2,
                    "Y" => score += 6,
                    "Z" => score += 7,
                    _ => {}
                },
                _ => {}
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 15);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 12);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day02/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!("Part 1: {} points", part1::solve(&input));
    println!("Part 2: {} points", part2::solve(&input));
}
