use std::fs;

mod part1 {
    pub fn solve(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .max()
            .unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validate() {
            let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

            assert_eq!(solve(input), 24000);
        }
    }
}

mod part2 {
    pub fn solve(input: &str) -> usize {
        let mut calories: Vec<usize> = input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect();

        calories.sort_by(|a, b| b.cmp(a));

        calories[0..3].iter().sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validate() {
            let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

            assert_eq!(solve(input), 45000);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Unable to read input file");

    println!("Part 1: {} calories", part1::solve(&input));
    println!("Part 2: {} calories", part2::solve(&input));
}
