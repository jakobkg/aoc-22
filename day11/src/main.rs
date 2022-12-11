use std::{fs, process::exit};

struct Monkey {
    items: Vec<usize>,
    test: fn(usize) -> usize,
    op: fn(usize) -> usize
}

impl Monkey {
    pub fn turn(&mut self) {
        let mut passes: Vec<(usize, usize)> = Vec::new();

        for item in self.items.iter_mut() {
            *item = (self.op)(*item) / 3;
            passes.push((*item, (self.test)(*item)));
        }
    }
}

mod part1 {
    use crate::Monkey;

    pub fn solve(input: &str) -> usize {
        let monkey = Monkey {
            items: vec![79,98],
            test: |worry| {if worry % 23 == 0 {2} else {3}},
            op: |old| {old * 19},
        };

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
    const TEST_INPUT: &str = 
"Monkey 0:
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
        assert_eq!(part1::solve(TEST_INPUT), 10605);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 45000);
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

    println!("Part 1: The level of monkey business is {}", part1::solve(&input));
    println!("Part 2: {} calories", part2::solve(&input));
}
