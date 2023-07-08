use std::fmt::Display;
use crate::model::board::Board;
use crate::model::pieces::piece::{Color, Piece, PieceType};

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

impl Piece for Rook {
    fn new(color:Color, position: (usize, usize)) -> Rook {
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
    fn calc_valid_moves(&mut self, board: &Board){
        self.moves.clear();
        // Check all possible moves
        for &direction in &self.directions {
            let mut new_position = self.get_new_position(self.position, direction);
            while let Some(pos) = new_position {
                let tile = board.get_tile(pos);
                if tile.is_empty() {
                    self.moves.push(pos);
                    new_position = self.get_new_position(pos, direction);
                } else {
                    if tile.get_piece().as_ref().map_or(false, |p| p.get_color() != self.color) {
                        self.moves.push(pos);
                    }
                    break;
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
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
        PieceType::Rook
    }
}