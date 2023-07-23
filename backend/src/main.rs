// Import necessary modules
mod game;
mod board;
mod moves;
mod player;
mod utils;

// Import necessary dependencies
use game::*;
use player::*;
use moves::*;
use utils::*;

fn main() {
    // Create a new game state
    let mut game_state = GameState::new();

    // Main game loop
    loop {
        // Display the current game state
        display_game_state(&game_state);
        // Get the current player from the game state
        let current_player = game_state.get_current_player();

        // If the current player is human, get a move from the user input
        // If the current player is an AI, get a move from the AI's "brain"
        let mv_pos = match current_player.kind {
            PlayerKind::Human => get_user_move(),
            PlayerKind::Computer(_) => get_ai_move(current_player),
        };

        // Validate the move
        match validate_move(&game_state, mv_pos) {
            Ok(mv) => {
                // If the move is valid, apply it to the game state
                game_state.apply_move(&mv);
                // Check if the game is over
                if is_game_over(&game_state) {
                    // If the game is over, display the game state and break out of the loop
                    display_game_state(&game_state);
                    break;
                }
            },
            Err(e) => {
                // If the move is not valid, print an error message and continue the loop
                println!("Invalid move: {}", e);
            }
        }
    }
}
