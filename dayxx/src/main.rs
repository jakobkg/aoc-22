use std::{fs, process::exit};

mod part1 {
    pub fn solve(input: &str) -> usize {
        0
    }
}

mod part2 {
    pub fn solve(input: &str) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 1);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 1);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("dayxx/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!(
        "Part 1: The level of monkey business after 20 rounds is {}",
        part1::solve(&input)
    );
    println!(
        "Part 2: The level of monkey business after 10000 rounds is {}!!",
        part2::solve(&input)
    );
}
