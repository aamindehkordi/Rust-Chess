// Import necessary modules
mod game;
mod board;
mod moves;
mod player;

// Import necessary dependencies
use game::*;
use player::*;
use moves::*;

fn main() {
    // Create a new game state
    let mut game_state = GameState::new();

    // Main game loop
    loop {
        game_state.calculate_all_moves();

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
                game_state.calculate_all_moves();
                game_state.change_current_player();
                // Check if the game is over
                if is_game_over(&game_state) {
                    // If the game is over, display the game state and break out of the loop
                    display_game_state(&game_state);
                    break;
                }
                // Check for check
                if game_state.is_current_player_in_check() {
                    game_state.game_status = GameStatus::Check;
                    println!("Check!");
                }
            },
            Err(e) => {
                // If the move is not valid, print an error message and continue the loop
                println!("Invalid move: {}", e);
            }
        }
    }
}
