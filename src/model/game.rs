use crate::model::board::Board;
use crate::model::r#move::Move;
use std::error::Error;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        Self { board }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn valid_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        let mut piece = self.board.get_piece(from).expect("No piece at from").clone();
        piece.get_valid_moves(&self.board).contains(&to)
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let mut piece = self.board.get_piece(from).expect("No piece at from").clone();
        piece.execute_move(&mut self.board, from, to)?;
        Ok(())
    }
}
