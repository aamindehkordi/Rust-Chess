use crate::board::*;

pub type Moves = Vec<Move>;

#[derive(Debug, Copy, Clone)]
/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub struct Move {
    pub from: Position,
    pub to: Position,
}
