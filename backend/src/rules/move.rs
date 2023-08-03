use crate::board::piece::{Piece, PieceKind};
use crate::board::Position;
use crate::game::player::Color;

// Enum to represent different types of castle moves
#[derive(Copy, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

// Enum to represent different types of moves
#[derive(Clone)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    Capture,
    Castle(CastleType),
    EnPassant,
    Promotion(PieceKind),
    PromotionCapture(PieceKind),
}

// Struct to represent a move
#[derive(Clone)]
pub struct Move {
    pub from_piece: Piece,
    pub to: Position,
    pub move_type: MoveType,
    pub color: Color,
}

impl MoveType {
    pub fn is_normal(&self) -> bool {
        matches!(self, MoveType::Normal)
    }

    // Function to check if a move type is a promotion
    pub fn is_promotion(&self) -> bool {
        matches!(self, MoveType::Promotion(_))
    }

    // Function to check if a move type is a capture
    pub fn is_promo_capture(&self) -> bool {
        matches!(self, MoveType::PromotionCapture(_))
    }
}

impl Move {
    pub fn new(from_piece: Piece, to: Position, move_type: MoveType, color: Color) -> Self {
        Self {
            from_piece,
            to,
            move_type,
            color,
        }
    }

    pub fn is_capture(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Capture | MoveType::PromotionCapture(_)
        )
    }
}
