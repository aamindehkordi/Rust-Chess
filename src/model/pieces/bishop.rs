use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop {
    color: Color,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,

}

impl Bishop {
    pub fn new(color: Color) -> Bishop {
        Self {
            color,
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }

}

impl Move for Bishop {
    fn get_valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a bishop
        // This will depend on the current state of the board and the bishop's rules for movement
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        // Check if the move is valid
        if !self.is_valid_move(board, from, to) {
            return Err("Invalid move");
        }

        // Execute the move
        // This will depend on the bishop's rules for movement
        // You might need to update the bishop's state here (e.g., if it's the bishop's first move)

        Ok(())
    }
}