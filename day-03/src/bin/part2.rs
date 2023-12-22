fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let schematic: Vec<&[u8]> = input.lines().map(|s| -> &[u8] { s.as_bytes() }).collect();
    let mut machine = Machine::new(&schematic);
    let result = machine.gear_ratios_sum();
    result.to_string()
}

#[derive(Clone, Copy, PartialEq)]
struct Number {
    value: u32,
    index: usize,
    start: usize,
    end: usize,
}

impl Number {
    fn new(row: &[u8], index: usize, start: usize) -> Self {
        let mut curr = start;
        while curr < row.len() && (row[curr] as char).is_ascii_digit() {
            curr += 1;
        }
        let value: u32 = String::from_utf8(row[start..curr].to_vec())
            .unwrap()
            .parse()
            .unwrap();
        Number {
            value,
            index: index,
            start,
            end: curr,
        }
    }
}

#[derive(Clone, Copy)]
enum Entry {
    PartNumber(Number),
    Gear,
    Other,
}

struct Machine {
    machine: Vec<Vec<Entry>>,
}

impl Machine {
    fn new(schematic: &Vec<&[u8]>) -> Machine {
        let mut machine: Vec<Vec<Entry>> = Vec::new();
        for i in 0..schematic.len() {
            let srow = schematic[i];
            let mut row: Vec<Entry> = Vec::new();
            for j in 0..srow.len() {
                if srow[j].is_ascii_digit() {
                    if j > 0 && srow[j - 1].is_ascii_digit() {
                        row.push(row[j - 1]);
                    } else {
                        row.push(Entry::PartNumber(Number::new(srow, i, j)));
                    }
                } else if srow[j] == b'*' {
                    row.push(Entry::Gear);
                } else {
                    row.push(Entry::Other);
                }
            }
            machine.push(row);
        }

        Machine { machine }
    }

    fn gear_ratios_sum(&mut self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..self.machine.len() {
            let row = &self.machine[i];
            for j in 0..row.len() {
                match row[j] {
                    Entry::Gear => {
                        sum += self.gear_ratio(i as isize, j as isize);
                    }
                    _ => {}
                }
            }
        }
        sum
    }

    fn gear_ratio(&self, row: isize, col: isize) -> u32 {
        let mut part_numbers: Vec<Number> = Vec::new();

        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row - 1, col - 1));
        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row - 1, col));
        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row - 1, col + 1));

        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row, col - 1));
        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row, col + 1));

        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row + 1, col - 1));
        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row + 1, col));
        self.add_to_part_numbers(&mut part_numbers, self.entry_at(row + 1, col + 1));

        if part_numbers.len() == 2 {
            part_numbers[0].value * part_numbers[1].value
        } else {
            0
        }
    }

    fn add_to_part_numbers(&self, part_numbers: &mut Vec<Number>, entry: Entry) {
        match entry {
            Entry::PartNumber(number) if !part_numbers.contains(&number) => {
                part_numbers.push(number);
            }
            _ => {}
        }
    }

    fn entry_at(&self, row: isize, col: isize) -> Entry {
        if row < 0
            || row >= self.machine.len() as isize
            || col < 0
            || col >= self.machine[row as usize].len() as isize
        {
            Entry::Other
        } else {
            self.machine[row as usize][col as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835".to_string());
    }
}
