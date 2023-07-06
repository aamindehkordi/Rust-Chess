use crate::model::pieces::piece::{Color, Piece, PieceType};

// An abstract Tile class that represents a tile on the board with proper notation.
pub struct Tile {
    pub position: (usize, usize),
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn new(position: (usize, usize), piece: Option<Piece>) -> Self {
        Self { position, piece }
    }

    // Getters
    pub fn get_piece(&self) -> &Option<Piece> {
        &self.piece
    }

    pub fn get_position(&self) -> &(usize, usize) {
        &self.position
    }

    // Setters
    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }

    pub fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    // Methods
    pub fn to_string(&self) -> String {
        match &self.piece {
            Some(piece) => match piece.get_color() {
                Color::White => match piece.get_piece_type() {
                    PieceType::Pawn(_) => "♙",
                    PieceType::Rook(_) => "♖",
                    PieceType::Knight(_) => "♘",
                    PieceType::Bishop(_) => "♗",
                    PieceType::Queen(_) => "♕",
                    PieceType::King(_) => "♔",
                },
                Color::Black => match piece.get_piece_type() {
                    PieceType::Pawn(_) => "♟",
                    PieceType::Rook(_) => "♜",
                    PieceType::Knight(_) => "♞",
                    PieceType::Bishop(_) => "♝",
                    PieceType::Queen(_) => "♛",
                    PieceType::King(_) => "♚",
                },
            },
            None => " ",
        }.to_string()
    }

    pub fn is_empty(&self) -> bool {
        match &self.piece {
            Some(_) => false,
            None => true,
        }
    }
}