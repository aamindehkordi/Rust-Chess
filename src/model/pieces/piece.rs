use crate::model::board::Board;
use crate::model::tile::Tile;
use crate::model::pieces::pawn::{Pawn, Move as PawnMove};
use crate::model::pieces::rook::{Rook, Move as RookMove};
use crate::model::pieces::bishop::{Bishop, Move as BishopMove};
use crate::model::pieces::knight::{Knight, Move as KnightMove};
use crate::model::pieces::queen::{Queen, Move as QueenMove};
use crate::model::pieces::king::{King, Move as KingMove};



#[derive(Debug)]
pub enum Color {
    White,
    Black,
}
pub trait Move {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile>;
}

pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl Move for PieceType {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        match self {
            PieceType::Pawn(pawn) => pawn.get_valid_move_list(board),
            PieceType::Rook(rook) => rook.get_valid_move_list(board),
            PieceType::Knight(knight) => knight.get_valid_move_list(board),
            PieceType::Bishop(bishop) => bishop.get_valid_move_list(board),
            PieceType::Queen(queen) => queen.get_valid_move_list(board),
            PieceType::King(king) => king.get_valid_move_list(board),
        }
    }
}

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
