use crate::board::Board;
use crate::board::piece::PieceKind;
use crate::game::Game;
use crate::game::player::{Color, Player};
use crate::rules::r#move::Move;

type MoveHistory = Vec<Move>;


#[derive(Clone)]
pub struct GameState {
    pub fen: String, // FEN string
    pub players: (Player, Player), // Tuple of players
    pub turn: u8, // Turn counter
    pub move_history: MoveHistory, // MoveHistory struct
    pub white_in_check: bool, // White in check flag
    pub black_in_check: bool, // Black in check flag
}

impl GameState {
    pub fn new() -> Self {
        Self {
            fen: String::from(""),
            players: (Player::new(Color::White), Player::new(Color::Black)),
            turn: 0,
            move_history: Vec::new(),
            white_in_check: false,
            black_in_check: false,
        }
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_in_check,
            Color::Black => self.black_in_check,
        }
    }

}
