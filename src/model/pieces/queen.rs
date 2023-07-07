use std::fmt::{Display, Formatter};
// pieces/queen.rs
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Queen {
    color: Color,
    position: (usize, usize),
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Queen {
    pub fn new(color: Color, position: (usize, usize)) -> Queen {
        Self {
            color,
            position,
            pinned: None,
            has_moves: None,
        }
    }

}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "Q"),
            Color::Black => write!(f, "q"),
        }
    }
}

impl Move for Queen {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a queen
        // This will depend on the current state of the board and the queen's rules for movement
        todo!()
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {

        Ok(())
    }
}