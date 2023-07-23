use std::time::Duration;
// Import necessary modules and dependencies
use crate::board::{Board, Color};
use crate::player::Player;
use crate::moves::Move;
use crate::player::PlayerKind::Human;

pub enum GameStatus {
    InProgress,
    Check,
    Promotion,
    GameOver,
}

pub struct Timer {
    player1_time: Option<Duration>,
    player2_time: Option<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            player1_time: Some(Duration::new(0, 0)),
            player2_time: Some(Duration::new(0, 0)),
        }
    }
}

pub struct GameState {
    pub board: Board,
    players: [Player; 2],
    pub current_player: usize,  // index into players array
    pub(crate) move_history: Vec<Move>,
    game_status: GameStatus,
    timers: Timer,
}

impl GameState {
    // Function to create a new game state
    pub fn new() -> Self {
        let board = Board::new_standard();
        let players = [
            Player::new("Player 1".to_string(), Human, Color::White),
            Player::new("Player 2".to_string(), Human, Color::Black),
        ];
        let current_player = 0;
        let move_history = Vec::new();

        Self {
            board,
            players,
            current_player,
            move_history,
            game_status: GameStatus::InProgress,
            timers: Timer::new(),
        }
    }

    // Function to get the current player
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    // Function to change the current player
    pub fn change_current_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }
}

// Function to display the game state
pub fn display_game_state(game_state: &GameState) {
    println!("{}", game_state.board);
}

pub fn is_game_over(game_state: &GameState) -> bool {
    // ... check if the game is over ...
    false
}