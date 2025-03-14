use super::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board {
    to_play: Color,
    color_bb: [Bitboard; 2],
    type_bb: [Bitboard; 6],
}

impl Board {
    pub fn empty(to_play: Color) -> Self {
        Self {
            to_play,
            color_bb: [Bitboard::ZERO; 2],
            type_bb: [Bitboard::ZERO; 6],
        }
    }

    pub fn starting_position() -> Self {
        Self {
            to_play: Color::White,
            color_bb: [0xffff000000000000, 0x000000000000ffff],
            type_bb: [
                0x00ff00000000ff00,
                0x4200000000000042,
                0x8100000000000081,
                0x2400000000000024,
                0x1000000000000010,
                0x0800000000000008,
            ],
        }
    }

    pub fn get_square(&self, row: usize, col: usize) -> Option<Piece> {
        let color_idx = self
            .color_bb
            .iter()
            .enumerate()
            .filter(|(_, bb)| bb.get_bit(row, col))
            .map(|(i, _)| i)
            .next();

        let piece_idx = self
            .type_bb
            .iter()
            .enumerate()
            .filter(|(_, bb)| bb.get_bit(row, col))
            .map(|(i, _)| i)
            .next();

        match (color_idx, piece_idx) {
            (Some(color_idx), Some(piece_idx)) => Some(Piece::from_idx(color_idx, piece_idx)),
            _ => None,
        }
    }

    pub fn set_square(&mut self, row: usize, col: usize, piece: Piece) {
        for (i, bb) in self.color_bb.iter_mut().enumerate() {
            bb.store_bit(row, col, i == piece.to_idx().0)
        }

        for (i, bb) in self.type_bb.iter_mut().enumerate() {
            bb.store_bit(row, col, i == piece.to_idx().1)
        }
    }

    pub fn to_play(&self) -> Color {
        self.to_play
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for bb in self.color_bb.iter() {
            writeln!(f, "0x{:016x}", bb)?;
        }
        for bb in self.type_bb.iter() {
            writeln!(f, "0x{:016x}", bb)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in (0..8).rev() {
            for col in 0..8 {
                match self.get_square(row, col) {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, "-")?,
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
    use super::*;

    const STARTING_DEBUG_STR: &str = "\
        0xffff000000000000\n\
        0x000000000000ffff\n\
        0x00ff00000000ff00\n\
        0x4200000000000042\n\
        0x8100000000000081\n\
        0x2400000000000024\n\
        0x1000000000000010\n\
        0x0800000000000008\n\
    ";

    const STARTING_STR: &str = "\
        rnbqkbnr\n\
        pppppppp\n\
        --------\n\
        --------\n\
        --------\n\
        --------\n\
        PPPPPPPP\n\
        RNBQKBNR\n\
    ";

    #[test]
    fn display() {
        assert_eq!(format!("{}", Board::starting_position()), STARTING_STR);
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Board::starting_position()),
            STARTING_DEBUG_STR
        );
    }
}
