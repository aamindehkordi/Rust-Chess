use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct Rook {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 4],
    moves: Vec<Move>,
    piece_type: PieceType,
    first_move: bool,
    pinned: bool,
    has_moves: bool,
}

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "R"),
            Color::Black => write!(f, "r"),
        }
    }
}

impl Piece for Rook {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(0, 1), (0, -1), (1, 0), (-1, 0)],
            moves: Vec::new(),
            piece_type: PieceType::Rook,
            first_move: true,
            pinned: false,
            has_moves: true,
        }
    }

    fn update_moves(&mut self, board: Board) {
        self.moves.clear();

        if self.pinned {
            self.update_pinned();
            return;
        }

        for direction in self.directions {
            let mut new_position = self.get_new_position(self.position, direction);
            while let Some(pos) = new_position {
                self.check_and_add_move(board.clone(), pos);
                new_position = self.get_new_position(pos, direction);
            }
        }

        if self.moves.is_empty() {
            self.has_moves = false;
        }
    }

    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to().get_position();
        let this = board.pick_up_piece(&self.position).unwrap();

        if this.get_color() == self.color && this.get_type() == self.piece_type && this.get_position() == self.position {
            match mv.get_move_type() {
                MoveType::Normal | MoveType::Capture => {
                    board.move_piece(&self.position, to_position);
                    if mv.get_move_type().clone() == MoveType::Capture {
                        board.take_piece(mv.get_to());
                    }
                },
                _ => {},
            }
            self.position = to_position.clone();
            board.put_down_piece(&self.position, Some(this));
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
        PieceType::Rook
    }
}

impl Rook {
    fn update_pinned(&mut self) {
        self.moves.clear();
        self.has_moves = false;
    }

    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        let from_tile = board.get_tile(self.position).clone();
        let to_tile = board.get_tile(new_position).clone();
        let mv_type = if to_tile.is_empty() {
            MoveType::Normal
        } else {
            MoveType::Capture
        };

        // Create a copy of the board and make the move on the copied board.
        let mut board_copy = board.clone();
        board_copy.move_piece(&self.position, &new_position);

        // Only add the move if it wouldn't put the king in check.
        if !board_copy.is_king_in_check(&self.color) {
            let mut mv = Move::new(mv_type, from_tile, to_tile);
            mv.set_valid(true);
            self.moves.push(mv);
            self.has_moves = true;
        }
    }
}
