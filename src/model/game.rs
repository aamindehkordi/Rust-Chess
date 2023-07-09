use crate::model::board::Board;
use std::error::Error;

use crate::model::r#move::{Move};

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        Self { board }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let mut piece = self.board.get_piece(from).expect("No piece at from");
        piece.update_moves(self.get_board().clone());
        let mv = piece.create_move(self.get_board(), to);

        self.execute_move(mv)?;
        self.board.change_current_player();
        Ok(())
    }

    pub fn execute_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        let from = mv.get_from().position;
        self.board.get_piece(from).expect("No piece at from").execute(&mut self.board, mv);
        Ok(())
    }
}
