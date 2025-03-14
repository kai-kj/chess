#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece(pub Color, pub Type);

impl Piece {
    pub fn from_bb_index(bb_index: usize) -> Self {
        Piece(
            Color::from_bb_index(bb_index),
            Type::from_bb_index(bb_index),
        )
    }

    pub fn to_bb_index(self) -> usize {
        self.0.to_bb_index() + self.1.to_bb_index()
    }
}

impl std::str::FromStr for Piece {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P" => Ok(Piece(Color::White, Type::Pawn)),
            "N" => Ok(Piece(Color::White, Type::Knight)),
            "R" => Ok(Piece(Color::White, Type::Rook)),
            "B" => Ok(Piece(Color::White, Type::Bishop)),
            "Q" => Ok(Piece(Color::White, Type::Queen)),
            "K" => Ok(Piece(Color::White, Type::King)),
            "p" => Ok(Piece(Color::Black, Type::Pawn)),
            "n" => Ok(Piece(Color::Black, Type::Knight)),
            "r" => Ok(Piece(Color::Black, Type::Rook)),
            "b" => Ok(Piece(Color::Black, Type::Bishop)),
            "q" => Ok(Piece(Color::Black, Type::Queen)),
            "k" => Ok(Piece(Color::Black, Type::King)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Piece(Color::White, Type::Pawn) => write!(f, "P"),
            Piece(Color::White, Type::Knight) => write!(f, "N"),
            Piece(Color::White, Type::Rook) => write!(f, "R"),
            Piece(Color::White, Type::Bishop) => write!(f, "B"),
            Piece(Color::White, Type::Queen) => write!(f, "Q"),
            Piece(Color::White, Type::King) => write!(f, "K"),
            Piece(Color::Black, Type::Pawn) => write!(f, "p"),
            Piece(Color::Black, Type::Knight) => write!(f, "n"),
            Piece(Color::Black, Type::Rook) => write!(f, "r"),
            Piece(Color::Black, Type::Bishop) => write!(f, "b"),
            Piece(Color::Black, Type::Queen) => write!(f, "q"),
            Piece(Color::Black, Type::King) => write!(f, "k"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 6,
}

impl Color {
    fn from_bb_index(bb_index: usize) -> Self {
        match bb_index / 6 {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!("invalid bitboard index"),
        }
    }

    fn to_bb_index(&self) -> usize {
        *self as usize
    }

    pub fn iter() -> std::slice::Iter<'static, Color> {
        static COLORS: [Color; 2] = [Color::White, Color::Black];
        COLORS.iter()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Pawn = 0,
    Knight = 1,
    Rook = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl Type {
    fn from_bb_index(bb_index: usize) -> Self {
        match bb_index % 6 {
            0 => Type::Pawn,
            1 => Type::Knight,
            2 => Type::Rook,
            3 => Type::Bishop,
            4 => Type::Queen,
            5 => Type::King,
            _ => panic!("invalid bitboard index"),
        }
    }

    fn to_bb_index(&self) -> usize {
        *self as usize
    }

    pub fn iter() -> std::slice::Iter<'static, Type> {
        static TYPES: [Type; 6] = [
            Type::Pawn,
            Type::Knight,
            Type::Rook,
            Type::Bishop,
            Type::Queen,
            Type::King,
        ];
        TYPES.iter()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Pawn => write!(f, "pawn"),
            Type::Knight => write!(f, "knight"),
            Type::Rook => write!(f, "rook"),
            Type::Bishop => write!(f, "bishop"),
            Type::Queen => write!(f, "queen"),
            Type::King => write!(f, "king"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIECE_NAMES: [&str; 12] = ["P", "N", "R", "B", "Q", "K", "p", "n", "r", "b", "q", "k"];

    #[test]
    fn bb_index() {
        for &c in Color::iter() {
            for &t in Type::iter() {
                let i = Piece(c, t).to_bb_index();
                assert_eq!(i, t.to_bb_index() + c.to_bb_index());
                assert_eq!(t, Type::from_bb_index(i));
                assert_eq!(c, Color::from_bb_index(i));
                assert_eq!(Piece(c, t), Piece::from_bb_index(i));
            }
        }
    }

    #[test]
    fn parse() {
        let mut i = 0;
        for &c in Color::iter() {
            for &t in Type::iter() {
                assert_eq!(PIECE_NAMES[i].parse::<Piece>(), Ok(Piece(c, t)));
                i += 1;
            }
        }
    }

    #[test]
    fn display() {
        let mut i = 0;
        for &c in Color::iter() {
            for &t in Type::iter() {
                assert_eq!(Piece(c, t).to_string(), PIECE_NAMES[i]);
                i += 1;
            }
        }
    }
}
