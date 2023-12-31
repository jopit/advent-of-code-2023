fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("unknown card: {c}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn new(input: &str) -> Self {
        // Create a map from cards to their frequencies.
        // We replace the actual values of the cards with
        // the value of the corresponding index into the
        // array of frequencies.
        //
        // The only reason we sort the cards is to group
        // cards with the same value together. This makes
        // it easy to replace them with the same index value.
        let mut cards = input.as_bytes().to_owned();
        cards.sort();
        let mut current = cards[0];
        let mut index = 0;
        for i in 0..cards.len() {
            if cards[i] != current {
                current = cards[i];
                index += 1;
            }
            cards[i] = index
        }
        let mut counts = [0, 0, 0, 0, 0];
        for i in 0..counts.len() {
            counts[cards[i] as usize] += 1;
        }

        // Use the card frequencies to find the hand's type
        counts.sort();
        match counts {
            [0, 0, 0, 0, 5] => Type::FiveOfAKind,
            [0, 0, 0, 1, 4] => Type::FourOfAKind,
            [0, 0, 0, 2, 3] => Type::FullHouse,
            [0, 0, 1, 1, 3] => Type::ThreeOfAKind,
            [0, 0, 1, 2, 2] => Type::TwoPair,
            [0, 1, 1, 1, 2] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => panic!("bad input: {input}, cards: {:#?}", cards),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand(Type, Card, Card, Card, Card, Card);

impl Hand {
    fn new(input: &str) -> Self {
        let mut cards = input.chars().map(|c| Card::new(c));
        Hand(
            Type::new(input),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        )
    }
}

fn process(input: &str) -> String {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    for line in input.lines() {
        let tmp: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand = Hand::new(tmp[0]);
        let bid: u32 = tmp[1].parse().unwrap();
        hands.push((hand, bid));
    }

    // Reverse sort the hands
    hands.sort_by(|hand1, hand2| hand2.0.cmp(&hand1.0));

    // Calculate the winnings
    let mut winnings: u32 = 0;
    for rank in 0..hands.len() {
        winnings += hands[rank].1 * (rank + 1) as u32;
    }
    winnings.to_string()
}

#[cfg(test)]
mod tests {
    use super::Card::*;
    use super::Type::*;
    use super::*;

    #[test]
    fn test_new_hand() {
        let hand = Hand::new("32T3K");
        assert_eq!(hand, Hand(OnePair, Three, Two, Ten, Three, King))
    }

    #[test]
    fn test_new_type() {
        let hand = "AAAAA";
        assert_eq!(Type::new(hand), FiveOfAKind);

        let hand = "AA8AA";
        assert_eq!(Type::new(hand), FourOfAKind);

        let hand = "23332";
        assert_eq!(Type::new(hand), FullHouse);

        let hand = "TTT98";
        assert_eq!(Type::new(hand), ThreeOfAKind);

        let hand = "23432";
        assert_eq!(Type::new(hand), TwoPair);

        let hand = "A23A4";
        assert_eq!(Type::new(hand), OnePair);

        let hand = "23456";
        assert_eq!(Type::new(hand), HighCard);
    }

    #[test]
    fn test_process() {
        let result = process(
            "32T3K 765
             T55J5 684
             KK677 28
             KTJJT 220
             QQQJA 483",
        );
        assert_eq!(result, "6440".to_string());
    }
}
