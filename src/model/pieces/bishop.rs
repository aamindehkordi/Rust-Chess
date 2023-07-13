use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::moves::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct Bishop {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 4],
    moves: Vec<Move>,
    piece_type: PieceType,
    can_take: bool,
    takeable: bool,
    pinned: bool,
    has_moves: bool,
}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "B"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Debug for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "B"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 1), (1, -1), (1, 1)],
            moves: Vec::new(),
            piece_type: PieceType::Bishop,
            can_take: false,
            takeable: false,
            pinned: false,
            has_moves: true,
        }
    }

    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    fn get_type(&self) -> PieceType {
        PieceType::Bishop
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }
    fn push_move(&mut self, mv: &mut Move){
        self.moves.push(mv.clone());
    }
}

impl Bishop {
    fn update_pinned(&mut self) {
        self.moves.clear();
        self.has_moves = false;
        self.takeable = true;
    }

    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        let from_tile = board.get_tile(self.position).clone();
        let to_tile = board.get_tile(new_position).clone();
        let mut mv_type = MoveType::Invalid;
        if to_tile.is_empty() { // Check if the tile the piece is moving to is empty.
            let mv_type = MoveType::Normal;
        } else {
            let mv_type = MoveType::Capture;
        };

        // Create a copy of the board and make the move on the copied board.
        let mut board_copy = board;
        board_copy.move_piece(&self.position, &new_position);

        // Only add the move if it wouldn't put the king in check.
        if !board_copy.is_king_in_check(&self.color) {
            let mut mv = Move::new(mv_type.clone(), self.position, new_position);
            mv.set_valid(true);
            self.moves.push(mv);
            self.has_moves = true;
            if mv_type == MoveType::Capture {
                self.can_take = true;
            }
        }
    }
}
