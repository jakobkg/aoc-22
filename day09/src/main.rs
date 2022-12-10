use std::{collections::HashSet, fs, process::exit, cmp::Ordering};

#[derive(Debug)]
struct Rope {
    x: i32,
    y: i32,
    next: Box<RopeSegment>,
}

#[derive(Debug)]
struct RopeSegment {
    x: i32,
    y: i32,
    history: HashSet<(i32, i32)>,
    next: Option<Box<RopeSegment>>,
}

impl RopeSegment {
    pub fn new() -> Self {
        let mut history = HashSet::new();

        history.insert((0, 0));

        Self {
            x: 0,
            y: 0,
            history,
            next: None,
        }
    }

    pub fn extend(&mut self) {
        match &mut self.next {
            Some(next) => {
                next.extend();
            }

            None => {
                self.next = Some(Box::new(RopeSegment::new()));
            }
        }
    }

    pub fn update_next(&mut self) {
        match &mut self.next {
            Some(next) => {
                if (self.x - next.x).abs() <= 1 && (self.y - next.y).abs() <= 1 {
                    return;
                }

                match self.x.cmp(&next.x) {
                    Ordering::Less => {
                        next.x = self.x + 1;
                        next.y = self.y;
                        next.log_position();
                        next.update_next();
                        return;
                    }

                    Ordering::Greater => {
                        next.x = self.x - 1;
                        next.y = self.y;
                        next.log_position();
                        next.update_next();
                        return;
                    }

                    _ => {}
                }

                match self.y.cmp(&next.y) {
                    Ordering::Less => {
                        next.x = self.x;
                        next.y = self.y + 1;
                        next.log_position();
                        next.update_next();
                        return;
                    }

                    Ordering::Greater => {
                        next.x = self.x;
                        next.y = self.y - 1;
                        next.log_position();
                        next.update_next();
                        return;
                    }

                    _ => {}
                }
            }
            None => {}
        }
    }

    pub fn log_position(&mut self) {
        self.history.insert((self.x, self.y));
    }
}

impl Rope {
    pub fn new() -> Self {
        let next = Box::new(RopeSegment::new());
        Self { x: 0, y: 0, next }
    }

    pub fn extend(&mut self) {
        self.next.extend()
    }

    pub fn len(&self) -> usize {
        let mut len = 2;
        let mut ptr = &self.next;

        while let Some(next) = &ptr.next {
            ptr = next;
            len += 1;
        }

        len
    }

    pub fn up(&mut self) {
        self.y += 1;
        self.update_tail();
    }

    pub fn down(&mut self) {
        self.y -= 1;
        self.update_tail();
    }

    pub fn left(&mut self) {
        self.x -= 1;
        self.update_tail();
    }

    pub fn right(&mut self) {
        self.x += 1;
        self.update_tail();
    }

    pub fn update_tail(&mut self) {
        if (self.x - self.next.x).abs() <= 1 && (self.y - self.next.y).abs() <= 1 {
            return;
        }

        if self.x > self.next.x + 1 {
            self.next.x = self.x - 1;
            self.next.y = self.y;
            self.next.log_position();
            self.next.update_next();
        }

        if self.x < self.next.x - 1 {
            self.next.x = self.x + 1;
            self.next.y = self.y;
            self.next.log_position();
            self.next.update_next();
        }

        if self.y > self.next.y + 1 {
            self.next.x = self.x;
            self.next.y = self.y - 1;
            self.next.log_position();
            self.next.update_next();
        }

        if self.y < self.next.y - 1 {
            self.next.x = self.x;
            self.next.y = self.y + 1;
            self.next.log_position();
            self.next.update_next();
        }
    }
}

mod part1 {
    use crate::Rope;

    pub fn solve(input: &str) -> usize {
        let mut rope = Rope::new();

        for line in input.lines() {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

            let movement = match tokens[0] {
                "U" => Rope::up,
                "D" => Rope::down,
                "L" => Rope::left,
                "R" => Rope::right,
                _ => {
                    panic!("Unknown movement direction at line {line}")
                }
            };

            for _ in 0..tokens[1].parse::<usize>().unwrap() {
                movement(&mut rope);
            }
        }

        rope.next.history.len()
    }
}

mod part2 {
    use crate::Rope;

    pub fn solve(input: &str) -> usize {
        let mut rope = Rope::new();

        while rope.len() < 9 {
            rope.extend();
        }

        for line in input.lines() {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

            let movement = match tokens[0] {
                "U" => Rope::up,
                "D" => Rope::down,
                "L" => Rope::left,
                "R" => Rope::right,
                _ => {
                    panic!("Unknown movement direction at line {line}")
                }
            };

            for _ in 0..tokens[1].parse::<usize>().unwrap() {
                movement(&mut rope);
            }
        }

        let mut tail = &rope.next;

        while let Some(next) = &tail.next {
            tail = next;
        }

        tail.history.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const TEST_INPUT_2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    const TEST_INPUT_3: &str = "R 5\nU 8";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 13);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 1);
        assert_eq!(part2::solve(TEST_INPUT_2), 36);
        assert_eq!(part2::solve(TEST_INPUT_3), 2);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day09/input") {
            Ok(input) => input,
            Err(_) => {
                print!("Could not find input file");
                exit(1);
            }
        },
    };

    println!(
        "Part 1: The tail of the rope visited {} squares",
        part1::solve(&input)
    );
    println!(
        "Part 2: The tail of the rope visited {} squares",
        part2::solve(&input)
    );
}
