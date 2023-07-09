use crate::model::pieces::piece::{PieceType};
use crate::model::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An enum that represents the type of move.
///
/// # Variants
/// * `Normal` - A normal move.
/// * `DoublePush` - A double pawn push.
/// * `Capture` - A capture.
/// * `EnPassant` - An en passant capture.
/// * `Castle` - A castle.
/// * `Promotion` - A promotion.
/// * `PromoteAndCapture` - A capture that leads to a promotion.
pub enum MoveType {
    Normal,
    DoublePush,
    Capture,
    EnPassant,
    Castle(CastleType),
    Promotion(PieceType),
    PromoteAndCapture(PieceType),
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An enum that represents the type of castle.
///
/// # Variants
/// * `Kingside` - A kingside castle.
/// * `Queenside` - A queenside castle.
pub enum CastleType {
    Kingside,
    Queenside,
}

#[derive(Clone, PartialEq)]
/// A struct that represents a move.
///
/// # Fields
/// * `move_type` - The type of move.
/// * `from` - The tile the piece is moving from.
/// * `to` - The tile the piece is moving to.
/// * `piece` - The type of piece.
pub struct Move {
    move_type: MoveType,
    from: Tile,
    to: Tile,
    valid: bool,
}

impl Move {
    pub fn new(
        move_type: MoveType,
        from: Tile,
        to: Tile,
    ) -> Self {
        Self {
            move_type,
            from,
            to,
            valid: false,
        }
    }

    pub fn get_move_type(&self) -> &MoveType {
        &self.move_type
    }

    pub fn get_from(&self) -> &Tile {
        &self.from
    }

    pub fn get_to(&self) -> &Tile {
        &self.to
    }

    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn valid(&self) -> bool {
        self.valid
    }


}