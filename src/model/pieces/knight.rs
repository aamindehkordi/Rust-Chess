use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, PieceType};
use crate::model::r#move::Move;

#[derive(Clone, PartialEq, Debug)]
pub struct Knight {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 8],
    moves: Vec<(usize, usize)>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "N"),
            Color::Black => write!(f, "n"),
        }
    }
}
impl Knight {
    pub fn new(color:Color, position: (usize, usize)) -> Knight {
        Self {
            color,
            position,
            directions: [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
            moves: Vec::new(),
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }

    pub fn get_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            if let Some(new_position) = get_new_position(self.position, direction) {
                let tile = board.get_tile(new_position);
                if tile.is_empty() || tile.get_piece().as_ref().map_or(false, |p| p.get_color() != &self.color) {
                    self.moves.push(new_position);
                }
            }
        }
        self.moves.clone()
    }

}
const BOARD_SIZE: i32 = 8;

fn is_in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < BOARD_SIZE && y >= 0 && y < BOARD_SIZE
}

fn get_new_position(position: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> {
    let (x, y) = position;
    let (dx, dy) = direction;

    let new_x = x as i32 + dx;
    let new_y = y as i32 + dy;

    if is_in_bounds(new_x, new_y) {
        Some((new_x as usize, new_y as usize))
    } else {
        None
    }
}

impl Move for Knight {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.get_moves(board)
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        if let Some(mut piece) = board.pick_up_piece(from) {
            if let PieceType::Knight(knight) = &mut piece.piece_type {
                knight.position = to;
            }
            else {
                board.put_down_piece(from, piece);
                return Err(format!("Piece at ({}, {}) is not a knight, it is a {:?}", from.0, from.1, board.get_piece(from).unwrap().piece_type));
            }
            board.put_down_piece(to, piece);
            Ok(())
        } else {
            Err(format!("No piece at the starting position ({}, {})", from.0, from.1))
        }
    }

}