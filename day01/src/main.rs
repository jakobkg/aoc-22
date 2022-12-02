use std::{fs, process::exit};

mod part1 {
    pub fn solve(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .max()
            .unwrap()
    }
}

mod part2 {
    pub fn solve(input: &str) -> usize {
        let mut calories: Vec<usize> = input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect();

        calories.sort_by(|a, b| b.cmp(a));

        calories[0..3].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 24000);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 45000);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day01/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!("Part 1: {} calories", part1::solve(&input));
    println!("Part 2: {} calories", part2::solve(&input));
}
