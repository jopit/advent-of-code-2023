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
    let id: usize = iter
        .next()
        .expect("can't find id in \"{game}\"")
        .parse()
        .unwrap();

    let mut samples = iter.next().unwrap().split(';');
    while let Some(sample) = samples.next() {
        if is_impossible(sample) {
            return 0;
        }
    }

    id
}

fn is_impossible(sample: &str) -> bool {
    let mut color_values = sample.split(',');
    while let Some(color_value) = color_values.next() {
        let mut cv = color_value.trim().split_ascii_whitespace();
        let value: usize = cv.next().unwrap().parse().unwrap();
        let color = cv.next().unwrap();
        match color {
            "red" => {
                if value > 12 {
                    return true;
                } else {
                    continue;
                }
            }
            "green" => {
                if value > 13 {
                    return true;
                } else {
                    continue;
                }
            }
            "blue" => {
                if value > 14 {
                    return true;
                } else {
                    continue;
                }
            }
            _ => panic!("unknown color {color}"),
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_impossible() {
        assert!(!is_impossible(" 2 red"));
        assert!(!is_impossible(" 3 green"));
        assert!(!is_impossible(" 4 blue"));

        assert!(!is_impossible(" 12 red"));
        assert!(!is_impossible(" 13 green"));
        assert!(!is_impossible(" 14 blue"));

        assert!(is_impossible(" 13 red"));
        assert!(is_impossible(" 14 green"));
        assert!(is_impossible(" 15 blue"));
    }

    #[test]
    fn test_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(game);
        assert_eq!(result, 1);

        let game = "Game 1: 20 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(game);
        assert_eq!(result, 0);
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
        assert_eq!(result, "8".to_string());
    }
}
