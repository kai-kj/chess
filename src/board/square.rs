#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Square {
    row: usize,
    col: usize,
}

impl Square {
    pub fn new(row: usize, col: usize) -> Square {
        assert!(row < 8 && col < 8);
        Square { row, col }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn rank(&self) -> char {
        ('1' as usize + self.row) as u8 as char
    }

    pub fn file(&self) -> char {
        ('a' as usize + self.col) as u8 as char
    }
}

impl std::str::FromStr for Square {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars();

        Ok(Square {
            col: match s.next() {
                Some('a') => 0,
                Some('b') => 1,
                Some('c') => 2,
                Some('d') => 3,
                Some('e') => 4,
                Some('f') => 5,
                Some('g') => 6,
                Some('h') => 7,
                _ => return Err(()),
            },
            row: match s.next() {
                Some('1') => 0,
                Some('2') => 1,
                Some('3') => 2,
                Some('4') => 3,
                Some('5') => 4,
                Some('6') => 5,
                Some('7') => 6,
                Some('8') => 7,
                _ => return Err(()),
            },
        })
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQUARE_NAMES: [[&str; 8]; 8] = [
        ["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"],
        ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"],
        ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"],
        ["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"],
        ["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"],
        ["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"],
        ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"],
        ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"],
    ];

    #[test]
    fn parse() {
        for row in 0..8 {
            for col in 0..8 {
                assert_eq!(
                    SQUARE_NAMES[row][col].parse::<Square>(),
                    Ok(Square::new(row, col))
                );
            }
        }
    }

    #[test]
    fn display() {
        for row in 0..8 {
            for col in 0..8 {
                assert_eq!(Square::new(row, col).to_string(), SQUARE_NAMES[row][col]);
            }
        }
    }
}
