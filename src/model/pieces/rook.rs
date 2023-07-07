use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, PieceType};
use crate::model::r#move::Move;

#[derive(Clone, PartialEq, Debug)]
pub struct Rook {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 4],
    moves: Vec<(usize, usize)>,
    first_move: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "R"),
            Color::Black => write!(f, "r"),
        }
    }
}

impl Rook {
    pub fn new(color:Color, position: (usize, usize)) -> Rook {
        Self {
            color,
            position,
            directions: [(0, 1), (0, -1), (1, 0), (-1, 0)],
            moves: Vec::new(),
            first_move: None,
            pinned: None,
            has_moves: None,
        }
    }

    pub fn get_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            let mut new_position = get_new_position(self.position, direction);
            while let Some(pos) = new_position {
                let tile = board.get_tile(pos);
                if tile.is_empty() {
                    self.moves.push(pos);
                    new_position = get_new_position(pos, direction);
                } else {
                    if tile.get_piece().as_ref().map_or(false, |p| p.get_color() != &self.color) {
                        self.moves.push(pos);
                    }
                    break;
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

impl Move for Rook {
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)> {
        self.get_moves(board)
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        if let Some(mut piece) = board.pick_up_piece(from) {
            if let PieceType::Rook(rook) = &mut piece.piece_type {
                rook.position = to;
            }
            else {
                board.put_down_piece(from, piece);
                return Err(format!("Piece at ({}, {}) is not a rook, it is a {:?}", from.0, from.1, board.get_piece(from).unwrap().piece_type));
            }
            board.put_down_piece(to, piece);
            Ok(())
        } else {
            Err(format!("No piece at the starting position ({}, {})", from.0, from.1))
        }
    }
}
