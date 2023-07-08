use crate::model::pieces::piece::{Color, Piece, PieceType};

// An abstract Tile class that represents a tile on the board with proper notation.
pub struct Tile {
    pub position: (usize, usize),
    pub piece: Option<Box<dyn Piece>>,
}

impl Tile {
    pub fn new(position: (usize, usize), piece: Option<Box<dyn Piece>>) -> Self {
        Self { position, piece }
    }

    // Getters
    pub fn get_piece(&self) -> &Option<Box<dyn Piece>> {
        &self.piece
    }

    pub fn get_position(&self) -> &(usize, usize) {
        &self.position
    }

    pub fn is_empty(&self) -> bool {
        match &self.piece {
            Some(_) => false,
            None => true,
        }
    }

    pub fn set_piece(&mut self, piece: Option<Box<dyn Piece>>) {
        self.piece = piece;
    }

}