
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

    pub fn set_current_player(&mut self, color: Color) {
        self.current_player = &color;
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
    if let Some(piece) = self.board.get_piece(from) {
        let valid_moves = piece.get_piece_type().get_valid_moves(&self.board);
        valid_moves.contains(&to)
    } else {
        false
    }
}


    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board.move_piece(from, to);


    }
    // Add methods for game rules, checking game state, etc.
}
