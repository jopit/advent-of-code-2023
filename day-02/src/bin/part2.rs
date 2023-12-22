use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut games = input.lines();
    let mut result: usize = 0;
    while let Some(game) = games.next() {
        result += parse_game(game);
    }
    result.to_string()
}

fn parse_game(game: &str) -> usize {
    let input = &game["Game ".len()..];
    let mut iter = input.split(':');

    // Skip the id
    iter.next();

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("red", 0);
    map.insert("green", 0);
    map.insert("blue", 0);
    let mut samples = iter.next().unwrap().split(';');
    while let Some(sample) = samples.next() {
        update_map(sample, &mut map);
    }
    let power = map["red"] * map["green"] * map["blue"];
    power
}

fn update_map<'a>(sample: &'a str, map: &mut HashMap<&'a str, usize>) {
    let mut color_values = sample.split(',');
    while let Some(color_value) = color_values.next() {
        let mut cv = color_value.trim().split_ascii_whitespace();

        let value: usize = cv.next().unwrap().parse().unwrap();
        let color = cv.next().unwrap();

        let existing = map.get(color).unwrap_or(&0);
        if value > *existing {
            map.insert(color, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_map() {
        let mut map = HashMap::new();
        map.insert("red", 1);
        update_map(" 3 blue, 4 red", &mut map);
        assert_eq!(map["red"], 4);
        assert_eq!(map.get("green"), None);
        assert_eq!(map["blue"], 3)
    }

    #[test]
    fn test_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(game);
        assert_eq!(result, 48);

        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_game(game);
        assert_eq!(result, 12);

        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = parse_game(game);
        assert_eq!(result, 1560);

        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = parse_game(game);
        assert_eq!(result, 630);

        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse_game(game);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_process() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string());
    }
}
