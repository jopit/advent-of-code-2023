use std::iter::zip;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn ways_to_win(&self) -> u32 {
        let count: u32 = (0..=self.time)
            .map(|hold_time| hold_time * (self.time - hold_time)) /* hold_time is also the speed */
            .filter(|distance| *distance > self.distance)
            .count() as u32;
        count
    }
}

fn process(input: &str) -> String {
    let races = parse_races(input);
    let total: u32 = races.iter().map(|race| race.ways_to_win()).product();
    total.to_string()
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim().split(':').last().unwrap().trim())
        .collect();
    let times = lines[0]
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap());
    let distances = lines[1]
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap());
    let races: Vec<Race> = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    races
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 15,
            distance: 40,
        };
        assert_eq!(race.ways_to_win(), 8);
    }

    #[test]
    fn test_process() {
        let result = process(
            "Time:      7  15   30
             Distance:  9  40  200",
        );
        assert_eq!(result, "288".to_string());
    }
}
