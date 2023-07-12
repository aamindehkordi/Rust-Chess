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

impl MoveType {
    pub fn is_capture(&self) -> bool {
        match self {
            Self::Capture => true,
            Self::PromoteAndCapture(_) => true,
            _ => false,
        }
    }

    pub fn is_promotion(&self) -> bool {
        match self {
            Self::Promotion(_) => true,
            Self::PromoteAndCapture(_) => true,
            _ => false,
        }
    }

    pub fn is_castle(&self) -> bool {
        match self {
            Self::Castle(_) => true,
            _ => false,
        }
    }

    pub fn is_en_passant(&self) -> bool {
        match self {
            Self::EnPassant => true,
            _ => false,
        }
    }

    pub fn is_double_push(&self) -> bool {
        match self {
            Self::DoublePush => true,
            _ => false,
        }
    }

    pub fn is_normal(&self) -> bool {
        match self {
            Self::Normal => true,
            _ => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Self::Invalid => true,
            _ => false,
        }
    }
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

#[derive(Clone, PartialEq, Debug)]
/// A struct that represents a move.
///
/// # Fields
/// * `move_type` - The type of move.
/// * `from` - The position the piece is moving from.
/// * `to` - The position the piece is moving to.
/// * `piece` - The type of piece.
pub struct Move {
    move_type: MoveType,
    from: (usize, usize),
    to: (usize, usize),
    valid: bool,
}

impl Move {
    pub fn new(
        move_type: MoveType,
        from: (usize, usize),
        to: (usize, usize),
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

    pub fn get_from(&self) -> &(usize, usize) {
        &self.from
    }

    pub fn get_to(&self) -> &(usize, usize) {
        &self.to
    }

    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}