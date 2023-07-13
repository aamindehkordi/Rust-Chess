// src/model/game.rs
use crate::model::board::Board;
use crate::model::moves::r#move::Move;
use crate::model::moves::move_generator::MoveGenerator;
use crate::model::moves::move_validator::MoveValidator;
use std::error::Error;

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
        let move_validator = MoveValidator::new();
        let move_generator = MoveGenerator::new();
        let curr_player = self.board.get_current_player();
        let mut piece = self.board.get_piece(from.clone()).expect("No piece at from");
        if piece.get_color() != curr_player.clone() {
            return Err("Not your turn".into());
        }
        move_generator.generate_moves(&mut piece, &mut self.board);
        move_validator.is_game_over(&self.board);

        let mv = move_generator.get_move(&from, &to, &piece, &self.board);

        if !move_validator.is_legal(&mv, &piece, &mut self.board) {
            return Err("Illegal move".into());
        }
        piece.execute(&mut self.board, mv.clone());

        Ok(())
    }
}
