// src/model/game.rs
use crate::model::board::Board;
use crate::model::moves::r#move::Move;
use crate::model::moves::move_generator::MoveGenerator;
use crate::model::moves::move_validator::MoveValidator;
use std::error::Error;
use crate::model::pieces::piece::Piece;

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
        let move_generator = MoveGenerator::new();
        let curr_player = self.board.get_current_player();
        let mut piece = self.board.get_piece(from.clone());
        if Some(piece) {
            if piece.get_color() != curr_player.clone() {
                return Err("Not your turn".into());
            }
            move_generator.generate_moves(&mut piece, &mut self.board);
            self.is_game_over();

            let mv = move_generator.get_move(&from, &to, &piece, &self.board);

            if !self.is_legal(&mv, &piece) {
                return Err("Illegal move".into());
            }

            piece.execute(&mut self.board, mv.clone());
            Ok(())
        } else {
            Err("No piece at from".into())
        }
    }

    pub fn is_game_over(&self) -> bool {
        // Logic to check if game is over
        let curr_player = board.get_current_player();
        if self.board.is_king_in_check(curr_player) {
            // Check if checkmate
            if board.is_checkmate(curr_player) {
                return true;
            }
        }
        false
    }

    pub fn is_legal(&self, mv: &Move, piece: &Box<dyn Piece>) -> bool {
        if !mv.valid() {
            return false;
        }

        let destination = mv.get_to(); // Get the destination of the move
        let dest_piece = board.get_piece(destination.clone()); //  Get the piece at the destination
        // Check if the destination is empty or if the piece at the destination is of a different color
        if let Some(dest_piece) = dest_piece {
            if dest_piece.get_color() == piece.get_color() { // If the piece at the destination is of the same color
                return false; // The move is not legal
            }
        }
        // check if the king is in check after the move
        return board.temp_move_piece(&piece.get_position(), &destination);
        /*match piece.get_type() {
            PieceType::Pawn => self.is_legal_for_pawn(mv, piece, board),
            PieceType::Rook => self.is_legal_for_rook(mv, piece, board),
            PieceType::Knight => self.is_legal_for_knight(mv, piece, board),
            PieceType::Bishop => self.is_legal_for_bishop(mv, piece, board),
            PieceType::Queen => self.is_legal_for_queen(mv, piece, board),
            PieceType::King => self.is_legal_for_king(mv, piece, board),
        }*/
    }

}

