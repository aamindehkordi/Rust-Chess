use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

pub struct Bishop {
    color: Color,
    can_take: Option<bool>,
    takeable: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,

}

impl Bishop {
    pub fn new(color: Color) -> Bishop {
        Self {
            color,
            can_take: None,
            takeable: None,
            pinned: None,
            has_moves: None,
        }
    }

}


pub trait Move {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile>;
}

impl Move for Bishop {
    fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }

}