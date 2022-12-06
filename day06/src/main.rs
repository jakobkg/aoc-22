use std::{fs, process::exit, collections::HashSet};

fn incredible_solution(input: &str, window_width: usize) -> usize {
    input
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(window_width)
        .fold(0, |accumulator, window| {
            let mut set = HashSet::new();
            if accumulator == 0 && window.iter().all(|&(_, character)| set.insert(character)) {
                window.last().unwrap().0 + 1
            } else {
                accumulator
            }
        })
}

mod part1 {
    use crate::incredible_solution;

    pub fn solve(input: &str) -> usize {
        incredible_solution(input, 4)
    }
}

mod part2 {
    use crate::incredible_solution;

    pub fn solve(input: &str) -> usize {
        incredible_solution(input, 14)
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
        "Part 2: {} characters before the first message",
        part2::solve(&input)
    );
}
