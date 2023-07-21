// src/model/game.rs
use crate::model::board::Board;
use crate::model::moves::r#move::{MoveType};
use std::error::Error;
use crate::model::moves::move_validator::MoveValidator;
use crate::model::pieces::piece::{Piece, PieceType};
use crate::model::pieces::piece::Color;



pub struct Game {
    board: Board,
    current_turn: Color,
}

const STARTING_POSITION: &str = "r3k2r/2pppp1p/b4n1b/6p1/6P1/B4N1B/2PPPP1P/R3K2R w";

impl Game {
    pub fn new() -> Self {
        let board = Board::from_fen(STARTING_POSITION);
        Self { board, current_turn: Color::White }
    }

    pub fn get_board(&self) -> &Board { &self.board }

    pub fn promote(&mut self, from:(usize, usize), to: (usize, usize), piece_type: PieceType) {
        let mut piece = self.board.get_piece(from).unwrap();
        let mv = self.board.move_generator.create_promotion_move(&mut piece, to, piece_type);
        piece.execute(&mut self.board, mv.clone());
        self.board.move_generator.generate_moves_for_piece(&mut piece, &self.board);
        self.change_current_player();
        self.board.move_history.push(mv.to_history(piece.clone_box()));
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<Option<MoveType>, Box<dyn Error>> {
        if let Some(mut piece)  = self.board.get_piece(from){
            if piece.get_color() != self.current_turn.clone() {
                return Err("Not your turn".into());
            }

            let mv = self.board.move_generator.get_move(&from, &to, &piece, &self.board);
            let validator = MoveValidator::new(self.board.clone(), piece.clone_box(), mv.clone());

            if !validator.validate() {
                return Err("Illegal move".into());
            }

            if mv.get_move_type() == &MoveType::Promo {
                return Ok(Some(MoveType::Promo));
            }

            piece.execute(&mut self.board, mv.clone());
            self.change_current_player();
            self.board.move_history.push(mv.to_history(piece.clone_box()));
            self.board.get_all_possible_moves();

            if self.is_game_over() {
                return Err("Game Over".into());
            }
            Ok(None)
        } else {
            Err("No piece at from".into())
        }
    }

    pub fn is_game_over(&mut self) -> bool {
        // Logic to check if game is over
        let curr_player = self.board.get_current_player();
        if self.board.is_king_in_check(curr_player) {
            // Check if king has any legal moves
            let king = self.board.find_king(curr_player.clone());
            let king_moves = self.board.get_piece(king).unwrap().get_moves().clone();
            if king_moves.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn change_current_player(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        self.board.current_turn = self.current_turn.clone();
    }

}
