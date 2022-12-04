use std::{fs, process::exit};

fn parse_line(line: &str) -> (usize, usize, usize, usize) {
    let mut values: Vec<usize> = Vec::new();
    let groups: Vec<&str> = line.split(",").collect();

    for group in groups {
        let mut ends: Vec<usize> = group
            .split("-")
            .map(|end| end.parse::<usize>().unwrap())
            .collect();

        values.append(&mut ends);
    }

    (values[0], values[1], values[2], values[3])
}

mod part1 {
    use crate::parse_line;

    pub fn solve(input: &str) -> usize {
        let mut overlapping_pair_count = 0usize;

        for line in input.lines() {
            let (first_start, first_end, second_start, second_end) = parse_line(line);

            if first_start >= second_start && first_end <= second_end {
                overlapping_pair_count += 1;
            } else if second_start >= first_start && second_end <= first_end {
                overlapping_pair_count += 1;
            }
        }

        overlapping_pair_count
    }
}

mod part2 {
    use crate::parse_line;

    pub fn solve(input: &str) -> usize {
        let mut overlapping_pair_count = 0usize;

        for line in input.lines() {
            let (first_start, first_end, second_start, second_end) = parse_line(line);

            if (first_start..=first_end).any(|n| [second_start, second_end].contains(&n))
                || (second_start..=second_end).any(|n| [first_start, first_end].contains(&n))
            {
                overlapping_pair_count += 1;
            }
        }

        overlapping_pair_count
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 2);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 4);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day04/input") {
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
