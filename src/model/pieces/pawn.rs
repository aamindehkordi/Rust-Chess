use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn {
    color: Color,
    position: (usize, usize),
    first_move: Option<bool>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    can_en_passant: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Pawn {
    pub fn new(color: Color, position: (usize, usize)) -> Pawn {
        Self {
            color,
            position,
            first_move: None,
            can_take: None,
            takeable: None,
            can_en_passant: None,
            pinned: None,
            has_moves: None,
        }
    }
}

// Upper case for white, lower case for black
impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "P"),
            Color::Black => write!(f, "p"),
        }
    }
}

impl Move for Pawn {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a pawn
        // This will depend on the current state of the board and the pawn's rules for movement
        todo!()
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {

        Ok(())
    }
}
