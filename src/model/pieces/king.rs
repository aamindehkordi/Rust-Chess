// pieces/king.rs
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct King {
    color: Color,
    has_moved: bool,
    in_check: Option<bool>,
    has_moves: Option<bool>,
}

impl King {
    pub fn new(color: Color) -> King {
        Self {
            color,
            has_moved: false,
            in_check: None,
            has_moves: None,
        }
    }

    pub(crate) fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }
}
