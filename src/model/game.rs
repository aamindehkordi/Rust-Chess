// src/model/game.rs
use crate::model::board::Board;
use crate::model::moves::r#move::{Move, MoveHistory, MoveType};
use crate::model::moves::move_generator::MoveGenerator;
use std::error::Error;
use crate::model::moves::r#move::CastleType::{Kingside, Queenside};
use crate::model::pieces::piece::{Piece, PieceType};
use crate::model::pieces::piece::Color;



pub struct Game {
    board: Board,
    current_turn: Color,
    white_king: (usize, usize),
    black_king: (usize, usize),
    move_history: Vec<MoveHistory>,
    move_generator: MoveGenerator,
}

const STARTING_POSITION: &str = "rnbqk2r/pppppPpp/8/8/1p6/8/PPpPPPPP/R3KBNR w";

impl Game {
    pub fn new() -> Self {
        let board = Board::from_fen(STARTING_POSITION);
        let move_generator = MoveGenerator::new();
        Self { board, current_turn: Color::White, white_king: (0, 4), black_king: (7, 4), move_history: Vec::new(), move_generator }
    }

    pub fn get_board(&self) -> &Board { &self.board }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(Option<MoveType>), Box<dyn Error>> {
        if let Some(mut piece)  = self.board.get_piece(from.clone()){
            if piece.get_color() != self.current_turn.clone() {
                return Err("Not your turn".into());
            }
            self.move_generator.generate_moves(&mut piece, &mut self.board);
            self.is_game_over();

            let mv = self.move_generator.get_move(&from, &to, &piece, &self.board);

            if !self.is_legal(&mv, &piece) {
                return Err("Illegal move".into());
            }
            if mv.get_move_type() == &MoveType::Promo {
                return Ok(Some(MoveType::Promo));
            }

            piece.execute(&mut self.board, mv.clone());
            self.change_current_player();
            self.move_history.push(mv.to_history(piece.clone_box()));
            Ok((None))
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
        let current_position = piece.get_position();
        let destination = mv.get_to(); // Get the destination of the move
        let dest_piece = self.board.get_piece(destination.clone()); //  Get the piece at the destination



        // Check if the move is a double push
        if mv.get_move_type() == &MoveType::DoublePush {

            // Check if the piece is a pawn
            if piece.get_type() != PieceType::Pawn {
                return false;
            }

            // Check if the pawn has moved in the move history
            if self.move_history.len() > 0 {
                for mv in self.move_history.iter() {
                    if mv.get_piece().get_type() == PieceType::Pawn {
                        if mv.get_piece().get_position().0 == piece.get_position().0 {
                            return false;
                        }
                    }
                }
            }

            // Check if the pawn is moving two squares
            if destination.0 - current_position.0 != 2 {
                return false;
            }

            // Check if the pawn is moving forward
            if destination.1 != current_position.1 {
                return false;
            }

            return true;
        }

        // Check if the move is en passant
        if mv.get_move_type() == &MoveType::EnPassant {
            // check if the last move was a pawn moving two squares
            if self.move_history.len() < 1 {
                return false;
            }
            let last_move = self.move_history.last().unwrap();
            if last_move.get_move_type() != &MoveType::DoublePush {
                return false;
            }
            let last_move_to = last_move.get_to();
            // check if the last move was next to the current push
            if last_move_to.0 != current_position.0 || last_move_to.1 != destination.1 {
                return false;
            }
            // check if the last move was a pawn
            let last_move_piece = last_move.get_piece();
            if last_move_piece.get_type() != PieceType::Pawn {
                return false;
            }
            // check if the last move was the opposite color
            if last_move_piece.get_color() == piece.get_color() {
                return false;
            }

            return true;
        }

        // Check if the move is castling Kingside
        if mv.get_move_type() == &MoveType::Castle(Kingside) {
            // Check if the king is in check
            if self.board.is_king_in_check(&piece.get_color()) {
                return false;
            }

            let king = self.board.get_piece(piece.get_position().clone()).unwrap();
            let rook = self.board.get_piece((piece.get_position().0, 7)).unwrap();


            // Check if the king or rook have moved in the move history
            if self.move_history.len() > 0 {
                for mv in self.move_history.iter() {
                    if mv.get_piece().get_type() == PieceType::King {
                        if mv.get_piece().get_color() == piece.get_color() {
                            return false;
                        }
                    }

                }
            }

            // Check if the king will move into check or through a square that is attacked
            let travelling_squares = [(piece.get_position().0, piece.get_position().1 + 1), (piece.get_position().0, piece.get_position().1 + 2), destination.clone()];
            for square in travelling_squares.iter() {
                if self.board.is_square_attacked(square.clone(), piece.get_color().clone()) {
                    return false;
                }
            }

            return true;
        }

        // Check if the move is castling Queenside
        if mv.get_move_type() == &MoveType::Castle(Queenside) {
            // Check if the king is in check
            if self.board.is_king_in_check(&piece.get_color()) {
                return false;
            }

            let king = self.board.get_piece(piece.get_position().clone()).unwrap();
            let rook = self.board.get_piece((piece.get_position().0, 0)).unwrap();

            // Check if the king or rook have moved in the move history
            if self.move_history.len() > 0 {
                for mv in self.move_history.iter() {
                    if mv.get_piece().get_type() == PieceType::King {
                        if mv.get_piece().get_color() == piece.get_color() {
                            return false;
                        }
                    }

                }
            }

            // Check if the king will move into check or through a square that is attacked
            let travelling_squares = [(piece.get_position().0, piece.get_position().1 - 1), (piece.get_position().0, piece.get_position().1 - 2), (piece.get_position().0, piece.get_position().1-3)];
            for square in travelling_squares.iter() {
                if self.board.is_square_attacked(square.clone(), piece.get_color().clone()) {
                    return false;
                }
            }

            return true;
        }

        // Check if the move is a promotion
        if mv.get_move_type() == &MoveType::Promo {
            // Check if the piece is a pawn
            if piece.get_type() != PieceType::Pawn {
                return false;
            }

            // Check if the pawn is moving forward
            if destination.0 != current_position.0 {
                return false;
            }

            // Check if the pawn is moving to the last rank
            if piece.get_color() == Color::White {
                if destination.1 != 7 {
                    return false;
                }
            } else {
                if destination.1 != 0 {
                    return false;
                }
            }

            return true;
        }


        // Check if the destination is empty or if the piece at the destination is of a different color
        if let Some(dest_piece) = dest_piece {
            if dest_piece.get_color() == piece.get_color() { // If the piece at the destination is of the same color
                return false;
            }

            // Check if the move is a capture
            if mv.get_move_type() != (&MoveType::Capture) {
                return false;
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
