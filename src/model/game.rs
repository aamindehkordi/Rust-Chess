use crate::model::board::Board;
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
        let mut piece = self.board.get_piece(from).expect("No piece at from");
        if piece.get_color() != self.board.get_current_player() {
            return false;
        }
        piece.calc_valid_moves(&self.board);
        piece.get_moves().contains(&to)

    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let mut piece = self.board.get_piece(from).expect("No piece at from");
        piece.execute_move(&mut self.board, from, to)?;
        self.board.change_current_player();
        Ok(())
    }
}
