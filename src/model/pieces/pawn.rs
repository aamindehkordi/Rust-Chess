use crate::model::board::Board;
use crate::model::pieces::piece::Color;
use crate::model::tile::Tile;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn {
    color: Color,
    first_move: Option<bool>,
    can_take: Option<bool>,
    takeable: Option<bool>,
    can_en_passant: Option<bool>,
    pinned: Option<bool>,
    has_moves: Option<bool>,
}

impl Pawn {
    pub fn new(color: Color) -> Pawn {
        Self {
            color,
            first_move: None,
            can_take: None,
            takeable: None,
            can_en_passant: None,
            pinned: None,
            has_moves: None,
        }
    }

    pub(crate) fn get_valid_move_list(&self, board: &Board) -> Vec<Tile> {
        todo!()
    }
}
