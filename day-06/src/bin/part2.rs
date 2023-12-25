fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        // Use quadratic formula to calculate the upper and lower bounds for the winning hold times

        let sqrt_discriminant = f64::sqrt((self.time * self.time) - 4.0 * (self.distance));

        let upper = ((self.time) + sqrt_discriminant) / 2.0;
        let upper = if f64::floor(upper) == upper {
            (upper + 1.0) as u64
        } else {
            upper as u64
        };

        let lower = ((self.time) - sqrt_discriminant) / 2.0;
        let lower = if f64::ceil(lower) == lower {
            (lower - 1.0) as u64
        } else {
            f64::ceil(lower) as u64
        };

        upper - lower + 1
    }
}

fn process(input: &str) -> String {
    let race = parse_race(input);
    let total = race.ways_to_win();
    total.to_string()
}

fn parse_race(input: &str) -> Race {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim().split(':').last().unwrap().trim())
        .collect();
    let time = lines[0]
        .split_ascii_whitespace()
        .fold(String::new(), |mut acc, num| {
            acc.push_str(num);
            acc
        })
        .parse::<f64>()
        .unwrap();
    let distance = lines[1]
        .split_ascii_whitespace()
        .fold(String::new(), |mut acc, num| {
            acc.push_str(num);
            acc
        })
        .parse::<f64>()
        .unwrap();

    Race { time, distance }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 15.0,
            distance: 40.0,
        };
        assert_eq!(race.ways_to_win(), 8);
    }

    #[test]
    fn test_process() {
        let result = process(
            "Time:      7  15   30
             Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string());
    }
}
