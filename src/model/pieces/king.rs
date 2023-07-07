use std::fmt::Display;
// pieces/king.rs
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct King {
    color: Color,
    position: (usize, usize),
    has_moved: bool,
    in_check: Option<bool>,
    has_moves: Option<bool>,
}

impl King {
    pub fn new(color: Color, position: (usize, usize)) -> King {
        Self {
            color,
            position,
            has_moved: false,
            in_check: None,
            has_moves: None,
        }
    }
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "K"),
            Color::Black => write!(f, "k"),
        }
    }
}


impl Move for King {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a king
        // This will depend on the current state of the board and the king's rules for movement
        todo!()
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {

        Ok(())
    }
}

