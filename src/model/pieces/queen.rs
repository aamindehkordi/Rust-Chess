// pieces/queen.rs
use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Queen {
    color: Color,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Queen {
    pub fn new(color: Color) -> Queen {
        Self {
            color,
            pinned: None,
            has_moves: None,
        }
    }

    pub(crate) fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }
}
