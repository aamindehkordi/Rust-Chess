use crate::board::{Color, Board};

pub enum PlayerKind {
    Human,
    Computer(Brain),
}

pub struct Brain {
    pub board: Board,
    pub color: Color,
}

impl Brain {
    pub fn new(board: Board, color: Color) -> Self {
        Self {
            board,
            color,
        }
    }
}

pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub color: Color,
}

impl Player {
    pub fn new(name:String, kind: PlayerKind, color: Color) -> Self {
        Self {
            name,
            kind,
            color,
        }
    }

    pub fn brain(&self) -> Option<&Brain> {
        match &self.kind {
            PlayerKind::Computer(brain) => Some(brain),
            _ => None,
        }
    }
}

