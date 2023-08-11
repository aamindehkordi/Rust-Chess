use crate::board::square::Position;
use crate::piece::PieceKind;

pub mod move_gen;

/// A list of offsets for each direction.
/// The directions are in the following order:
/// North West, North, North East, West, East, South West, South, South East.
pub const DIRECTION_OFFSETS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

pub enum MoveType {
    Quiet,
    Capture,
    Castle(CastleSide),
    EnPassant,
    Promotion(PieceKind),
}

pub struct Move {
    pub simple: SimpleMove,
    pub move_type: MoveType,
}

/// A list of moves.
/// Each move is a pair of positions.
pub type SimpleMoves = Vec<SimpleMove>;

/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub type SimpleMove = (Position, Position);
