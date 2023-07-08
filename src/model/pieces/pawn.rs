use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn {
    color: Color,
    position: (usize, usize),
    directions: [(i32, i32); 2],
    moves: Vec<(usize, usize)>,
    first_move: bool,
    can_take: Option<bool>,
    takeable: Option<bool>,
    can_en_passant: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "P"),
            Color::Black => write!(f, "p"),
        }
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (usize, usize)) -> Self {
        let directions = match color {
            Color::White => [(1, 0), (1, 1)],
            Color::Black => [(-1, 0), (-1, -1)],
        };
        Self {
            color,
            position,
            directions,
            moves: Vec::new(),
            first_move: true,
            can_take: None,
            takeable: None,
            can_en_passant: None,
            pinned: None,
            has_moves: None,
        }
    }
    fn calc_valid_moves(&mut self, board: &Board) {
        self.moves.clear();
        // Check if the tile in front of the pawn is empty
        if let Some(new_position) = self.get_new_position(self.position, self.directions[0]) {
            let tile = board.get_tile(new_position);
            if tile.is_empty() {
                self.moves.push(new_position);
                self.has_moves = Some(true);
            }
        }
        // Check if it is the first move
        if self.first_move {
            // Check if the two tiles in front of the pawn is empty
            if let Some(new_position) = self.get_new_position(self.position, self.directions[0]) {
                let tile = board.get_tile(new_position);
                if tile.is_empty() {
                    if let Some(new_position) = self.get_new_position(new_position, self.directions[0]) {
                        let tile = board.get_tile(new_position);
                        if tile.is_empty() {
                            self.moves.push(new_position);
                            self.has_moves = Some(true);
                        }
                    }
                }
            }
        }
        // Check if the pawn can take a piece
        for &direction in &self.directions[1..] {
            if let Some(new_position) = self.get_new_position(self.position, direction) {
                let tile = board.get_tile(new_position);
                if !tile.is_empty() {
                    let piece = tile.get_piece();
                    if piece.as_ref().map_or(false, |p| p.get_color() != self.color) {
                        self.moves.push(new_position);
                        self.has_moves = Some(true);
                        self.can_take = Some(true);
                    }
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        if self.moves.contains(&to) {
            board.move_piece(from, to);
            self.position = to;
            self.first_move = false;
            Ok(())
        } else {
            Err(format!("Invalid move from {:?} to {:?}", from, to))
        }
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_moves(&self) -> &Vec<(usize, usize)> {
        &self.moves
    }

    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
}
