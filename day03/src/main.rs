use std::{fs, process::exit};

fn char_to_priority(char: char) -> usize {
    if char.is_ascii_lowercase() {
        (char as u32 - 'a' as u32 + 1) as usize
    } else {
        (char as u32 - 'A' as u32 + 27) as usize
    }
}

mod part1 {
    use crate::char_to_priority;

    pub fn solve(input: &str) -> usize {
        let mut score = 0usize;

        for line in input.lines() {
            let first = line[0..line.len() / 2].to_string();
            let second = line[line.len() / 2..line.len()].to_string();

            for letter in first.chars() {
                if second.contains(letter) {
                    score += char_to_priority(letter);
                    break;
                }
            }
        }

        score
    }
}

mod part2 {
    use crate::char_to_priority;

    pub fn solve(input: &str) -> usize {
        let mut score = 0;
        let mut lines = input.lines();

        loop {
            let first = match lines.next() {
                Some(line) => line.to_string(),
                None => break,
            };

            let second = match lines.next() {
                Some(line) => line.to_string(),
                None => {
                    break;
                }
            };

            let third = match lines.next() {
                Some(line) => line.to_string(),
                None => {
                    break;
                }
            };

            for letter in first.chars() {
                if second.contains(letter) && third.contains(letter) {
                    score += char_to_priority(letter);
                    break;
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 157);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 70);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day03/input") {
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
