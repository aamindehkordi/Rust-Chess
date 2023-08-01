use std::time::Duration;
use crate::board::{Color, Board};

#[derive(Clone)]
pub struct Timer {
    pub time: Duration,
    pub increment: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: Duration::new(0, 0),
            increment: Duration::new(0, 0),
        }
    }

    pub fn increment(&mut self) {
        self.time += self.increment;
    }

    pub fn reset(&mut self) {
        self.time = Duration::new(0, 0);
    }
}

#[derive(Clone)]
pub enum PlayerKind {
    Human,
    Computer(Brain),
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub timer: Timer,
    pub color: Color,
}

impl Player {
    pub fn new(name:String, kind: PlayerKind, color: Color) -> Self {
        Self {
            name,
            kind,
            timer: Timer::new(),
            color,
        }
    }

}

