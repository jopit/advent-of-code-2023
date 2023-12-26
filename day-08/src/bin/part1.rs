use std::collections::HashMap;

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

struct Directions {
    directions: Vec<Direction>,
    index: usize,
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
            index: 0,
        }
    }
}

impl Iterator for Directions {
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

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn process(input: &str) -> String {
    use Direction::*;

    let mut lines = input.lines();

    let mut directions = lines
        .next()
        .map(|l| l.trim())
        .map(|l| Directions::new(l))
        .unwrap();

    lines.next();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        let node = parse_node(line.trim());
        nodes.insert(node.name, node);
    }

    let mut count: u32 = 0;
    let mut node = &nodes["AAA"];
    while node.name != "ZZZ" {
        node = match directions.next().unwrap() {
            Left => &nodes[node.left],
            Right => &nodes[node.right],
        };
        count += 1;
    }

    count.to_string()
}

fn parse_node(line: &str) -> Node {
    let mut words = line.split_ascii_whitespace();
    let name = words.next().unwrap().trim();
    words.next(); // consume '='
    let left = &words.next().unwrap()[1..4];
    let right = &words.next().unwrap().trim()[0..3];
    Node { name, left, right }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn test_directions() {
        let input = "RL";
        let mut directions = Directions::new(input);
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
            "RL

             AAA = (BBB, CCC)
             BBB = (DDD, EEE)
             CCC = (ZZZ, GGG)
             DDD = (DDD, DDD)
             EEE = (EEE, EEE)
             GGG = (GGG, GGG)
             ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2".to_string());

        let result = process(
            "LLR

             AAA = (BBB, BBB)
             BBB = (AAA, ZZZ)
             ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6".to_string());
    }
}
