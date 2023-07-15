use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::moves::r#move::{CastleType, Move, MoveType};

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
            MoveType::Normal => {
                board.move_piece(&self.position, to_position);
            },
            MoveType::Capture => {
                board.move_piece(&self.position, to_position);
                board.take_piece(mv.get_to());
            },
            MoveType::Castle(CastleType::Kingside) => {
                let rook_position = match self.color {
                    Color::Black => (7, 7),
                    Color::White => (0, 7),
                };
                let mut rook = board.pick_up_piece(&rook_position).unwrap();
                board.move_piece(&self.position, to_position);
                board.move_piece(&rook_position, &(to_position.0, to_position.1 - 1));
                rook.set_position((to_position.0, to_position.1 - 1));
                board.put_down_piece(&(to_position.0, to_position.1 - 1), Some(rook));
            },
            MoveType::Castle(CastleType::Queenside) => {
                let rook_position = match self.color {
                    Color::Black => (7, 0),
                    Color::White => (0, 0),
                };
                let mut rook = board.pick_up_piece(&rook_position).unwrap();
                board.move_piece(&self.position, to_position);
                board.move_piece(&rook_position, &(to_position.0, to_position.1 + 1));
                rook.set_position((to_position.0, to_position.1 + 1));
                board.put_down_piece(&(to_position.0, to_position.1 + 1), Some(rook));
            }
            _ => {},
        }
        self.position = *to_position;
        this.set_position(*to_position);
        board.put_down_piece(&self.position, Some(this));
        self.has_moved = true;
    }
    //...
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

    fn push_move(&mut self, mv: &Move){
        self.moves.push(mv.clone());
    }
}
