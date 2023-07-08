use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 4],
    moves: Vec<(usize, usize)>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "B"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 1), (1, -1), (1, 1)],
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
            let mut new_position = self.get_new_position(self.position, direction);
            while let Some(pos) = new_position {
                let tile = board.get_tile(pos);
                if tile.is_empty() {
                    self.moves.push(pos);
                } else if tile.get_piece().as_ref().map_or(false, |p| p.get_color() != &self.color) {
                    self.moves.push(pos);
                    break;
                } else {
                    break;
                }
                new_position = self.get_new_position(pos, direction);
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
