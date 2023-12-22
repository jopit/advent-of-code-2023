
struct Row<'a> {
    row: &'a [u8],
    index: usize,
    curr: usize,
}

impl<'a> Row<'a> {
    fn new(row: &'a [u8], index: usize) -> Self {
        Row {
            row,
            index,
            curr: 0,
        }
    }
}

impl Iterator for Row<'_> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr < self.row.len() && !(self.row[self.curr] as char).is_ascii_digit() {
            self.curr += 1;
        }
        if self.curr == self.row.len() {
            return None;
        }
        let start = self.curr;

        while self.curr < self.row.len() && (self.row[self.curr] as char).is_ascii_digit() {
            self.curr += 1;
        }
        let value: u32 = String::from_utf8(self.row[start..self.curr].to_vec())
            .unwrap()
            .parse()
            .unwrap();
        let result = Number {
            value,
            index: self.index,
            start,
            end: self.curr,
        };
        Some(result)
    }
}

struct Number {
    value: u32,
    index: usize,
    start: usize,
    end: usize,
}

impl Number {
    fn is_part_number(&self, schematic: &Vec<&[u8]>) -> bool {
        let row = schematic[self.index];

        if self.start > 0 && row[self.start - 1] != b'.' {
            return true;
        }

        if self.end < row.len() && row[self.end] != b'.' {
            return true;
        }

        if self.index != 0 {
            let row_above = schematic[self.index - 1];
            if self.start > 0 && row_above[self.start - 1] != b'.' {
                return true;
            }
            for i in self.start..self.end {
                if row_above[i] != b'.' {
                    return true;
                }
            }
            if self.end < row.len() && row_above[self.end] != b'.' {
                return true;
            }
        }

        if self.index != schematic.len() - 1 {
            let row_below = schematic[self.index + 1];
            if self.start > 0 && row_below[self.start - 1] != b'.' {
                return true;
            }
            for i in self.start..self.end {
                if row_below[i] != b'.' {
                    return true;
                }
            }
            if self.end < row.len() && row_below[self.end] != b'.' {
                return true;
            }
        }

        false
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut result: u32 = 0;
    let schematic: Vec<&[u8]> = input.lines().map(|s| -> &[u8] { s.as_bytes() }).collect();
    for row_index in 0..schematic.len() {
        let mut row = Row::new(schematic[row_index], row_index);
        while let Some(number) = row.next() {
            if number.is_part_number(&schematic) {
                result += number.value;
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_iter() {
        let schematic = "467..114..";
        let mut row = Row::new(schematic.as_bytes(), 0);

        let first = row.next().unwrap();
        assert_eq!(first.value, 467);

        let next = row.next().unwrap();
        assert_eq!(next.value, 114);

        assert!(row.next().is_none())
    }

    fn into_schematic(strings: Vec<&str>) -> Vec<&[u8]> {
        strings.iter().map(|s| s.as_bytes()).collect()
    }

    #[test]
    fn test_is_part_number() {
        let schematic = into_schematic(vec!["*...", ".1.."]);
        let mut row = Row::new(schematic[1], 1);
        let number = row.next().unwrap();
        assert!(number.is_part_number(&schematic));

        let schematic = into_schematic(vec!["...*", "..1."]);
        let mut row = Row::new(schematic[1], 1);
        let number = row.next().unwrap();
        assert!(number.is_part_number(&schematic));

        let schematic = into_schematic(vec![".1..", "*..."]);
        let mut row = Row::new(schematic[0], 0);
        let number = row.next().unwrap();
        assert!(number.is_part_number(&schematic));

        let schematic = into_schematic(vec!["..1.", "...*"]);
        let mut row = Row::new(schematic[0], 0);
        let number = row.next().unwrap();
        assert!(number.is_part_number(&schematic));

        let schematic = into_schematic(vec!["....", ".1.."]);
        let mut row = Row::new(schematic[1], 1);
        let number = row.next().unwrap();
        assert!(!number.is_part_number(&schematic));
    }

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
        assert_eq!(result, "4361".to_string());
    }
}
