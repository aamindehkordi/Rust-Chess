use crate::model::board::Board;
use crate::model::moves::move_validator::MoveValidator;
use crate::model::moves::r#move::{Move, MoveType};
use crate::model::pieces::piece::{Piece, PieceType};

pub struct MoveGenerator {
    move_validator: MoveValidator,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            move_validator: MoveValidator::new(),
        }
    }

    pub fn generate_moves(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        match piece.get_type() {
            PieceType::Pawn => self.generate_moves_for_pawn(piece, board),
            PieceType::Rook => self.generate_moves_for_rook(piece, board),
            PieceType::Knight => self.generate_moves_for_knight(piece, board),
            PieceType::Bishop => self.generate_moves_for_bishop(piece, board),
            PieceType::Queen => self.generate_moves_for_queen(piece, board),
            PieceType::King => self.generate_moves_for_king(piece, board),
        }
    }

    pub fn get_move(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> Move {
        let piece_type = piece.get_type();
        let mut move_type = MoveType::Invalid;

        match piece_type {
            PieceType::Pawn => move_type = self.get_move_type_for_pawn(from, to, piece, board),
            PieceType::Rook => move_type = self.get_move_type_for_rook(from, to, piece, board),
            PieceType::Knight => move_type = self.get_move_type_for_knight(from, to, piece, board),
            PieceType::Bishop => move_type = self.get_move_type_for_bishop(from, to, piece, board),
            PieceType::Queen => move_type = self.get_move_type_for_queen(from, to, piece, board),
            PieceType::King => move_type = self.get_move_type_for_king(from, to, piece, board),
        }

        let mut mv = Move::new(move_type.clone(), from.clone(), to.clone());
        if move_type != MoveType::Invalid {
            mv.set_valid(true);
        }
        mv
    }

    fn get_move_type_for_pawn(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        // Logic to get the move type for a pawn
        todo!()
    }

    fn generate_moves_for_pawn(&self, piece: &Box<dyn Piece>, board: &Board) -> Vec<Move> {
        // Logic to generate moves for a pawn
        todo!()
    }

    fn get_move_type_for_rook(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        // Logic to get the move type for a rook
        todo!()
    }

    fn generate_moves_for_rook(&self, piece: &Box<dyn Piece>, board: &Board) -> Vec<Move> {
        // Logic to generate moves for a rook
        todo!()
    }

    fn get_move_type_for_knight(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let mut move_type = MoveType::Invalid; // Create a variable to store the move type
        let directions = piece.get_directions(); // Get the directions in which the piece can move
        let new_pos = (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32); // Calculate the new position

        // Loop through the directions
        for &direction in directions {
            if direction == new_pos { // Check if the new position is in the directions
                move_type = MoveType::Normal; // Set the move type to normal
                break; // Break out of the loop
            }
        }

        if move_type == MoveType::Invalid { // Check if the move type is invalid
            if let Some(dest_piece) = board.get_piece(to.clone()) { // Get the piece at the new position
                if dest_piece.get_color() != piece.get_color() { // Check if the piece at the new position is of a different color
                    move_type = MoveType::Capture; // Set the move type to capture
                }
            }
        }

        move_type // Return the move type

    }

    fn generate_moves_for_knight(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new(); // Create a vector to store the moves
        let directions = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]; // Create an array of tuples to store the directions in which a knight can move
        // Loop through the directions
        for &direction in &directions {
            let new_pos = (piece.get_position().0 as i32 + direction.0, piece.get_position().1 as i32 + direction.1); // Calculate the new position
            if new_pos.0 >= 0 && new_pos.0 < 8 && new_pos.1 >= 0 && new_pos.1 < 8 { // Check if the new position is on the board
               if let Some(dest_piece) = board.get_piece((new_pos.0 as usize, new_pos.1 as usize)){ // Get the piece at the new position
                   if dest_piece.get_color() != piece.get_color() { // Check if the piece at the new position is of a different color
                       piece.push_move(&mut Move::new(MoveType::Capture, piece.get_position(), (new_pos.0 as usize, new_pos.1 as usize))); // Add a capture move to the vector
                   }
               } else {
                   piece.push_move(&mut Move::new(MoveType::Normal, piece.get_position(), (new_pos.0 as usize, new_pos.1 as usize))); // Add the move to the vector
               }
            }
        }
        moves
    }

    fn get_move_type_for_bishop(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        // Logic to get the move type for a bishop
        todo!()
    }

    fn generate_moves_for_bishop(&self, piece: &Box<dyn Piece>, board: &Board) -> Vec<Move> {
        // Logic to generate moves for a bishop
        todo!()
    }

    fn get_move_type_for_queen(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        // Logic to get the move type for a queen
        todo!()
    }

    fn generate_moves_for_queen(&self, piece: &Box<dyn Piece>, board: &Board) -> Vec<Move> {
        // Logic to generate moves for a queen
        todo!()
    }

    fn get_move_type_for_king(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        // Logic to get the move type for a king
        todo!()
    }

    fn generate_moves_for_king(&self, piece: &Box<dyn Piece>, board: &Board) -> Vec<Move> {
        // Logic to generate moves for a king
        todo!()
    }
}
