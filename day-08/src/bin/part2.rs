use std::collections::HashMap;

use aoc_utils::Tokenizer;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions {
    directions: Vec<Direction>,
}

impl Directions {
    fn new(input: &str) -> Self {
        Self {
            directions: input
                .chars()
                .map(|c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => {
                        panic!("unknown direction: {c}");
                    }
                })
                .collect(),
        }
    }

    fn iter<'a>(&'a self) -> DirectionsIterator<'a> {
        DirectionsIterator {
            directions: &self.directions,
            index: 0,
        }
    }
}

struct DirectionsIterator<'a> {
    directions: &'a Vec<Direction>,
    index: usize,
}

impl<'a> Iterator for DirectionsIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(self.directions[self.index]);
        self.index += 1;
        if self.index == self.directions.len() {
            self.index = 0;
        }
        next
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn next(&self, direction: Direction, node_map: &HashMap<&str, Node<'a>>) -> Self {
        use Direction::*;
        match direction {
            Left => node_map[self.left],
            Right => node_map[self.right],
        }
    }
}

fn start_nodes<'a>(node_map: &HashMap<&str, Node<'a>>) -> Vec<Node<'a>> {
    let mut nodes = Vec::new();
    node_map.keys().for_each(|key| {
        if key.ends_with("A") {
            nodes.push(node_map[key]);
        }
    });
    nodes
}

fn detect_cycle<'a>(
    start: Node<'a>,
    directions: &Directions,
    node_map: &'a HashMap<&'a str, Node<'a>>,
) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut current = start;
    let mut directions = directions.iter();
    while !current.name.ends_with("Z") {
        nodes.push(current);
        let direction = directions.next().unwrap();
        current = current.next(direction, node_map);
    }
    nodes
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn process(input: &str) -> String {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .map(|l| l.trim())
        .map(|l| Directions::new(l))
        .unwrap();

    lines.next();

    let mut node_map: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        let node = parse_node(line.trim());
        node_map.insert(node.name, node);
    }

    let mut steps: Vec<u64> = Vec::new();
    let start_nodes = start_nodes(&node_map);
    for node in start_nodes.iter() {
        let cycle = detect_cycle(*node, &directions, &node_map);
        steps.push(cycle.len() as u64);
    }

    let output = steps
        .iter()
        .map(|e| *e)
        .reduce(|acc, e| lcm(acc, e))
        .unwrap();
    output.to_string()
}

fn parse_node(line: &str) -> Node {
    let mut tokens = Tokenizer::new(line);

    let name = tokens.get();
    tokens.consume("=");
    tokens.consume("(");
    let left = tokens.get();
    tokens.consume(",");
    let right = tokens.get();
    tokens.consume(")");

    Node { name, left, right }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn test_directions() {
        let input = "RL";
        let directions = Directions::new(input);
        let mut directions = directions.iter();
        for _ in 0..100 {
            assert_eq!(directions.next().unwrap(), Right);
            assert_eq!(directions.next().unwrap(), Left);
        }
    }

    #[test]
    fn test_parse_node() {
        let line = "AAA = (BBB, CCC)";
        let value = parse_node(line);
        assert_eq!(value.name, "AAA");
        assert_eq!(value.left, "BBB");
        assert_eq!(value.right, "CCC");
    }

    #[test]
    fn test_process() {
        let result = process(
            "LR

             11A = (11B, XXX)
             11B = (XXX, 11Z)
             11Z = (11B, XXX)
             22A = (22B, XXX)
             22B = (22C, 22C)
             22C = (22Z, 22Z)
             22Z = (22B, 22B)
             XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string());
    }
}
