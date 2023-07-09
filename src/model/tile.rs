use crate::model::pieces::piece::{Color, Piece, PieceType};

pub struct Tile {
    pub(crate) position: (usize, usize),
    pub(crate) piece: Option<Box<dyn Piece>>,
    pub(crate) attacked_by: Vec<(usize, usize)>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && match (&self.piece, &other.piece) {
            (Some(this_piece), Some(other_piece)) => {
                this_piece.get_color() == other_piece.get_color()
                && this_piece.get_type() == other_piece.get_type()
                && this_piece.get_position() == other_piece.get_position()
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            position: self.position,
            piece: self.piece.as_ref().map(|piece| piece.clone_box()),
            attacked_by: self.attacked_by.clone(),
        }
    }
}

impl Tile {
    pub fn new(position: (usize, usize), piece: Option<Box<dyn Piece>>) -> Self {
        Self { position, piece, attacked_by: Vec::new() }
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

    pub fn attacked(&mut self, pos: (usize, usize)) {
        self.attacked_by.push(pos);
    }

}