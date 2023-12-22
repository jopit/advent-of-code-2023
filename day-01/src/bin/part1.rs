fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = input.lines();
    let mut result: usize = 0;
    while let Some(line) = lines.next() {
        let mut value = String::new();
        let c1 = find_first_digit(line);
        value.push(c1);
        let c2 = find_last_digit(line);
        value.push(c2);
        result += value.parse::<usize>().unwrap();
    }
    result.to_string()
}

fn find_first_digit(text: &str) -> char {
    let chars = text.chars();
    for c in chars {
        if c.is_digit(10) {
            return c
        }
    }
    panic!("no digit in string {text}")
}

fn find_last_digit(text: &str) -> char {
    let chars: Vec<char> = text.chars().collect();
    for index in (0..chars.len()).rev() {
        if chars[index].is_digit(10) {
            return chars[index]
        }
    }
    panic!("no digit in string {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142".to_string());
    }
}
