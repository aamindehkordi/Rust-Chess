use std::fmt::{Debug, Display};
use crate::model::board::Board;
use crate::model::r#move::{Move, MoveType};

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::White, Color::White) => true,
            (Color::Black, Color::Black) => true,
            _ => false,
        }
    }
}

const BOARD_SIZE: i32 = 8;
pub trait Piece: Display {
    fn new(color: Color, position: (usize, usize)) -> Self where Self: Sized;
    fn create_move(&self, board: &Board, new_position: (usize, usize)) -> Move {
        let from_tile = board.get_tile(self.get_position());
        let to_tile = board.get_tile(new_position);
        let mv_type = if to_tile.is_empty() {
            MoveType::Normal
        } else {
            MoveType::Capture
        };
        Move::new(mv_type, from_tile.clone(), to_tile.clone())
    }
    fn update_moves(&mut self, board: Board);
    fn execute(&mut self, board: &mut Board, mv: Move) {
        let to_position = mv.get_to().get_position();
        let mut this = board.pick_up_piece(&self.get_position()).unwrap();

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
            self.set_position(to_position.clone());
            this.set_position(to_position.clone());
            board.put_down_piece(&self.get_position(), Some(this));
            self.update_moves(board.clone());
        }
    }
    fn clone_box(&self) -> Box<dyn Piece>;

    fn is_in_bounds(&self, x: i32, y: i32) -> bool where Self: Sized {
        x >= 0 && x < BOARD_SIZE && y >= 0 && y < BOARD_SIZE
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

    fn get_color(&self) -> Color;
    fn get_position(&self) -> (usize, usize);
    fn get_moves(&self) -> &Vec<Move>;
    fn get_type(&self) -> PieceType;
    fn set_position(&mut self, position: (usize, usize));
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }


}
