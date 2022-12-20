use std::{fs, process::exit};

fn compare_packets(left: &str, right: &str) -> bool {
    
}

mod part1 {
    use crate::compare_packets;

    pub fn solve(input: &str) -> usize {
        let mut sum = 0;

        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .enumerate()
            .for_each(|(num, lines)| {
                if compare_packets(lines[0], lines[1]) {
                    sum += num;
                }
            });

        sum
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
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 1);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 1);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day13/input") {
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
