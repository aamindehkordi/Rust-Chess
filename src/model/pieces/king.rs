use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct King {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<Move>,
    piece_type: PieceType,
    has_moved: bool,
    in_check: bool,
    has_moves: bool,
    can_take: bool,
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "K"),
            Color::Black => write!(f, "k"),
        }
    }
}

impl Piece for King {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            moves: Vec::new(),
            piece_type: PieceType::King,
            has_moved: false,
            in_check: false,
            has_moves: true,
            can_take: false,
        }
    }

    fn update_moves(&mut self, board: Board) {
        self.moves.clear();

        for direction in self.directions {
            if let Some(new_position) = self.get_new_position(self.position, direction) {
                self.check_and_add_move(board.clone(), new_position);
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
                MoveType::Normal | MoveType::Castle(_) => {
                    board.move_piece(&self.position, to_position);
                },
                MoveType::Capture => {
                    board.move_piece(&self.position, to_position);
                    board.take_piece(mv.get_to());
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
        PieceType::King
    }
}

impl King {
    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        let from_tile = board.get_tile(self.position).clone();
        let mut to_tile = board.get_tile(new_position).clone();
        to_tile.attacked(from_tile.position);
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
            let mut mv = Move::new(mv_type.clone(), from_tile, to_tile);
            mv.set_valid(true);
            self.moves.push(mv);
            self.has_moves = true;
            if mv_type == MoveType::Capture {
                self.can_take = true;
            }
        }

    }

    // A function to add castling moves could also be added here.
    // This would involve checking if the king and rook have not moved,
    // and if there are no pieces between them, and if the squares the king would
    // move over are not under attack.
}
