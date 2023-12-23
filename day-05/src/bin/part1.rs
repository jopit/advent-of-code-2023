use std::{cmp, str::Lines};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct Range {
    length: u64,
    source_start: u64,
    destination_start: u64,
}

impl Range {
    fn new(range: &str) -> Self {
        let tmp: Vec<u64> = range
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Range {
            length: tmp[2],
            source_start: tmp[1],
            destination_start: tmp[0],
        }
    }

    fn convert(&self, source: u64) -> Option<u64> {
        if source < self.source_start {
            return None;
        }
        if source >= self.source_start + self.length {
            return None;
        }
        Some(self.destination_start + (source - self.source_start))
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Map { ranges }
    }

    fn convert(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .flat_map(|range| range.convert(source))
            .next()
            .unwrap_or(source)
    }
}

struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn new(maps: Vec<Map>) -> Self {
        Almanac { maps }
    }

    fn convert(&self, source: u64) -> u64 {
        self.maps
            .iter()
            .fold(source, |value, map| map.convert(value))
    }
}

fn process(input: &str) -> String {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    lines.next(); // consume blank line

    let mut maps: Vec<Map> = Vec::new();
    while let Some(_) = lines.next() {
        process_map(&mut maps, &mut lines);
    }

    let almanac = Almanac::new(maps);
    let result = seeds
        .iter()
        .map(|seed| almanac.convert(*seed))
        .reduce(|smallest, current| cmp::min(smallest, current))
        .unwrap();

    result.to_string()
}

fn process_map(maps: &mut Vec<Map>, lines: &mut Lines) {
    let mut ranges: Vec<Range> = Vec::new();
    while let Some(line) = lines.map(|l| l.trim()).next() {
        if line.len() == 0 {
            break;
        }
        ranges.push(Range::new(line));
    }
    maps.push(Map::new(ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_new() {
        let range = Range::new("50 98 2");
        assert_eq!(range.destination_start, 50);
        assert_eq!(range.source_start, 98);
        assert_eq!(range.length, 2);
    }

    #[test]
    fn test_range_convert() {
        let range = Range::new("50 98 2");
        assert_eq!(range.convert(97), None);
        assert_eq!(range.convert(98), Some(50));
        assert_eq!(range.convert(99), Some(51));
        assert_eq!(range.convert(100), None);
    }

    #[test]
    fn test_map_convert() {
        let ranges = vec![Range::new("50 98 2"), Range::new("52 50 48")];
        let map = Map::new(ranges);

        assert_eq!(map.convert(0), 0);
        assert_eq!(map.convert(1), 1);

        assert_eq!(map.convert(48), 48);
        assert_eq!(map.convert(49), 49);
        assert_eq!(map.convert(50), 52);
        assert_eq!(map.convert(51), 53);
        assert_eq!(map.convert(52), 54);

        assert_eq!(map.convert(95), 97);
        assert_eq!(map.convert(96), 98);
        assert_eq!(map.convert(97), 99);
        assert_eq!(map.convert(98), 50);
        assert_eq!(map.convert(99), 51);
    }

    #[test]
    fn test_almanac() {
        let mut maps: Vec<Map> = Vec::new();
        let map = Map::new(vec![Range::new("50 98 2"), Range::new("52 50 48")]);
        maps.push(map);
        let map = Map::new(vec![
            Range::new("0 15 37"),
            Range::new("37 52 2"),
            Range::new("39 0 15"),
        ]);
        maps.push(map);
        let almanac = Almanac::new(maps);

        assert_eq!(almanac.convert(79), 81);
        assert_eq!(almanac.convert(14), 53);
        assert_eq!(almanac.convert(55), 57);
        assert_eq!(almanac.convert(13), 52);
    }

    #[test]
    fn test_process() {
        let result = process(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!(result, "35".to_string());
    }
}
