use crate::board::*;
use crate::piece::*;

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new_standard(),
            turn: Color::White,
        }
    }

    pub fn play(&mut self) {
        println!("{}", self.board);
    }
}
