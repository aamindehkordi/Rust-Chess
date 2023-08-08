use crate::game::player::{Color, Player};
use crate::rules::r#move::Move;

type MoveHistory = Vec<Move>;

#[derive(Clone)]
pub struct GameState {
    pub fen: String,               // FEN string
    pub players: (Player, Player), // Tuple of players
    pub turn: u8,                  // Turn counter
    pub move_history: MoveHistory, // MoveHistory struct
    pub white_in_check: bool,      // White in check flag
    pub black_in_check: bool,      // Black in check flag
}

impl Default for GameState {
    /**
     * Creates a new instance of the struct with default values.
     *
     * This function is a convenient way to create a new instance of the struct Chessboard with default values.
     *
     * @return A new Chessboard instance with default values.
     */
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /**
     * Creates a new Chessboard instance.
     *
     * This function initializes a new Chessboard struct with default values and returns it.
     *
     * @return A new Chessboard instance.
     */
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

    /**
     * Updates the turn to the next player's turn.
     *
     * This function checks the current turn and updates it to the next player's turn. If the current turn
     * is 0, it will be updated to 1. If the current turn is 1, it will be updated to 0.
     */
    pub fn next_turn(&mut self) {
        if self.turn == 0 {
            self.turn = 1;
        } else {
            self.turn = 0;
        }
    }

    /**
     * Checks if the specified player color is in check.
     *
     * This function checks if the specified player color is currently in check in the Chessboard struct.
     *
     * @param color - The player color to check.
     * @return true if the player color is in check, false otherwise.
     */
    pub fn is_in_check(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_in_check,
            Color::Black => self.black_in_check,
        }
    }
}
