use regex::Regex;

const EXPRESSION: &str = "([0-9]|one|two|three|four|five|six|seven|eight|nine)";
const REV_EXPRESSION: &str = "([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)";

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let re = Regex::new(EXPRESSION).unwrap();
    let rre = Regex::new(REV_EXPRESSION).unwrap();

    let mut lines = input.lines();
    let mut result: usize = 0;
    while let Some(line) = lines.next() {
        let mut value = String::new();
        
        let c1 = find_first_digit(line, &re);
        value.push(c1);
        let c2 = find_last_digit(line, &rre);
        value.push(c2);

        result += value.parse::<usize>().unwrap();
    }
    result.to_string()
}

fn find_first_digit(text: &str, re: &Regex) -> char {
    let Some(caps) = re.captures(text) else {
        panic!("no digit in string \"{text}\"")
    };
    match caps.get(1).map_or("", |m| m.as_str()) {
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4" => '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" => '8',
        "nine" | "9" => '9',

        _ => {
            panic!("no digit in string \"{text}\"")
        }
    }
}

fn find_last_digit(text: &str, re: &Regex) -> char {
    let mut chars: Vec<char> = text.chars().collect();
    chars.reverse();
    let rtext: String = chars.iter().collect();

    let Some(caps) = re.captures(&rtext) else {
        panic!("no digit in string \"{text}\"")
    };
    match caps.get(1).map_or("", |m| m.as_str()) {
        "eno" | "1" => '1',
        "owt" | "2" => '2',
        "eerht" | "3" => '3',
        "ruof" | "4" => '4',
        "evif" | "5" => '5',
        "xis" | "6" => '6',
        "neves" | "7" => '7',
        "thgie" | "8" => '8',
        "enin" | "9" => '9',

        _ => {
            panic!("no digit in string \"{text}\"")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_digit_one() {
        let re = Regex::new(EXPRESSION).unwrap();
        let result = find_first_digit("aonea", &re);
        assert_eq!(result, '1');
    }

    #[test]
    fn test_find_first_digit_digit() {
        let re = Regex::new(EXPRESSION).unwrap();
        let result = find_first_digit("aonea", &re);
        assert_eq!(result, '1');
        let result = find_first_digit("atwoa", &re);
        assert_eq!(result, '2');
        let result = find_first_digit("athreea", &re);
        assert_eq!(result, '3');
        let result = find_first_digit("afoura", &re);
        assert_eq!(result, '4');
        let result = find_first_digit("afivea", &re);
        assert_eq!(result, '5');
        let result = find_first_digit("asixa", &re);
        assert_eq!(result, '6');
        let result = find_first_digit("asevena", &re);
        assert_eq!(result, '7');
        let result = find_first_digit("aeighta", &re);
        assert_eq!(result, '8');
        let result = find_first_digit("aninea", &re);
        assert_eq!(result, '9');
    }

    #[test]
    fn test_find_last_digit() {
        let re = Regex::new(REV_EXPRESSION).unwrap();
        let result = find_last_digit("aonea", &re);
        assert_eq!(result, '1');
        let result = find_last_digit("atwoa", &re);
        assert_eq!(result, '2');
        let result = find_last_digit("athreea", &re);
        assert_eq!(result, '3');
        let result = find_last_digit("afoura", &re);
        assert_eq!(result, '4');
        let result = find_last_digit("afivea", &re);
        assert_eq!(result, '5');
        let result = find_last_digit("asixa", &re);
        assert_eq!(result, '6');
        let result = find_last_digit("asevena", &re);
        assert_eq!(result, '7');
        let result = find_last_digit("aeighta", &re);
        assert_eq!(result, '8');
        let result = find_last_digit("aninea", &re);
        assert_eq!(result, '9');
    }

    #[test]
    fn test_find_last_digit_digit() {
        let re = Regex::new(REV_EXPRESSION).unwrap();
        let result = find_last_digit("a1a", &re);
        assert_eq!(result, '1');
    }

    #[test]
    fn it_works() {
        let result = process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281".to_string());
    }
}
