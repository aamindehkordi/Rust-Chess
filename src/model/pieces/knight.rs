use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

#[derive(Clone, PartialEq, Debug)]
pub struct Knight {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<(usize, usize)>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "N"),
            Color::Black => write!(f, "n"),
        }
    }
}
impl Piece for Knight {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
            moves: Vec::new(),
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }
    fn calc_valid_moves(&mut self, board: &Board) {
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            if let Some(new_position) = self.get_new_position(self.position, direction) {
                let tile = board.get_tile(new_position);
                if tile.is_empty() || tile.get_piece().as_ref().map_or(false, |p| p.get_color() != &self.color) {
                    self.moves.push(new_position);
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_moves(&self) -> &Vec<(usize, usize)> {
        &self.moves
    }
}

