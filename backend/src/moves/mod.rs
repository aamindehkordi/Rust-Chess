use crate::board::square::Position;
use crate::piece::PieceKind;

pub mod move_gen;

/// A list of offsets for each direction.
/// The directions are in the following order:
/// North, South, East, West, North East, North West, South East, South West.
pub const DIRECTION_OFFSETS: [i8; 8] = [-8, 8, 1, -1, -7, -9, 9, 7];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleSide {
    /// King side castle.
    KingSide = 0,
    /// Queen side castle.
    QueenSide = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveType {
    /// A move that does not capture or promote.
    Quiet,
    /// A move that moves a pawn two squares.
    DoublePush,
    /// A move that captures a piece.
    Capture,
    /// A move that captures a piece en passant.
    EnPassant,
    /// A move that castles.
    Castle(CastleSide),
    /// A move that promotes.
    Promotion(PieceKind),
    /// A move that captures a piece and promotes.
    PromotionCapture(PieceKind),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    /// The position of the piece to move.
    pub simple: FromTo,
    /// The type of move.
    pub move_type: MoveType,
}

impl Move {
    pub fn new(simple: FromTo, move_type: MoveType) -> Move {
        Move { simple, move_type }
    }
}

/// A list of moves.
/// Each move is a pair of positions.
pub type Moves = Vec<Move>;

/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub type FromTo = (Position, Position);
