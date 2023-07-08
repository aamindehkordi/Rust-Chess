use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

#[derive(Clone, PartialEq, Debug)]
pub struct King {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<(usize, usize)>,
    has_moved: bool,
    in_check: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "K"),
            Color::Black => write!(f, "k"),
        }
    }
}

impl Piece for King {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            moves: Vec::new(),
            has_moved: false,
            in_check: None,
            has_moves: None,
        }
    }
    fn calc_valid_moves(&mut self, board: &Board) {
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            if let Some(new_position) = self.get_new_position(self.position, direction) {
                let tile = board.get_tile(new_position);
                if tile.is_empty() || tile.get_piece().as_ref().map_or(false, |p| p.get_color() != self.color) {
                    self.moves.push(new_position);
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_moves(&self) -> &Vec<(usize, usize)> {
        &self.moves
    }

    fn get_type(&self) -> PieceType {
        PieceType::King
    }
}
