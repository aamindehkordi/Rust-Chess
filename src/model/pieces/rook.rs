use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

pub struct Rook {
    color: Color,
    first_move: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Rook {
    pub fn new(color: Color) -> Rook {
        Self {
            color,
            first_move: None,
            pinned: None,
            has_moves: None,
        }
    }
}

pub trait Move {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile>;
}

impl Move for Rook {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }
}
