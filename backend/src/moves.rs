use crate::board::*;

pub type Moves = Vec<Move>;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}
