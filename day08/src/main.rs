use std::{fs, ops::Index, process::exit};

struct Forest {
    trees: Vec<Vec<u8>>,
}

impl Forest {
    pub fn size(&self) -> usize {
        self.trees.len()
    }
}

impl Index<usize> for Forest {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.trees[index]
    }
}

fn input_to_forest(input: &str) -> Forest {
    let mut forest: Forest = Forest { trees: Vec::new() };

    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|line| {
            forest.trees.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            )
        });

    forest
}

mod part1 {
    use crate::input_to_forest;

    pub fn solve(input: &str) -> usize {
        let forest = input_to_forest(input);
        let mut visible_count = 0;

        for row in 0..forest.size() {
            'col: for col in 0..forest.size() {
                if row == 0 || col == 0 || row == forest.size() - 1 || col == forest.size() - 1 {
                    visible_count += 1;
                    continue 'col;
                }

                for check in 0..col {
                    if forest[row][check] >= forest[row][col] {
                        break;
                    }

                    if check == col - 1 {
                        visible_count += 1;
                        continue 'col;
                    }
                }

                for check in col + 1..forest.size() {
                    if forest[row][check] >= forest[row][col] {
                        break;
                    }

                    if check == forest.size() - 1 {
                        visible_count += 1;
                        continue 'col;
                    }
                }

                for check in 0..row {
                    if forest[check][col] >= forest[row][col] {
                        break;
                    }

                    if check == row - 1 {
                        visible_count += 1;
                        continue 'col;
                    }
                }

                for check in row + 1..forest.size() {
                    if forest[check][col] >= forest[row][col] {
                        break;
                    }

                    if check == forest.size() - 1 {
                        visible_count += 1;
                        continue 'col;
                    }
                }
            }
        }

        visible_count
    }
}

mod part2 {
    use crate::input_to_forest;

    pub fn solve(input: &str) -> usize {
        let forest = input_to_forest(input);
        let mut record = 0;
        let mut left_score;
        let mut right_score;
        let mut up_score;
        let mut down_score;

        for row in 0..forest.size() {
            'col: for col in 0..forest.size() {
                if row == 0 || col == 0 || row == forest.size() - 1 || col == forest.size() - 1 {
                    continue 'col;
                }

                left_score = 0;

                for check in (0..col).rev() {
                    left_score += 1;

                    if forest[row][check] >= forest[row][col] {
                        break;
                    }
                }

                right_score = 0;

                for check in col + 1..forest.size() {
                    right_score += 1;

                    if forest[row][check] >= forest[row][col] {
                        break;
                    }
                }

                up_score = 0;

                for check in (0..row).rev() {
                    up_score += 1;

                    if forest[check][col] >= forest[row][col] {
                        break;
                    }
                }

                down_score = 0;

                for check in row + 1..forest.size() {
                    down_score += 1;

                    if forest[check][col] >= forest[row][col] {
                        break;
                    }
                }

                let candidate = up_score * down_score * left_score * right_score;

                if candidate > record {
                    record = candidate
                }
            }
        }

        record
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 21);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 8);
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

    println!("Part 1: {} trees are visible", part1::solve(&input));
    println!(
        "Part 2: The highest possible scenic score is {}",
        part2::solve(&input)
    );
}
