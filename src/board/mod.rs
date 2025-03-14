mod bitboard;
mod fen;
mod piece;
mod square;

use bitboard::{Bitboard, BitboardTrait};
use piece::{Color, Piece, Type};
use square::Square;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    to_play: Color,
    bitboards: [Bitboard; 12],
}

impl Board {
    pub fn empty(to_play: Color) -> Self {
        Self {
            to_play,
            bitboards: [Bitboard::ZERO; 12],
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::empty(Color::White);
        for (i, &color) in Color::iter().enumerate() {
            board.set_square(Square::new(i * 7, 0), Piece(color, Type::Rook));
            board.set_square(Square::new(i * 7, 1), Piece(color, Type::Knight));
            board.set_square(Square::new(i * 7, 2), Piece(color, Type::Bishop));
            board.set_square(Square::new(i * 7, 3), Piece(color, Type::Queen));
            board.set_square(Square::new(i * 7, 4), Piece(color, Type::King));
            board.set_square(Square::new(i * 7, 5), Piece(color, Type::Bishop));
            board.set_square(Square::new(i * 7, 6), Piece(color, Type::Knight));
            board.set_square(Square::new(i * 7, 7), Piece(color, Type::Rook));
        }

        for j in 0..8 {
            board.set_square(Square::new(1, j), Piece(Color::White, Type::Pawn));
            board.set_square(Square::new(6, j), Piece(Color::Black, Type::Pawn));
        }

        board
    }

    // pub fn get_bitboard(&self, piece: Piece) -> Bitboard {
    //     self.bitboards[piece.to_bb_index()]
    // }

    pub fn get_square(&self, square: Square) -> Option<Piece> {
        for (i, bb) in self.bitboards.iter().enumerate() {
            if bb.get_bit(square.row(), square.col()) {
                return Some(Piece::from_bb_index(i));
            }
        }
        None
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) {
        for (i, bb) in self.bitboards.iter_mut().enumerate() {
            bb.store_bit(square.row(), square.col(), i == piece.to_bb_index())
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "to play: {}\n", self.to_play)?;
        write!(f, "  a b c d e f g h \n")?;
        for row in (0..8).rev() {
            for col in 0..8 {
                if col == 0 {
                    write!(f, "{} ", ('1' as usize + row) as u8 as char)?;
                }
                match self.get_square(Square::new(row, col)) {
                    Some(piece) => write!(f, "{} ", piece)?,
                    None => write!(f, "- ")?,
                }
                if col == 7 {
                    write!(f, "\n")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    const STARTING_STR: &str = "\
        to play: white\n  \
          a b c d e f g h \n\
        8 r n b q k b n r \n\
        7 p p p p p p p p \n\
        6 - - - - - - - - \n\
        5 - - - - - - - - \n\
        4 - - - - - - - - \n\
        3 - - - - - - - - \n\
        2 P P P P P P P P \n\
        1 R N B Q K B N R \n\
    ";

    #[test]
    fn display() {
        let board = Board::starting_position();
        assert_eq!(format!("{}", board), STARTING_STR);
    }
}
