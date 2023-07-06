use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Knight {
    color: Color,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Knight {
    pub fn new(color:Color) -> Knight {
        Self {
            color,
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }

    pub(crate) fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }
}
