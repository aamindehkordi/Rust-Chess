use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;
use crate::model::r#move::Move;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn {
    color: Color,
    first_move: Option<bool>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    can_en_passant: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Pawn {
    pub fn new(color: Color) -> Pawn {
        Self {
            color,
            first_move: None,
            can_take: None,
            takeable: None,
            can_en_passant: None,
            pinned: None,
            has_moves: None,
        }
    }
}

impl Move for Pawn {
    fn get_valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        // Calculate valid moves for a pawn
        // This will depend on the current state of the board and the pawn's rules for movement
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        // Check if the move is valid
        if !self.is_valid_move(board, from, to) {
            return Err("Invalid move");
        }

        // Execute the move
        // This will depend on the pawn's rules for movement
        // You might need to update the pawn's state here (e.g., if it's the pawn's first move)

        Ok(())
    }
}
