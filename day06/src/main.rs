use std::{fs, process::exit};

mod part1 {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        input
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .windows(4)
            .fold(0, |a, w| {
                let mut set = HashSet::new();
                if a == 0 && w.iter().all(move |p| set.insert(p.1)) {
                    w[3].0 + 1
                } else {
                    a
                }
            })
    }
}

mod part2 {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        input
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .windows(14)
            .fold(0, |a, w| {
                let mut set = HashSet::new();
                if a == 0 && w.iter().all(move |p| set.insert(p.1)) {
                    w[13].0 + 1
                } else {
                    a
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT_1), 7);
        assert_eq!(part1::solve(TEST_INPUT_2), 5);
        assert_eq!(part1::solve(TEST_INPUT_3), 6);
        assert_eq!(part1::solve(TEST_INPUT_4), 10);
        assert_eq!(part1::solve(TEST_INPUT_5), 11);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT_1), 19);
        assert_eq!(part2::solve(TEST_INPUT_2), 23);
        assert_eq!(part2::solve(TEST_INPUT_3), 23);
        assert_eq!(part2::solve(TEST_INPUT_4), 29);
        assert_eq!(part2::solve(TEST_INPUT_5), 26);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day05/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!(
        "Part 1: {} characters before the first marker",
        part1::solve(&input)
    );
    println!(
        "Part 2: {} characters before first message",
        part2::solve(&input)
    );
}
