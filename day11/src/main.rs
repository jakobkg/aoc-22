use std::{collections::VecDeque, fs, process::exit};

struct Test {
    divisor: u128,
    true_receiver: usize,
    false_receiver: usize,
}

#[derive(Copy, Clone)]
enum WorryLevel {
    Old,
    Value(u128),
}

#[derive(Copy, Clone)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
}

#[derive(Copy, Clone)]
enum ReliefMechanism {
    DivideByThree,
    Modulo,
}

struct Monkey {
    items: VecDeque<u128>,
    test: Test,
    op: Operation,
    relief_mechanism: ReliefMechanism,
    denominator: Option<u128>,
}

impl Monkey {
    pub fn inspect_and_fling(&mut self) -> Vec<(u128, usize)> {
        let mut passes: Vec<(u128, usize)> = Vec::new();

        while let Some(mut item) = self.items.pop_front() {
            item = match &self.op {
                Operation::Add(level) => match level {
                    WorryLevel::Old => item + item,
                    WorryLevel::Value(val) => item + val,
                },
                Operation::Multiply(level) => match level {
                    WorryLevel::Old => match item.checked_mul(item) {
                        Some(product) => product,
                        None => panic!("This number is too fuckin big"),
                    },
                    WorryLevel::Value(val) => match item.checked_mul(*val) {
                        Some(product) => product,
                        None => panic!("This number is way huge"),
                    },
                },
            };

            match self.relief_mechanism {
                ReliefMechanism::DivideByThree => {
                    item /= 3;
                }
                ReliefMechanism::Modulo => match self.denominator {
                    Some(denominator) => item %= denominator,
                    None => {}
                },
            }

            let receiver = if item % self.test.divisor == 0 {
                self.test.true_receiver
            } else {
                self.test.false_receiver
            };

            passes.push((item, receiver));
        }

        passes
    }

    pub fn give(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

struct Gang {
    monkeys: Vec<Monkey>,
    fling_counts: Vec<u128>,
}

impl Gang {
    pub fn new() -> Self {
        Self {
            monkeys: Vec::new(),
            fling_counts: Vec::new(),
        }
    }

    pub fn fling_stuff_around(&mut self) {
        for flinger in 0..self.monkeys.len() {
            let flings = self.monkeys[flinger].inspect_and_fling();

            for fling in flings {
                self.fling_counts[flinger] += 1;

                let (thing, catcher) = fling;
                self.monkeys[catcher].give(thing);
            }
        }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
        self.fling_counts.push(0);
    }
}

fn parse_monkeys(input: &str, relief_mechanism: ReliefMechanism) -> Gang {
    let mut monkeys = Gang::new();

    let mut denominator = 1u128;

    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|monkey_description| {
            let items = monkey_description[1][18..]
                .split(", ")
                .map(|str| match str.parse::<u128>() {
                    Ok(number) => number,
                    Err(_) => {
                        panic!("Failed to parse item worry level from {str}")
                    }
                })
                .collect::<Vec<u128>>();

            let op = monkey_description[2][23..]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .chunks(2)
                .map(|chunk| {
                    let level = match chunk[1] {
                        "old" => WorryLevel::Old,
                        _ => WorryLevel::Value(match chunk[1].parse::<u128>() {
                            Ok(value) => value,
                            Err(_) => panic!(
                                "Failed to parse monkey operand from \"{}\"",
                                monkey_description[2]
                            ),
                        }),
                    };

                    match chunk[0] {
                        "+" => Operation::Add(level),
                        "*" => Operation::Multiply(level),
                        _ => panic!(
                            "Failed to parse monkey operator from \"{}\"",
                            monkey_description[2]
                        ),
                    }
                })
                .collect::<Vec<Operation>>()[0];

            let test = Test {
                divisor: monkey_description[3]
                    .split_ascii_whitespace()
                    .last()
                    .map(|str| match str.parse::<u128>() {
                        Ok(divisor) => divisor,
                        Err(_) => panic!(
                            "Failed to parse divisor from line {}",
                            monkey_description[3]
                        ),
                    })
                    .unwrap(),
                true_receiver: monkey_description[4]
                    .split_ascii_whitespace()
                    .last()
                    .map(|str| match str.parse::<usize>() {
                        Ok(receiver) => receiver,
                        Err(_) => panic!(
                            "Failed to parse receiver from line {}",
                            monkey_description[4]
                        ),
                    })
                    .unwrap(),
                false_receiver: monkey_description[5]
                    .split_ascii_whitespace()
                    .last()
                    .map(|str| match str.parse::<usize>() {
                        Ok(receiver) => receiver,
                        Err(_) => panic!(
                            "Failed to parse receiver from line {}",
                            monkey_description[4]
                        ),
                    })
                    .unwrap(),
            };

            denominator *= test.divisor;

            Monkey {
                items: items.into(),
                op,
                test,
                relief_mechanism,
                denominator: None,
            }
        })
        .for_each(|monkey| monkeys.add_monkey(monkey));

    for monkey in monkeys.monkeys.iter_mut() {
        monkey.denominator = Some(denominator);
    }

    monkeys
}

mod part1 {
    use crate::{parse_monkeys, ReliefMechanism};

    pub fn solve(input: &str) -> u128 {
        let mut monkeys = parse_monkeys(input, ReliefMechanism::DivideByThree);

        for _ in 0..20 {
            monkeys.fling_stuff_around();
        }

        let mut counts = monkeys.fling_counts;

        counts.sort();

        match counts
            .iter()
            .rev()
            .take(2)
            .copied()
            .reduce(|accumulator, element| accumulator * element)
        {
            Some(businesslevel) => businesslevel,
            None => panic!("Not enough monkeys"),
        }
    }
}

mod part2 {
    use crate::{parse_monkeys, ReliefMechanism};

    pub fn solve(input: &str) -> u128 {
        let mut monkeys = parse_monkeys(input, ReliefMechanism::Modulo);

        for _ in 0..10000 {
            monkeys.fling_stuff_around();
        }

        let mut counts = monkeys.fling_counts;

        counts.sort();

        match counts
            .iter()
            .rev()
            .take(2)
            .copied()
            .reduce(|accumulator, element| accumulator * element)
        {
            Some(businesslevel) => businesslevel,
            None => panic!("Not enough monkeys"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
  
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 10_605);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 2_713_310_158);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day11/input") {
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
