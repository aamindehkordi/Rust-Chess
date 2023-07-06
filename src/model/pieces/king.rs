// pieces/king.rs
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct King {
    color: Color,
    has_moved: bool,
    in_check: Option<bool>,
    has_moves: Option<bool>,
}

impl King {
    pub fn new(color: Color) -> King {
        Self {
            color,
            has_moved: false,
            in_check: None,
            has_moves: None,
        }
    }
}


impl Move for King {
    fn get_valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a king
        // This will depend on the current state of the board and the king's rules for movement
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        // Check if the move is valid
        if !self.is_valid_move(board, from, to) {
            return Err("Invalid move");
        }

        // Execute the move
        // This will depend on the king's rules for movement
        // You might need to update the king's state here (e.g., if it's the king's first move)

        Ok(())
    }
}

