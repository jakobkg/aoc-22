use std::{collections::HashMap, fs, process::exit};

type Stack<'a> = Vec::<&'a str>;

trait Reset {
    fn reset(&mut self);
}

impl <'a>Reset for Stack<'a> {
    fn reset(&mut self) {
        self.clear();
        self.push("/");
    }
}

fn disk_usage(input: &str) -> HashMap<String, usize> {
    let mut usage: HashMap<String, usize> = HashMap::new();
    let mut dirstack: Stack = Stack::new();

    usage.insert("/".into(), 0);
    dirstack.reset();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Stack>();

        match tokens[0] {
            "$" => match tokens[1] {
                "cd" => match tokens[2] {
                    "/" => {
                        dirstack.reset();
                    }
                    ".." => {
                        dirstack.pop();
                    }
                    _ => {
                        dirstack.push(tokens[2]);

                        if !usage.contains_key(&dirstack.join("/")) {
                            usage.insert(dirstack.join("/"), 0);
                        }
                    }
                },
                "ls" => continue,
                _ => panic!("An oopsie occurred while parsing line: {line}"),
            },
            "dir" => continue,
            _ => {
                for i in 1..=dirstack.len() {
                    let dir = dirstack[0..i].join("/");
                    usage
                        .entry(dir)
                        .and_modify(|size| *size += tokens[0].parse::<usize>().unwrap());
                }
            }
        }
    }

    usage
}

mod part1 {
    use crate::disk_usage;

    pub fn solve(input: &str) -> usize {
        let mut usage = disk_usage(input);

        usage
            .drain()
            .filter(|(_, size)| size <= &mut 100_000)
            .fold(0, |sum, (_, size)| sum + size)
    }
}

mod part2 {
    use crate::disk_usage;

    pub fn solve(input: &str) -> usize {
        let mut usage = disk_usage(input);

        let total_usage = match usage.get("/") {
            Some(usage) => usage.clone(),
            None => {
                panic!("/ has no size for some reason")
            }
        };

        usage
            .drain()
            .filter(|(_, size)| total_usage - size < 40_000_000)
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 95437);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 24933642);
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
    println!("Part 2: {}", part2::solve(&input));
}
