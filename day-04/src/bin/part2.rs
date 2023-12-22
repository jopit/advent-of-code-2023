fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct Card {
    id: usize,
    number_of_matches: usize,
}

impl Card {
    fn new(card: &str) -> Self {
        let tmp: Vec<&str> = card[4..].trim().split(':').collect();
        let id: usize = tmp[0].trim().parse().unwrap();
        let numbers: Vec<&str> = tmp[1].split('|').collect();

        let winning_numbers: Vec<&str> = numbers[0]
            .split_ascii_whitespace()
            .map(|n| n.trim())
            .collect();
        let our_numbers: Vec<&str> = numbers[1]
            .split_ascii_whitespace()
            .map(|n| n.trim())
            .collect();

        let mut number_of_matches: usize = 0;
        for n in our_numbers.iter() {
            if winning_numbers.contains(n) {
                number_of_matches += 1;
            }
        }

        Card {
            id,
            number_of_matches,
        }
    }
}

fn process(input: &str) -> String {
    let mut cards: Vec<Card> = Vec::new();
    for card in input.lines() {
        cards.push(Card::new(card));
    }

    let mut index: usize = 0;
    while index < cards.len() {
        let card = cards[index];
        for i in card.id..(card.id + card.number_of_matches) {
            cards.push(cards[i]);
        }
        index += 1;
    }

    cards.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_card() {
        let card = Card::new("Card         1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.id, 1);
        assert_eq!(card.number_of_matches, 4);
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
        assert_eq!(result, "30".to_string());
    }
}
