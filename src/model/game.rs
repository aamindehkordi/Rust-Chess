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
        let mut piece = self.board.get_piece(from.clone()).expect("No piece at from");

        move_generator.generate_moves(&mut piece, &mut self.board);
        move_validator.is_game_over(&self.board);

        let mv = piece.create_move(self.get_board(), to);

        // Check if move is legal
        if !move_validator.is_legal(&mv, &piece, &mut self.board) {
            return Err("Illegal move".into());
        }

        // Execute move
        self.board.move_piece(&from, &to);

        Ok(())
    }

    pub fn execute_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        let from = mv.get_from();
        self.board.get_piece(from.clone()).expect("No piece at from").execute(&mut self.board, mv);
        Ok(())
    }
}
