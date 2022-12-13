use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    process::exit,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn neighbours(&self) -> Vec<Position> {
        let mut neighbours = Vec::new();

        match self.x.checked_sub(1) {
            Some(left) => neighbours.push(Position { x: left, y: self.y }),
            _ => {}
        }

        match self.y.checked_sub(1) {
            Some(above) => neighbours.push(Position {
                x: self.x,
                y: above,
            }),
            _ => {}
        }

        neighbours.push(Position {
            x: self.x + 1,
            y: self.y,
        });
        neighbours.push(Position {
            x: self.x,
            y: self.y + 1,
        });

        neighbours
    }
}

#[derive(Debug)]
struct Node {
    position: Position,
    height: usize,
    neighbours: Vec<Position>,
}

impl Node {
    pub fn new(position: Position, height: usize) -> Self {
        Self {
            position,
            height,
            neighbours: Vec::new(),
        }
    }
}

struct Graph {
    nodes: HashMap<Position, Node>,
    s: Option<Position>,
    e: Option<Position>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            s: None,
            e: None,
        }
    }

    pub fn add_node(&mut self, mut node: Node) {
        for neighbour in node.position.neighbours() {
            match self.nodes.get_mut(&neighbour) {
                Some(other) => {
                    if node.height <= other.height + 1 {
                        other.neighbours.push(node.position);
                    }
                    if other.height <= node.height + 1 {
                        node.neighbours.push(other.position);
                    }
                }
                None => {}
            }
        }

        self.nodes.insert(node.position, node);
    }

    pub fn find_path(&self, from: Position, to: Position) -> Result<Vec<Position>, ()> {
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();
        let mut prev = HashMap::new();

        explored.insert(from);
        queue.push_back(from);

        let mut pos = loop {
            let pos = match queue.pop_front() {
                Some(pos) => pos,
                None => return Err(()),
            };

            if pos == to {
                break pos;
            } else {
                for &neighbour in self.nodes.get(&pos).unwrap().neighbours.iter() {
                    if explored.insert(neighbour) {
                        prev.insert(neighbour, pos);
                        queue.push_back(neighbour);
                    }
                }
            }
        };

        let mut path = vec![];

        while pos != from {
            pos = *prev.get(&pos).unwrap();
            path.push(pos);
        }

        Ok(path)
    }
}

fn letter_to_height(letter: char) -> usize {
    match letter {
        'S' => 'a'.to_digit(36),
        'E' => 'z'.to_digit(36),
        _ => letter.to_digit(36),
    }
    .unwrap() as usize
}

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            graph.add_node(Node::new(Position { x, y }, letter_to_height(c)));

            match c {
                'S' => graph.s = Some(Position { x, y }),
                'E' => graph.e = Some(Position { x, y }),
                _ => {}
            }
        })
    });

    graph
}

mod part1 {
    use crate::parse_graph;

    pub fn solve(input: &str) -> usize {
        let graph = parse_graph(input);

        match (graph.s, graph.e) {
            (Some(s), Some(e)) => match graph.find_path(s, e) {
                Ok(path) => path.len(),
                Err(_) => 0,
            },
            _ => 0,
        }
    }
}

mod part2 {
    use crate::{letter_to_height, parse_graph};

    pub fn solve(input: &str) -> usize {
        let graph = parse_graph(input);

        match graph.e {
            Some(e) => {
                graph
                    .nodes
                    .values()
                    .filter(|node| node.height == letter_to_height('a'))
                    .map(|node| graph.find_path(node.position, e))
                    .filter(|result| result.is_ok())
                    .map(|path| path.unwrap().len())
                    .min()
                    .unwrap()
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn validate_part1() {
        assert_eq!(part1::solve(TEST_INPUT), 31);
    }

    #[test]
    fn validate_part2() {
        assert_eq!(part2::solve(TEST_INPUT), 29);
    }
}

fn main() {
    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day12/input") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        },
    };

    println!(
        "Part 1: The shortest path from S to E is {} steps",
        part1::solve(&input)
    );
    println!(
        "Part 2: The shortest path from any a to E is {} steps",
        part2::solve(&input)
    );
}
