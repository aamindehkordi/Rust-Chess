use crate::model::pieces::piece::{Color, Piece, PieceType};


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
    Promo,
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

#[derive(Clone, PartialEq, Eq, Debug)]
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
    color: Color,
}

pub struct MoveHistory {
    piece: Box<dyn Piece>,
    mv: Move,
    notation: String,
}

impl Clone for MoveHistory {
    fn clone(&self) -> Self {
        Self {
            piece: self.piece.clone_box(),
            mv: self.mv.clone(),
            notation: self.notation.clone(),
        }
    }
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
            Self::Promo => true,
            Self::Promotion(_) => true,
            _ => false,
        }
    }

    pub fn is_promote_and_capture(&self) -> bool {
        match self {
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

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Invalid => false,
            _ => true,
        }
    }

    pub fn get_promotion_piece(&self) -> Option<PieceType> {
        match self {
            Self::Promotion(piece_type) => Some(*piece_type),
            Self::PromoteAndCapture(piece_type) => Some(*piece_type),
            _ => None,
        }
    }
}

impl Move {
    pub fn new(
        move_type: MoveType,
        from: (usize, usize),
        to: (usize, usize),
        color: Color,
    ) -> Self {
        Self {
            move_type,
            from,
            to,
            valid: false,
            color,
        }
    }

    pub fn to_history(&self, piece: Box<dyn Piece>) -> MoveHistory {
        MoveHistory {
            piece: piece.clone_box(),
            mv: Self {
                move_type: self.move_type.clone(),
                from: self.from,
                to: self.to,
                valid: self.valid,
                color: self.color.clone(),
            },
            notation: Self::get_notation(piece.clone_box()),
        }
    }

    pub fn get_notation(_piece: Box<dyn Piece>) -> String {
        
        // TODO: Add notation
        String::new()
    }
    pub fn get_color(&self) -> Color {
        self.color.clone()
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

    pub fn get_promotion(&self) -> PieceType {
        match self.move_type {
            MoveType::Promotion(piece_type) => piece_type,
            MoveType::PromoteAndCapture(piece_type) => piece_type,
            _ => PieceType::Pawn
        }
    }
}

impl MoveHistory {
    pub fn new(
        piece: Box<dyn Piece>,
        mv: Move,
        notation: String,
    ) -> Self {
        Self {
            piece,
            mv,
            notation,
        }
    }

    pub fn get_piece(&self) -> &Box<dyn Piece> {
        &self.piece
    }

    pub fn get_move_type(&self) -> &MoveType {
        &self.mv.move_type
    }

    pub fn get_from(&self) -> &(usize, usize) {
        &self.mv.from
    }

    pub fn get_to(&self) -> &(usize, usize) {
        &self.mv.to
    }

    pub fn get_notation(&self) -> &String {
        &self.notation
    }
}