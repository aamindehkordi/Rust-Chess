use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::r#move::{Move, MoveType};

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

    fn create_move(&self, board: &Board, new_position: (usize, usize)) -> Move {
        let from_tile = board.get_tile(self.position);
        let to_tile = board.get_tile(new_position);
        let mv_type = if to_tile.is_empty() {
            if self.first_move && ((self.position.0 as i32 - new_position.0 as i32).abs() == 2) {
                MoveType::DoublePush
            } else {
                MoveType::Normal
            }
        } else {
            MoveType::Capture
        };
        Move::new(mv_type, from_tile.clone(), to_tile.clone())
    }

    fn update_moves(&mut self, board: Board) {
        self.moves.clear();

        if self.pinned && !self.can_take {
            self.update_pinned();
            return;
        }

        for direction in self.directions {
            if let Some(new_position) = self.get_new_position(self.position, direction) {
                self.check_and_add_move(board.clone(), new_position);
            }
        }

        if self.moves.is_empty() {
            self.has_moves = false;
            self.can_take = false;
        }
    }

    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to().get_position();
        let mut this = board.pick_up_piece(&self.position).unwrap();

        if this.get_color() == self.color && this.get_type() == self.piece_type && this.get_position() == self.position {
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
            self.update_moves(board.clone());
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
        PieceType::Pawn
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
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
        let mv_type = if to_tile.is_empty() {
            if self.first_move && ((self.position.0 as i32 - new_position.0 as i32).abs() == 2) {
                MoveType::DoublePush
            } else {
                MoveType::Normal
            }
        } else if self.position.1 as i32 - new_position.1 as i32 != 0 {
            MoveType::Capture
        } else {
            MoveType::Invalid
        };

        // Create a copy of the board and make the move on the copied board.
        let mut board_copy = board;
        board_copy.move_piece(&self.position, &new_position);

        // Only add the move if it wouldn't put the king in check.
        if !board_copy.is_king_in_check(&self.color) {
            let mut mv = Move::new(mv_type.clone(), from_tile, to_tile);
            mv.set_valid(true);
            self.moves.push(mv);
            self.has_moves = true;
            if mv_type == MoveType::Capture {
                self.can_take = true;
            }
        }
    }
}
