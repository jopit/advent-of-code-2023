use std::isize;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    // Think of the ground as a pipe that doesn't connect to anything
    Ground,
}

impl Pipe {
    fn new(c: char) -> Pipe {
        use Pipe::*;
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            _ => panic!("unknown pipe: '{c}'"),
        }
    }

    fn opens_to(&self, direction: Direction) -> bool {
        use Direction::*;
        use Pipe::*;
        match *self {
            NorthSouth => match direction {
                North | South => true,
                East | West => false,
            },
            EastWest => match direction {
                East | West => true,
                North | South => false,
            },
            NorthEast => match direction {
                North | East => true,
                South | West => false,
            },
            NorthWest => match direction {
                North | West => true,
                South | East => false,
            },
            SouthWest => match direction {
                South | West => true,
                North | East => false,
            },
            SouthEast => match direction {
                South | East => true,
                North | West => false,
            },
            Ground => false,
        }
    }
}

trait Location<T> {
    fn neighbor(&self, direction: Direction) -> T;
}

impl Location<(isize, isize)> for (isize, isize) {
    fn neighbor(&self, direction: Direction) -> (isize, isize) {
        use Direction::*;
        match direction {
            North => (self.0 - 1, self.1),
            South => (self.0 + 1, self.1),
            East => (self.0, self.1 + 1),
            West => (self.0, self.1 - 1),
        }
    }
}

struct Map {
    map: Vec<Vec<Pipe>>,
    start_row: usize,
    start_col: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        // Build the map of the pipes
        let mut map: Vec<Vec<Pipe>> = Vec::new();
        let mut start_row = 0_usize;
        let mut start_col = 0_usize;
        for (row, line) in input.lines().map(|line| line.trim()).enumerate() {
            map.push(Vec::new());
            for (col, c) in line.chars().enumerate() {
                let pipe = if c == 'S' {
                    start_row = row;
                    start_col = col;
                    Pipe::new('.') // for now
                } else {
                    Pipe::new(c)
                };
                map[row].push(pipe);
            }
        }
        let mut map = Map {
            map,
            start_row,
            start_col,
        };

        // Figure out what kind of pipe the start is
        use Direction::*;
        use Pipe::*;
        let start = (start_row as isize, start_col as isize);
        let connections = (
            map.pipe_at(start.neighbor(North)).opens_to(South),
            map.pipe_at(start.neighbor(South)).opens_to(North),
            map.pipe_at(start.neighbor(East)).opens_to(West),
            map.pipe_at(start.neighbor(West)).opens_to(East),
        );
        let start_pipe = match connections {
            (true, true, false, false) => NorthSouth,
            (true, false, true, false) => NorthEast,
            (true, false, false, true) => NorthWest,
            (false, true, true, false) => SouthEast,
            (false, true, false, true) => SouthWest,
            (false, false, true, true) => EastWest,
            _ => panic!(
                "start doesn't connect properly (north, south, east, west): {:?}",
                connections
            ),
        };
        map.map[start_row][start_col] = start_pipe;

        map
    }

    fn pipe_at(&self, coords: (isize, isize)) -> Pipe {
        use Pipe::*;
        let (row, col) = coords;
        if row < 0 || row >= self.map.len() as isize {
            return Ground;
        }
        if col < 0 || col >= self.map[row as usize].len() as isize {
            return Ground;
        }
        self.map[row as usize][col as usize]
    }

    fn can_move(&self, location: (isize, isize), direction: Direction) -> bool {
        if !self.pipe_at(location).opens_to(direction) {
            return false;
        }
        if !self
            .pipe_at(location.neighbor(direction))
            .opens_to(direction.opposite())
        {
            return false;
        }
        true
    }

    fn calculate_distance(&self) -> u32 {
        use Direction::*;

        let start = (self.start_row as isize, self.start_col as isize);

        // Pick initial direction for the start. Track where we are by recording
        // the row and column, and the direction we came from
        let mut current: ((isize, isize), Direction) = if self.can_move(start, North) {
            (start.neighbor(North), South)
        } else if self.can_move(start, South) {
            (start.neighbor(South), North)
        } else if self.can_move(start, East) {
            (start.neighbor(East), West)
        } else if self.can_move(start, West) {
            (start.neighbor(West), East)
        } else {
            panic!("no route from start: {:?}", start)
        };

        // Find our way back to the start
        let mut count: u32 = 1;
        while current.0 != start {
            current = if current.1 != North && self.can_move(current.0, North) {
                (current.0.neighbor(North), South)
            } else if current.1 != South && self.can_move(current.0, South) {
                (current.0.neighbor(South), North)
            } else if current.1 != East && self.can_move(current.0, East) {
                (current.0.neighbor(East), West)
            } else if current.1 != West && self.can_move(current.0, West) {
                (current.0.neighbor(West), East)
            } else {
                panic!("no route from location: {:?}", current.0)
            };
            count += 1;
        }

        // Distance is half the steps from start back to start
        count / 2
    }
}

fn process(input: &str) -> String {
    let map = Map::new(input);
    map.calculate_distance().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Pipe::*;

    #[test]
    fn test_find_start_pipe() {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, SouthEast);

        let input = ".....
        .F-7.
        .|.|.
        .S-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, NorthEast);

        let input = ".....
        .F-7.
        .|.|.
        .L-S.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, NorthWest);

        let input = ".....
        .F-S.
        .|.|.
        .L-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, SouthWest);

        let input = ".....
        .FS7.
        .|.|.
        .L-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, EastWest);

        let input = ".....
        .-S-.
        .|.|.
        .L-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, EastWest);

        let input = ".....
        .F-7.
        .|.S.
        .L-J.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, NorthSouth);

        let input = ".....
        .F-|.
        .|.S.
        .L-|.
        .....";
        let map = Map::new(input);
        let start = map.map[map.start_row][map.start_col];
        assert_eq!(start, NorthSouth);
    }

    #[test]
    fn test_process() {
        let result = process(
            ".....
             .S-7.
             .|.|.
             .L-J.
             .....",
        );
        assert_eq!(result, "4".to_string());

        let result = process(
            "7-F7-
             .FJ|7
             SJLL7
             |F--J
             LJ.LJ",
        );
        assert_eq!(result, "8".to_string());
    }
}
