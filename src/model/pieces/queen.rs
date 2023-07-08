use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

#[derive(Clone, PartialEq, Debug)]
pub struct Queen {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<(usize, usize)>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "Q"),
            Color::Black => write!(f, "q"),
        }
    }
}

impl Piece for Queen {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            moves: Vec::new(),
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            let mut new_position = self.get_new_position(self.position, direction);
            while let Some(pos) = new_position {
                let tile = board.get_tile(pos);
                if tile.is_empty() {
                    self.moves.push(pos);
                    new_position = self.get_new_position(pos, direction);
                } else {
                    if tile.get_piece().as_ref().map_or(false, |p| p.get_color() != &self.color) {
                        self.moves.push(pos);
                    }
                    break;
                }
            }
        }
        self.moves.clone()
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
