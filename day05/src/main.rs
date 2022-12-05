use std::{fs, process::exit};

fn parse_initial_state(input: &str) -> Vec<Vec<char>> {
    let mut state: Vec<Vec<char>> = Vec::new();

    input
        .lines()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            line.char_indices()
                .filter(|(idx, _)| *idx > 0 && (idx - 1) % 4 == 0)
                .for_each(|(idx, c)| {
                    if (idx - 1) / 4 == state.len() {
                        state.push(Vec::new());
                    }

                    if c != ' ' {
                        state[(idx - 1) / 4].push(c);
                    }
                })
        });

    state.iter_mut().for_each(|stack| stack.reverse());

    state
}

mod part1 {
    use crate::parse_initial_state;

    pub fn solve(input: &str) -> String {
        let mut state = parse_initial_state(input);

        input
            .lines()
            .filter(|line| line.starts_with("move"))
            .for_each(|line| {
                let split: Vec<&str> = line.split_ascii_whitespace().collect();
                let num = split[1].parse::<usize>().unwrap();
                let from = split[3].parse::<usize>().unwrap() - 1;
                let to = split[5].parse::<usize>().unwrap() - 1;
                for _ in 0..num {
                    let c = state[from].pop().unwrap();
                    state[to].push(c);
                }
            });

        let mut result = "".to_string();

        for mut stack in state {
            result = format!("{result}{}", stack.pop().unwrap_or('\0'));
        }

        result
    }
}

mod part2 {
    use crate::parse_initial_state;

    pub fn solve(input: &str) -> String {
        let mut state = parse_initial_state(input);

        input
            .lines()
            .filter(|line| line.starts_with("move"))
            .for_each(|line| {
                let split: Vec<&str> = line.split_ascii_whitespace().collect();

                let num = split[1].parse::<usize>().unwrap();
                let from = split[3].parse::<usize>().unwrap() - 1;
                let to = split[5].parse::<usize>().unwrap() - 1;
                let mut crane: Vec<char> = Vec::new();

                for _ in 0..num {
                    let c = state[from].pop().unwrap();
                    crane.push(c);
                }

                for _ in 0..num {
                    state[to].push(crane.pop().unwrap());
                }


            });

        let mut result = "".to_string();

        for mut stack in state {
            result = format!("{result}{}", stack.pop().unwrap_or(' '));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), "CMZ");
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), "MCD");
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

    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2: {}", part2::solve(&input));
}
