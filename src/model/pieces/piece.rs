use std::fmt::Display;
use crate::model::board::Board;

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

const BOARD_SIZE: i32 = 8;
pub trait Piece: Display {
    fn new(color: Color, position: (usize, usize)) -> Self where Self: Sized;
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)>;
    fn clone_box(&self) -> Box<dyn Piece>;
    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        let moves = self.get_valid_moves(board);
        if moves.contains(&to) {
            board.move_piece(from, to);
            Ok(())
        } else {
            Err(format!("Invalid move: {:?} to {:?}", from, to))
        }
    }

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
    fn get_color(&self) -> &Color;
    fn get_position(&self) -> (usize, usize);
    fn get_moves(&self) -> &Vec<(usize, usize)>;

}

pub enum PieceType {
    Pawn(Box<dyn Piece>),
    Rook(Box<dyn Piece>),
    Knight(Box<dyn Piece>),
    Bishop(Box<dyn Piece>),
    Queen(Box<dyn Piece>),
    King(Box<dyn Piece>),
}

impl PieceType {
    pub fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        match self {
            PieceType::Pawn(ref mut piece) => piece.get_valid_moves(board),
            PieceType::Rook(ref mut piece) => piece.get_valid_moves(board),
            PieceType::Knight(ref mut piece) => piece.get_valid_moves(board),
            PieceType::Bishop(ref mut piece) => piece.get_valid_moves(board),
            PieceType::Queen(ref mut piece) => piece.get_valid_moves(board),
            PieceType::King(ref mut piece) => piece.get_valid_moves(board),
        }
    }
}