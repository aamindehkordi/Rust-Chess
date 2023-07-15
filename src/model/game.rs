// src/model/game.rs
use crate::model::board::Board;
use crate::model::moves::r#move::{Move, MoveType};
use crate::model::moves::move_generator::MoveGenerator;
use std::error::Error;
use crate::model::pieces::piece::Piece;
use crate::model::pieces::piece::Color;



pub struct Game {
    board: Board,
    current_turn: Color,
    white_king: (usize, usize),
    black_king: (usize, usize),
    move_history: Vec<MoveHistory>,
    move_generator: MoveGenerator,
}

const STARTING_POSITION: &str = "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w";

impl Game {
    pub fn new() -> Self {
        let board = Board::from_fen(STARTING_POSITION);
        Self { board, current_turn: Color::White, white_king: (0, 4), black_king: (7, 4) }
    }

    pub fn get_board(&self) -> &Board { &self.board }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let move_generator = MoveGenerator::new();
        if let Some(mut piece)  = self.board.get_piece(from.clone()){
            if piece.get_color() != self.current_turn.clone() {
                return Err("Not your turn".into());
            }
            move_generator.generate_moves(&mut piece, &mut self.board);
            self.is_game_over();

            let mv = move_generator.get_move(&from, &to, &piece, &self.board);

            if !self.is_legal(&mv, &piece) {
                return Err("Illegal move".into());
            }
            if mv.get_move_type() == &MoveType::Promo {
                return Ok(Some(MoveType::Promo));
            }

            piece.execute(&mut self.board, mv.clone());
            self.change_current_player();
            Ok(())
        } else {
            Err("No piece at from".into())
        }
    }

    pub fn promote(&mut self, from:(usize, usize), to: (usize, usize), piece_type: PieceType) {
        let mut piece = self.board.get_piece(from).unwrap();
        let mv = self.move_generator.create_promotion_move(&mut piece, to, piece_type);
        piece.execute(&mut self.board, mv.clone());
        self.change_current_player();
        self.move_history.push(mv.to_history(piece.clone_box()));
    }

    pub fn is_legal(&self, mv: &Move, piece: &Box<dyn Piece>) -> bool {
        if !mv.valid() {
            return false;
        }

        let destination = mv.get_to(); // Get the destination of the move
        let dest_piece = self.board.get_piece(destination.clone()); //  Get the piece at the destination
        // Check if the destination is empty or if the piece at the destination is of a different color
        if let Some(dest_piece) = dest_piece {
            if dest_piece.get_color() == piece.get_color() { // If the piece at the destination is of the same color
                return false; // The move is not legal
            }
        }
        // check if the king is in check after the move
        return self.board.temp_move_piece(&piece.get_position(), &destination);
    }

    pub fn is_game_over(&self) -> bool {
        // Logic to check if game is over
        let curr_player = self.board.get_current_player();
        if self.board.is_king_in_check(curr_player) {
            // Check if checkmate
            if self.board.is_king_trapped(curr_player) {
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
