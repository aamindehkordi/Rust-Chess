use std::fmt::Display;
use crate::model::board::Board;
use crate::model::tile::Tile;
use crate::model::pieces::pawn::{Pawn};
use crate::model::pieces::rook::{Rook};
use crate::model::pieces::bishop::{Bishop};
use crate::model::pieces::knight::{Knight};
use crate::model::pieces::queen::{Queen};
use crate::model::pieces::king::{King};
use crate::model::r#move::Move;


#[derive(Clone, PartialEq, Debug)]
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
    pub fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        match self {
            PieceType::Pawn(ref mut pawn) => pawn.get_valid_moves(board),
            PieceType::Rook(ref mut rook) => rook.get_valid_moves(board),
            PieceType::Knight(ref mut knight) => knight.get_valid_moves(board),
            PieceType::Bishop(ref mut bishop) => bishop.get_valid_moves(board),
            PieceType::Queen(ref mut queen) => queen.get_valid_moves(board),
            PieceType::King(ref mut king) => king.get_valid_moves(board),
        }

    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.piece_type {
            PieceType::Pawn(ref pawn) => write!(f, "{}", pawn),
            PieceType::Rook(ref rook) => write!(f, "{}", rook),
            PieceType::Knight(ref knight) => write!(f, "{}", knight),
            PieceType::Bishop(ref bishop) => write!(f, "{}", bishop),
            PieceType::Queen(ref queen) => write!(f, "{}", queen),
            PieceType::King(ref king) => write!(f, "{}", king),
        }
    }
}

impl Move for Piece {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.piece_type.get_valid_moves(board)
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        match self.piece_type {
            PieceType::Pawn(ref mut pawn) => pawn.execute_move(board, from, to),
            PieceType::Rook(ref mut rook) => rook.execute_move(board, from, to),
            PieceType::Knight(ref mut knight) => knight.execute_move(board, from, to),
            PieceType::Bishop(ref mut bishop) => bishop.execute_move(board, from, to),
            PieceType::Queen(ref mut queen) => queen.execute_move(board, from, to),
            PieceType::King(ref mut king) => king.execute_move(board, from, to),
        }

    }
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
}
