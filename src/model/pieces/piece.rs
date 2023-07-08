use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::pawn::Pawn;

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
    fn calc_valid_moves(&mut self, board: &Board);
    fn clone_box(&self) -> Box<dyn Piece>;
    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        self.calc_valid_moves(board);
        let moves = self.get_moves();
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
    fn get_color(&self) -> Color;
    fn get_position(&self) -> (usize, usize);
    fn get_moves(&self) -> &Vec<(usize, usize)>;
    fn get_type(&self) -> PieceType;

}

#[derive(Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
