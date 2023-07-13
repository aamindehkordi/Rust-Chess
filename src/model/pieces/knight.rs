use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};
use crate::model::moves::r#move::{Move, MoveType};

#[derive(Clone, PartialEq)]
pub struct Knight {
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
impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "N"),
            Color::Black => write!(f, "n"),
        }
    }
}

impl Debug for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "N"),
            Color::Black => write!(f, "n"),
        }
    }
}

impl Piece for Knight {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Self {
            color,
            position,
            directions: [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
            moves: Vec::new(),
            piece_type: PieceType::Knight,
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

    fn get_position(&self) -> &(usize, usize) { &self.position }

    fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    fn get_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_directions(&self) -> &[(i32, i32)] { &self.directions }


    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn push_move(&mut self, mv: &mut Move){
        self.moves.push(mv.clone());
    }
}

impl Knight {
    fn update_pinned(&mut self) {
        self.moves.clear();
        self.has_moves = false;
        self.takeable = true;
    }

    fn check_and_add_move(&mut self, board: Board, new_position: (usize, usize)) {
        println!("Checking move to position: {:?}", new_position);
        let from_tile = board.get_tile(self.position).clone();
        let to_tile = board.get_tile(new_position).clone();
        let mv_type = if to_tile.is_empty() {
            MoveType::Normal
        } else if to_tile.get_piece().as_ref().expect("no piece").get_color() != self.color {
            MoveType::Capture
        } else { MoveType::Invalid };


        let mut mv = Move::new(mv_type, self.position, new_position);

        if to_tile.is_empty() || to_tile.get_piece().as_ref().unwrap().get_color() != self.get_color() {
            let mut board_copy = board;
            board_copy.move_piece(&self.position, &new_position);

            if !board_copy.is_king_in_check(&self.color) {
                mv.set_valid(true);
                self.moves.push(mv);
                self.has_moves = true;
                self.can_take = true;
            }
        } else {
            mv.set_valid(false);
            self.has_moves = true;
            self.can_take = true;
        }
    }
}
