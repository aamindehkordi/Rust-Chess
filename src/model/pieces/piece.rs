use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::moves::r#move::{Move, MoveType};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Color::White => match other {
                Color::White => Ordering::Equal,
                Color::Black => Ordering::Less,
            },
            Color::Black => match other {
                Color::White => Ordering::Greater,
                Color::Black => Ordering::Equal,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

const BOARD_SIZE: i32 = 8;
pub trait Piece: Display + Debug  {
    fn new(color: Color, position: (usize, usize)) -> Self where Self: Sized;

    // piece specific execute function
    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to();
        let mut this = board.pick_up_piece(&self.get_position()).unwrap();

        // check if the piece is the same as the one on the board
        if this.get_color() == self.get_color() && this.get_type() == self.get_type() && this.get_position() == self.get_position() {
            match mv.get_move_type() {
                MoveType::Normal => {
                    board.move_piece(&self.get_position(), to_position);
                },
                MoveType::Capture => {
                    board.move_piece(&self.get_position(), to_position);
                    board.take_piece(mv.get_to());
                },
                _ => {},
            }
            self.set_position(*to_position);
            this.set_position(*to_position);
            board.put_down_piece(&self.get_position(), Some(this));
        }
    }
    fn clone_box(&self) -> Box<dyn Piece>;

    fn is_in_bounds(&self, x: i32, y: i32) -> bool where Self: Sized {
        (0..BOARD_SIZE).contains(&x) && (0..BOARD_SIZE).contains(&y)
    }

    fn get_new_position(&self, position: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> where Self: Sized {
        let (x, y) = position;
        let (dx, dy) = direction;

        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if self.is_in_bounds(new_x, new_y) {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    fn get_name(&self) -> String {
        format!("{:?} {:?}", self.get_color(), self.get_type())
    }
    fn get_color(&self) -> Color;
    fn get_position(&self) -> &(usize, usize);
    fn get_moves(&self) -> &Vec<Move>;
    fn get_type(&self) -> PieceType;
    fn get_directions(&self) -> &[(i32, i32)];
    fn set_position(&mut self, position: (usize, usize));
    fn push_move(&mut self, mv: &Move);
    fn get_promotion_types(&self) -> Vec<PieceType> {
        vec![PieceType::Pawn, PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen]
    }
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            Self::Pawn => 1,
            Self::Rook => 5,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Queen => 9,
            Self::King => 0,
        }
    }
}

impl Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::White, Self::White) => true,
            (Self::Black, Self::Black) => true,
            _ => false,
        }
    }
}
