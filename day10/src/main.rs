use std::{collections::VecDeque, fs, process::exit};

#[derive(PartialEq)]
enum Operation {
    NOOP,
    ADD(i32),
}

#[derive(PartialEq)]
enum CPUState {
    Idle,
    Executing(Operation, i32),
}

struct CPU {
    x: i32,
    state: CPUState,
    clock: i32,
    queue: VecDeque<Operation>,
    signal_sum: i32,
    pixelbuf: String,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            x: 1,
            state: CPUState::Idle,
            clock: 0,
            queue: VecDeque::new(),
            signal_sum: 0,
            pixelbuf: String::new(),
        }
    }

    pub fn load_next_instruction(&mut self) {
        if let Some(op) = self.queue.pop_front() {
            self.state = CPUState::Executing(op, self.clock);
        } else {
            self.state = CPUState::Idle;
        }
    }

    pub fn tick(&mut self) {
        match &self.state {
            CPUState::Idle => {
                self.load_next_instruction();
            }

            CPUState::Executing(op, since) => match op {
                Operation::NOOP => {
                    self.load_next_instruction();
                }
                Operation::ADD(y) => {
                    if self.clock - since == 2 {
                        self.x += *y;
                        self.load_next_instruction();
                    }
                }
            },
        }

        if !(self.state == CPUState::Idle) {

            if ((self.x - 1)..=(self.x + 1)).contains(&(self.clock % 40)) {
                self.pixelbuf.push('#');
            } else {
                self.pixelbuf.push('.')
            }
            
            self.clock += 1;
            
            if self.clock % 40 == 0 {
                self.pixelbuf.push('\n');
            }

            if (self.clock + 20) % 40 == 0 {
                self.signal_sum += self.x * self.clock;
            }
        }

    }

    pub fn queue_op(&mut self, op: Operation) {
        self.queue.push_back(op);
    }

    pub fn is_done(&self) -> bool {
        self.queue.is_empty() && self.state == CPUState::Idle
    }
}

mod part1 {
    use crate::{Operation, CPU};

    pub fn solve(input: &str) -> i32 {
        let mut cpu = CPU::new();

        input.lines().for_each(|line| {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

            match tokens[0] {
                "noop" => cpu.queue_op(Operation::NOOP),
                "addx" => match tokens[1].parse::<i32>() {
                    Ok(y) => cpu.queue_op(Operation::ADD(y)),
                    Err(_) => panic!("Unable to parse add instruction! Line: {line}"),
                },
                _ => panic!("Unknown operation read! Line: {line}"),
            }
        });

        while !cpu.is_done() {
            cpu.tick();
        }

        cpu.signal_sum
    }
}

mod part2 {
    use crate::{Operation, CPU};

    pub fn solve(input: &str) -> String {
        let mut cpu = CPU::new();

        input.lines().for_each(|line| {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

            match tokens[0] {
                "noop" => cpu.queue_op(Operation::NOOP),
                "addx" => match tokens[1].parse::<i32>() {
                    Ok(y) => cpu.queue_op(Operation::ADD(y)),
                    Err(_) => panic!("Unable to parse add instruction! Line: {line}"),
                },
                _ => panic!("Unknown operation read! Line: {line}"),
            }
        });

        while !cpu.is_done() {
            cpu.tick();
        }

        cpu.pixelbuf
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";
    const TEST_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 13140);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), TEST_OUTPUT);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day07/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2:\n{}", part2::solve(&input));
}
