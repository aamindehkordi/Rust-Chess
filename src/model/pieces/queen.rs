use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct Queen {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<Move>,
    piece_type: PieceType,
    can_take: bool,
    takeable: bool,
    pinned: bool,
    has_moves: bool,
}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "Q"),
            Color::Black => write!(f, "q"),
        }
    }
}

impl Piece for Queen {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            moves: Vec::new(),
            piece_type: PieceType::Queen,
            can_take: false,
            takeable: false,
            pinned: false,
            has_moves: true,
        }
    }

    fn update_moves(&mut self, board: Board) {
        self.moves.clear();

        if self.pinned && !self.can_take {
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
            self.can_take = false;
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
        PieceType::Queen
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }
}

impl Queen {
    fn update_pinned(&mut self) {
        self.moves.clear();
        self.has_moves = false;
        self.takeable = true;
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
