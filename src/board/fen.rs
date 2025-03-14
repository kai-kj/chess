// https://www.chessprogramming.org/Forsyth-Edwards_Notation

use super::*;

pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";

pub fn encode_fen(board: Board) -> String {
    let mut fen = String::new();
    let mut empty = 0;
    for row in (0..8).rev() {
        for col in 0..8 {
            match board.get_square(row, col) {
                Some(piece) => {
                    if empty != 0 {
                        fen += &empty.to_string();
                        empty = 0;
                    }
                    fen += &piece.to_string();
                }
                None => empty += 1,
            }
        }
        if empty != 0 {
            fen += &empty.to_string();
            empty = 0;
        }
        if row > 0 {
            fen += "/"
        }
    }

    match board.to_play() {
        Color::White => fen += " w",
        Color::Black => fen += " b",
    }

    fen
}

pub fn decode_fen(fen: &str) -> Result<Board, FenDecodeError> {
    let mut fen = fen.split(" ");

    let piece_placement = match fen.next() {
        Some(placement) => placement,
        None => return Err(FenDecodeError::NoPiecePlacementSection),
    };

    let side_to_move = match fen.next() {
        Some(side_to_move) => side_to_move,
        None => return Err(FenDecodeError::NoSideToMoveSection),
    };

    let mut board = match side_to_move {
        "w" => Board::empty(Color::White),
        "b" => Board::empty(Color::Black),
        other => return Err(FenDecodeError::InvalidSideToMove(other.to_string())),
    };

    let mut row_count = 0;
    for row in piece_placement.split("/") {
        let mut col_count = 0;
        for piece in row.chars() {
            match piece.to_string().parse::<Piece>() {
                Ok(piece) => {
                    board.set_square(7 - row_count, col_count, piece);
                    col_count += 1
                }
                Err(_) => match piece.to_string().parse::<usize>() {
                    Ok(piece_num) => col_count += piece_num,
                    Err(_) => return Err(FenDecodeError::InvalidPiece(piece.to_string())),
                },
            }
        }

        if col_count != 8 {
            return Err(FenDecodeError::InvalidPieceCount(
                col_count,
                row.to_string(),
            ));
        }

        row_count += 1;
    }

    if row_count != 8 {
        return Err(FenDecodeError::InvalidRowCount(row_count));
    }

    Ok(board)
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum FenDecodeError {
    #[error("no <piece placement> section found")]
    NoPiecePlacementSection,
    #[error("no <side to move> section found")]
    NoSideToMoveSection,
    #[error("invalid side to move \"{0}\"")]
    InvalidSideToMove(String),
    #[error("invalid piece \"{0}\"")]
    InvalidPiece(String),
    #[error("invalid piece count {0} in row \"{1}\"")]
    InvalidPieceCount(usize, String),
    #[error("invalid row count {0}")]
    InvalidRowCount(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        assert_eq!(encode_fen(Board::starting_position()), STARTING_POSITION);
    }

    #[test]
    fn decode() {
        assert_eq!(
            decode_fen(STARTING_POSITION).unwrap(),
            Board::starting_position()
        );
    }

    #[test]
    fn decode_no_side_to_move() {
        assert_eq!(
            decode_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Err(FenDecodeError::NoSideToMoveSection)
        );
    }

    #[test]
    fn decode_invalid_piece() {
        assert_eq!(
            decode_fen("rnbqkbnx/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
            Err(FenDecodeError::InvalidPiece("x".to_string()))
        );
    }

    #[test]
    fn decode_invalid_piece_count() {
        assert_eq!(
            decode_fen("rnbqkbn/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
            Err(FenDecodeError::InvalidPieceCount(7, "rnbqkbn".to_string()))
        );
    }

    #[test]
    fn decode_invalid_row_count() {
        assert_eq!(
            decode_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP w"),
            Err(FenDecodeError::InvalidRowCount(7))
        );
    }
}
