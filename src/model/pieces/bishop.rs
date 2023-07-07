use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop {
    color: Color,
    position: (usize, usize),
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

impl Bishop {
    pub fn new(color: Color, position: (usize, usize)) -> Bishop {
        Self {
            color,
            position,
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }

}

impl Move for Bishop {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a bishop
        // This will depend on the current state of the board and the bishop's rules for movement
        todo!()
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        Ok(())
    }
}