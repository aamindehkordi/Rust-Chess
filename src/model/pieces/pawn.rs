use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::moves::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct Pawn {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 3],
    moves: Vec<Move>,
    piece_type: PieceType,
    first_move: bool,
    can_take: bool,
    takeable: bool,
    pinned: bool,
    has_moves: bool,
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "P"),
            Color::Black => write!(f, "p"),
        }
    }
}

impl Debug for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "P"),
            Color::Black => write!(f, "p"),
        }
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (usize, usize)) -> Self {
        let directions = match color {
            Color::White => [(1, 0), (1, -1), (1, 1)],
            Color::Black => [(-1, 0), (-1, -1), (-1, 1)],
        };
        Self {
            color,
            position,
            directions,
            moves: Vec::new(),
            piece_type: PieceType::Pawn,
            first_move: true,
            can_take: false,
            takeable: false,
            pinned: false,
            has_moves: true,
        }
    }

    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to(); // Get the destination of the move
        let mut this = board.pick_up_piece(&self.position).unwrap(); // Pick up the piece at the current position

        // Check if this is the piece we want to move
        if this.get_color() == self.color && this.get_type() == self.piece_type && this.get_position() == &self.position {
            match mv.get_move_type() {
                MoveType::Normal | MoveType::Promo | MoveType::DoublePush=> { // If the move is a normal move
                    board.move_piece(&self.position, to_position);
                },
                MoveType::Capture => { // If the move is a capture move
                    board.move_piece(&self.position, to_position);
                    board.take_piece(mv.get_to());
                },
                MoveType::EnPassant => { // If the move is an en passant
                    // get the position of the pawn that is being taken which is above or below the pawn that is moving
                    let taken_position = match self.color {
                        Color::White => (to_position.0 - 1, to_position.1),
                        Color::Black => (to_position.0 + 1, to_position.1),
                    };
                    board.move_piece(&self.position, to_position); // Move the pawn
                    board.take_piece(&taken_position); // Take the pawn that is being taken
                },
                MoveType::Promotion(_) => { // If the move is a promotion
                    board.move_piece(&self.position, to_position); // Move the pawn
                    board.take_piece(mv.get_to()); // Take the piece that is being taken
                    let mut new_piece: Box<dyn Piece> = match mv.get_promotion() {
                        PieceType::Queen => Box::new(crate::model::pieces::queen::Queen::new(self.color.clone(), to_position.clone())),
                        PieceType::Rook => Box::new(crate::model::pieces::rook::Rook::new(self.color.clone(), to_position.clone())),
                        PieceType::Bishop => Box::new(crate::model::pieces::bishop::Bishop::new(self.color.clone(), to_position.clone())),
                        PieceType::Knight => Box::new(crate::model::pieces::knight::Knight::new(self.color.clone(), to_position.clone())),
                        _ => Box::new(crate::model::pieces::pawn::Pawn::new(self.color.clone(), to_position.clone())),
                    };
                    new_piece.set_position(to_position.clone()); // Set the position of the new piece
                    board.put_down_piece(&to_position, Some(new_piece)); // Put down the new piece
                },
                MoveType::PromoteAndCapture(_) => { // If the move is a promotion and capture
                    board.move_piece(&self.position, to_position); // Move the pawn
                    board.take_piece(mv.get_to()); // Take the piece that is being taken
                    let mut new_piece: Box<dyn Piece> = match mv.get_promotion() {
                        PieceType::Queen => Box::new(crate::model::pieces::queen::Queen::new(self.color.clone(), to_position.clone())),
                        PieceType::Rook => Box::new(crate::model::pieces::rook::Rook::new(self.color.clone(), to_position.clone())),
                        PieceType::Bishop => Box::new(crate::model::pieces::bishop::Bishop::new(self.color.clone(), to_position.clone())),
                        PieceType::Knight => Box::new(crate::model::pieces::knight::Knight::new(self.color.clone(), to_position.clone())),
                        _ => Box::new(crate::model::pieces::pawn::Pawn::new(self.color.clone(), to_position.clone())),
                    };
                    new_piece.set_position(to_position.clone()); // Set the position of the new piece
                    board.put_down_piece(&to_position, Some(new_piece)); // Put down the new piece
                },
                _ => {},
            }
            self.position = *to_position;
            this.set_position(*to_position);
            board.put_down_piece(&self.position, Some(this));
            self.first_move = false;
        }
    }
    // ...
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn get_position(&self) -> &(usize, usize) {
        &self.position
    }

    fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn get_directions(&self) -> &[(i32, i32)] { &self.directions }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn push_move(&mut self, mv: &mut Move){
        self.moves.push(mv.clone());
    }
}

