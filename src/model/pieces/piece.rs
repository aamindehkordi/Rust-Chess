use crate::model::board::Board;
use crate::model::tile::Tile;
use crate::model::pieces::pawn::{Pawn};
use crate::model::pieces::rook::{Rook};
use crate::model::pieces::bishop::{Bishop};
use crate::model::pieces::knight::{Knight};
use crate::model::pieces::queen::{Queen};
use crate::model::pieces::king::{King};
use crate::model::r#move::Move;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl PieceType {
    pub fn get_valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        match self {
            PieceType::Pawn(pawn) => pawn.get_valid_moves(board),
            PieceType::Rook(rook) => rook.get_valid_moves(board),
            PieceType::Knight(knight) => knight.get_valid_moves(board),
            PieceType::Bishop(bishop) => bishop.get_valid_moves(board),
            PieceType::Queen(queen) => queen.get_valid_moves(board),
            PieceType::King(king) => king.get_valid_moves(board),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
        }
    }

    // Getters
    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    // Setters
    pub fn set_piece_type(&mut self, piece_type: PieceType) {
        self.piece_type = piece_type;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // Methods
    pub fn to_string(&self) -> &'static str {
        match self.piece_type {
            PieceType::Pawn(_) => "Pawn",
            PieceType::Rook(_) => "Rook",
            PieceType::Knight(_) => "Knight",
            PieceType::Bishop(_) => "Bishop",
            PieceType::Queen(_) => "Queen",
            PieceType::King(_) => "King",
        }
    }

    pub fn to_notation(&self) -> &'static str {
        match self.piece_type {
            PieceType::Pawn(_) => "P",
            PieceType::Rook(_) => "R",
            PieceType::Knight(_) => "N",
            PieceType::Bishop(_) => "B",
            PieceType::Queen(_) => "Q",
            PieceType::King(_) => "K",

        }
    }
}
