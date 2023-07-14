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

    fn get_position(&self) -> &(usize, usize) {
        &self.position
    }

    fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    fn get_type(&self) -> PieceType { PieceType::Bishop }

   fn get_directions(&self) -> &[(i32, i32)] { &self.directions }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }
    fn push_move(&mut self, mv: &mut Move){
        self.moves.push(mv.clone());
    }
}

