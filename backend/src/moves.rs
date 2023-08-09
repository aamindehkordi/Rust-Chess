use crate::board::*;
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
