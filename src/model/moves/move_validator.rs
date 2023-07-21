use crate::model::board::Board;
use crate::model::moves::r#move::{CastleType, Move, MoveType};
use crate::model::pieces::piece::{Color, Piece, PieceType};

pub struct MoveValidator {
    board: Board,
    from_piece: Box<dyn Piece>,
    mv: Move,
    from_pos: (usize, usize),
    to_pos: (usize, usize),
    to_piece: Option<Box<dyn Piece>>,
}

impl MoveValidator {
    pub fn new(
        board: Board,
        piece: Box<dyn Piece>,
        mv: Move,
    ) -> Self {
        Self {
            board: board.clone(),
            from_piece: piece,
            mv: mv.clone(),
            from_pos: *mv.get_from(),
            to_pos: *mv.get_to(),
            to_piece: board.get_piece(*mv.get_to()),
        }
    }

    pub fn get_piece(&self) -> &Box<dyn Piece> {
        &self.from_piece
    }

    pub fn get_move_type(&self) -> &MoveType {
        self.mv.get_move_type()
    }

    pub fn get_from(&self) -> &(usize, usize) {
        self.mv.get_from()
    }

    pub fn get_to(&self) -> &(usize, usize) {
        self.mv.get_to()
    }

    pub fn validate(&self) -> bool {
        if !self.mv.valid() {
            println!("Invalid move");
            return false;
        }

        // check if the move results in check
        if self.results_in_check() {
            println!("Move results in check");
            return false;
        }

        // Check if the piece is moving to a valid square
        if !self.from_piece.is_valid_move(&self.from_pos, &self.to_pos) {
            println!("Move not valid for piece");
            return false;
        }

        // check if the destination square is occupied by a piece of the same color
        if let Some(piece) = &self.to_piece {
            if piece.get_color() == self.from_piece.get_color() {
                println!("Destination square is occupied by a piece of the same color");
                return false;
            }
            if self.mv.get_move_type() == &MoveType::EnPassant && piece.get_type() != PieceType::Pawn {
                println!("??");
                return false;
            }
            if self.mv.get_move_type().is_promotion() && piece.get_type() != PieceType::Pawn {
                println!("Promotion is only valid for pawns");
                return false;
            }
            if self.mv.get_move_type().is_castle() {
                if piece.get_type() != PieceType::Rook {
                    println!("Not Empty for castling");
                    return false;
                }
                return false;
            }
            if self.mv.get_move_type() == &MoveType::Normal {
                println!("Not Empty");
                return false;
            }
        } // else if the destination square is empty
        else if self.mv.get_move_type() == &MoveType::Capture {
            println!("Empty for capture");
            return false;
        }

        match self.mv.get_move_type() {
            &MoveType::Capture => self.is_valid_capture(),
            &MoveType::DoublePush => self.is_valid_double_push(),
            &MoveType::EnPassant => self.is_valid_en_passant(),
            &MoveType::Castle(_) => self.is_valid_castle(),
            &MoveType::Promo => self.is_valid_promo(),
            _ => true
        }
    }

    fn results_in_check(&self) -> bool {
        self.board.temp_move_piece(&self.from_pos, &self.to_pos)
    }

    fn has_moved(&self) -> bool {
        if !self.board.move_history.is_empty() {
                for mov in &self.board.move_history {
                    if mov.get_piece().get_type() == PieceType::Pawn && mov.get_piece().get_position().0 == self.from_piece.get_position().0 {
                        println!("Piece has moved");
                        return true;
                    }
                }
            }
        false
    }

    fn is_valid_capture(&self) -> bool {
        if let Some(piece) = &self.to_piece {
            if piece.get_color() == self.from_piece.get_color() {
                return false;
            }
        }

        true
    }

    fn is_valid_double_push(&self) -> bool {
        // check if the piece is a pawn
        if self.from_piece.get_type() != PieceType::Pawn {
            println!("Not a pawn");
            return false;
        }

        // check if the piece has moved
        if self.has_moved() {
            println!("Piece has moved");
            return false;
        }

        // check if the piece is moving two squares
        if self.to_pos.0 > self.from_pos.0 {
            if self.to_pos.0 - self.from_pos.0 != 2 {
                println!("Not moving two squares");
                return false;
            }
        } else if self.from_pos.0 - self.to_pos.0 != 2 {
            println!("Not moving two squares");
            return false;
        }

        // check if the piece is moving forward
        if self.to_pos.1 != self.from_pos.1 {
            println!("Not moving forward");
            return false;
        }

        true
    }

    fn is_valid_en_passant(&self) -> bool {
        // check if the piece is a pawn
        if self.from_piece.get_type() != PieceType::Pawn {
            println!("Not a pawn");
            return false;
        }

        // check if the last move was a pawn moving two squares
        if self.board.move_history.is_empty() {
            println!("No moves");
            return false;
        }
        let last_move = self.board.move_history.last().unwrap();
        if last_move.get_move_type() != &MoveType::DoublePush {
            println!("Last move was not a double push");
            return false;
        }
        let last_move_to = last_move.get_to();
        // check if the last move was next to the current push
        if last_move_to.0 != self.from_pos.0 || last_move_to.1 != self.to_pos.1 {
            println!("Last move was not next to the current push");
            return false;
        }
        // check if the last move was a pawn
        let last_move_piece = last_move.get_piece();
        if last_move_piece.get_type() != PieceType::Pawn {
            println!("Last move was not a pawn");
            return false;
        }
        // check if the last move was the opposite color
        if last_move_piece.get_color() == self.from_piece.get_color() {
            println!("Last move was not the opposite color");
            return false;
        }

        true
    }

    fn is_valid_castle(&self) -> bool {
        // check if the piece is a king
        if self.from_piece.get_type() != PieceType::King {
            println!("Not a king");
            return false;
        }

        // get the file of the rook
        let file = match self.mv.get_move_type() {
            &MoveType::Castle(CastleType::Kingside) => 7,
            &MoveType::Castle(CastleType::Queenside) => 0,
            _ => return false
        } as usize;


        // Check if the king or rook have moved in the move history
        if !self.board.move_history.is_empty() {
            for mv in &self.board.move_history {
                if mv.get_piece().get_type() == PieceType::King && mv.get_piece().get_color() == self.from_piece.get_color() {
                    println!("King has moved");
                    return false;
                }
                if mv.get_piece().get_type() == PieceType::Rook {
                    // check if the rook is on the H file
                    if mv.get_from().0 == file && mv.get_piece().get_color() == self.from_piece.get_color() {
                        println!("Rook has moved");
                        return false;
                    }
                }

            }
        }

        // Check if the king will move into check or through a square that is attacked
        let traveling_squares = match self.mv.get_move_type() {
            &MoveType::Castle(CastleType::Kingside) => vec![(self.from_pos.0,file -1), (self.from_pos.0,file -2)],
            &MoveType::Castle(CastleType::Queenside) => vec![(self.from_pos.0,file +1), (self.from_pos.0,file +2), (self.from_pos.0,file +3)],
            _ => return false
        };
        for square in &traveling_squares {
            let (is_square_attacked, _) = self.board.is_square_attacked(*square, self.from_piece.get_color());
            if is_square_attacked {
                println!("King will move into check");
                return false;
            }
        }


        true
    }

    fn is_valid_promo(&self) -> bool {
        // check if the piece is a pawn
        if self.from_piece.get_type() != PieceType::Pawn {
            println!("Not a pawn");
            return false;
        }

        // check if the piece is on the last rank
        if self.from_piece.get_color() == Color::White {
            if self.from_pos.1 != 6 {
                println!("Not on the last rank");
                return false;
            }
        } else if self.from_pos.1 != 1 {
            println!("Not on the last rank");
            return false;
        }

        // check if the piece is moving forward
        if self.to_pos.1 != self.from_pos.1 {
            println!("Not moving forward");
            return false;
        }

        true
    }
}