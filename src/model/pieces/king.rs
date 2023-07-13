use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::moves::r#move::{Move, MoveType};

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

impl Debug for King {
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

    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to();
        let mut this = board.pick_up_piece(&self.position).unwrap();
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
        self.position = *to_position;
        this.set_position(*to_position);
        board.put_down_piece(&self.position, Some(this));
        board.change_current_player();
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

    fn get_type(&self) -> PieceType { PieceType::King }

    fn get_directions(&self) -> &[(i32, i32)] { &self.directions }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn push_move(&mut self, mv: &mut Move){
        self.moves.push(mv.clone());
    }
}

impl King {
    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        let from_tile = board.get_tile(self.position).clone();
        let mut to_tile = board.get_tile(new_position).clone();
        to_tile.attacked(from_tile.position);
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

    // A function to add castling moves could also be added here.
    // This would involve checking if the king and rook have not moved,
    // and if there are no pieces between them, and if the squares the king would
    // move over are not under attack.
}
