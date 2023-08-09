use crate::board::*;
use crate::piece::*;

pub const DIRECTION_OFFSETS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
pub type Moves = Vec<Move>;

#[derive(Debug, Copy, Clone)]
/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    /// Creates a new move.
    ///
    /// # Arguments
    /// * `from` - The position of the piece to move.
    /// * `to` - The position to move the piece to.
    ///
    /// # Example
    /// ```rs
    ///     let move_ = Move::new(0, 1);
    /// ```
    pub fn new(from: Position, to: Position) -> Move {
        Move { from, to }
    }
}

pub fn generate_moves(board: &Board) -> Moves {
    let mut moves = Moves::new();

    for start_square in 0..64 {
        let piece = board.squares[start_square].piece;
        if piece.type_ == PieceKind::None {
            continue;
        }

        if is_color(piece, board.turn) {
            if is_sliding_piece(piece) {
                generate_sliding_moves(board, start_square, &mut moves);
            }
        }



    }

    moves
}
