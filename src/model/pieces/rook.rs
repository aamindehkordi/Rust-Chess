use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::r#move::Move;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Rook {
    color: Color,
    first_move: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Rook {
    pub fn new(color: Color) -> Rook {
        Self {
            color,
            first_move: None,
            pinned: None,
            has_moves: None,
        }
    }

}

impl Move for Rook {
    fn get_valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a rook
        // This will depend on the current state of the board and the rook's rules for movement
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        // Check if the move is valid
        if !self.is_valid_move(board, from, to) {
            return Err("Invalid move");
        }

        // Execute the move
        // This will depend on the rook's rules for movement
        // You might need to update the rook's state here (e.g., if it's the rook's first move)

        Ok(())
    }
}