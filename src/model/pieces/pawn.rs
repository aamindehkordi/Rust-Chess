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
        let to_position = mv.get_to();
        let mut this = board.pick_up_piece(&self.position).unwrap();

        if this.get_color() == self.color && this.get_type() == self.piece_type && this.get_position() == &self.position {
            match mv.get_move_type() {
                MoveType::Normal | MoveType::DoublePush => {
                    board.move_piece(&self.position, to_position);
                },
                MoveType::Capture => {
                    board.move_piece(&self.position, to_position);
                    board.take_piece(mv.get_to());
                },
                _ => {},
            }
            self.position = *to_position;
            this.set_position(*to_position);
            board.put_down_piece(&self.position, Some(this));
            self.first_move = false;
            board.change_current_player();
        }
    }

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

impl Pawn {
    fn update_pinned(&mut self) {
        self.moves.clear();
        self.has_moves = false;
        self.takeable = true;
    }

    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        let from_tile = board.get_tile(self.position).clone();
        let to_tile = board.get_tile(new_position).clone();
        let mut mv_type = MoveType::Invalid;
        if to_tile.is_empty() {
            if self.first_move && ((self.position.0 as i32 - new_position.0 as i32).abs() == 2) {
                mv_type = MoveType::DoublePush;
            } else {
                mv_type = MoveType::Normal;
            }
        } else if self.position.1 as i32 - new_position.1 as i32 != 0 {
            mv_type = MoveType::Capture;
        } else {
            mv_type = MoveType::Invalid;
        };

        // Create a copy of the board and make the move on the copied board.
        let mut board_copy = board;
        board_copy.move_piece(&self.position, &new_position);

        // Only add the move if it wouldn't put the king in check.
        let mut mv = Move::new(mv_type.clone(), self.position, new_position);
        self.king_ok(board_copy, &mut mv);
    }
}
