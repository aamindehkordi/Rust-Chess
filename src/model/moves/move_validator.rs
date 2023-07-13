use crate::model::board::Board;
use crate::model::moves::r#move::{Move, MoveType};
use crate::model::pieces::piece::{Piece, PieceType};

pub struct MoveValidator {}

impl MoveValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_game_over(&self, board: &Board) -> bool {
        // Logic to check if game is over
        let curr_player = board.get_current_player();
        if board.is_king_in_check(curr_player) {
            // Check if checkmate
            if board.is_checkmate(curr_player) {
                return true;
            }
        }
        false
    }

    pub fn is_legal(&self, mv: &Move, piece: &Box<dyn Piece>, board: &mut Board) -> bool {
        match piece.get_type() {
            PieceType::Pawn => self.is_legal_for_pawn(mv, piece, board),
            PieceType::Rook => self.is_legal_for_rook(mv, piece, board),
            PieceType::Knight => self.is_legal_for_knight(mv, piece, board),
            PieceType::Bishop => self.is_legal_for_bishop(mv, piece, board),
            PieceType::Queen => self.is_legal_for_queen(mv, piece, board),
            PieceType::King => self.is_legal_for_king(mv, piece, board),
        }
    }

    fn is_legal_for_pawn(&self, mv: &Move, piece: &Box<dyn Piece>, board: &Board) -> bool {
        // Logic to validate moves for a pawn
        todo!()
    }

    fn is_legal_for_rook(&self, mv: &Move, piece: &Box<dyn Piece>, board: &Board) -> bool {
        // Logic to validate moves for a rook
        todo!()
    }

    fn is_legal_for_knight(&self, mv: &Move, piece: &Box<dyn Piece>, board: &mut Board) -> bool {
        if mv.get_move_type().is_valid() {
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
        // Get the current position of the piece
        return board.temp_move_piece(&piece.get_position(), &destination);
    }

    fn is_legal_for_bishop(&self, mv: &Move, piece: &Box<dyn Piece>, board: &Board) -> bool {
        // Logic to validate moves for a bishop
        todo!()
    }

    fn is_legal_for_queen(&self, mv: &Move, piece: &Box<dyn Piece>, board: &Board) -> bool {
        // Logic to validate moves for a queen
        todo!()
    }

    fn is_legal_for_king(&self, mv: &Move, piece: &Box<dyn Piece>, board: &Board) -> bool {
        // Logic to validate moves for a king
        todo!()
    }
}
