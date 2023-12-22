fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut result: u32 = 0;
    for card in input.lines() {
        result += process_card(card)
    }
    result.to_string()
}

fn process_card(card: &str) -> u32 {
    let card = card.split(':').last().unwrap();
    let numbers: Vec<&str> = card.split('|').collect();
    let winning_numbers: Vec<&str> = numbers[0]
        .split_ascii_whitespace()
        .map(|n| n.trim())
        .collect();
    let our_numbers: Vec<&str> = numbers[1]
        .split_ascii_whitespace()
        .map(|n| n.trim())
        .collect();

    let mut result: u32 = 0;
    for n in our_numbers.iter() {
        if winning_numbers.contains(n) {
            result = if result == 0 { 1 } else { result * 2 }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_card() {
        let result = process_card("Card         1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_process() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13".to_string());
    }
}
