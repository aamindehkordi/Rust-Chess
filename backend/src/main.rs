// Import necessary modules
mod game;
mod board;
mod moves;
mod player;
mod utils;
mod validation;

use game::*;
use player::*;
use moves::*;
use utils::*;
use validation::*;

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
        let mv = match current_player.kind {
            PlayerType::Human => get_user_move(),
            PlayerType::AI => get_ai_move(current_player.brain),
        };

        // Validate the move
        match validate_move(&game_state, & mv) {
            Ok(_) => {
                // If the move is valid, apply it to the game state
                apply_move(&mut game_state, & mv);

                // Check if the game is over
                if is_game_over(&game_state) {
                    // If the game is over, break the loop
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
