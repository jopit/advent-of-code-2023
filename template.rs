fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut result: u32 = 0;

    for line in input.lines() {
        todo!()
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process("");
        assert_eq!(result, "todo".to_string());
    }
}
