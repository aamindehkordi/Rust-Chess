
use crate::model::board::Board;
use crate::model::pieces::piece::Color;

pub struct Game {
    pub(crate) board: Board,
    current_player: &'static Color,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        let current_player = &Color::White;
        Self { board, current_player }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_current_player(&self) -> &Color {
        &self.current_player
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board.move_piece(from, to);



    }
    // Add methods for game rules, checking game state, etc.
}
