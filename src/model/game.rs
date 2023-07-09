use crate::model::board::Board;
use std::error::Error;
use crate::model::pieces::piece::PieceType;
use crate::model::r#move::{Move, MoveType};

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

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let piece = self.board.get_piece(from).expect("No piece at from");
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
