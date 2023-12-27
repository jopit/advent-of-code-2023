fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct History {
    values: Vec<i64>,
}

impl History {
    fn new(values: Vec<i64>) -> Self {
        History { values }
    }

    fn extrapolate(&self) -> i64 {
        let mut data: Vec<Vec<i64>> = Vec::new();
        data.push(self.values.clone());

        // Calculate the differences
        let mut values = data.last().unwrap();
        while values.iter().find(|v| **v != 0).is_some() {
            let mut diffs: Vec<i64> = Vec::new();
            for i in 0..(values.len() - 1) {
                let value = values[i + 1] - values[i];
                diffs.push(value);
            }
            data.push(diffs);
            values = data.last().unwrap();
        }

        // Reverse data so we can iterate starting at 0
        data.reverse();

        // Calculate the extrapolation value
        data[0].push(0);
        let mut value: i64 = 0;
        for index in 1..data.len() {
            value = data[index].last().unwrap() + data[index - 1].last().unwrap();
            data[index].push(value);
        }
        value
    }
}

fn process(input: &str) -> String {
    let value: i64 = input
        .lines()
        .map(|line| parse_line(line))
        .map(|history| process_history(history))
        .sum();
    value.to_string()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|v| v.parse::<i64>())
        .map(|v| v.unwrap())
        .collect()
}

fn process_history(values: Vec<i64>) -> i64 {
    let history = History::new(values);
    history.extrapolate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "0 3 6 9 12 15
             1 3 6 10 15 21
             10 13 16 21 30 45",
        );
        assert_eq!(result, "114".to_string());
    }
}
