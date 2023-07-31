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
        calculate_all_moves(&mut game_state);

        // Display the current game state
        display_game_state(&game_state);
        // Get the current player from the game state
        let current_player = get_current_player(&game_state);

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
                game_state = apply_move(&game_state, &mv);
                // Check if the game is over
                if is_game_over(&game_state) {
                    // If the game is over, display the game state and break out of the loop
                    display_game_state(&game_state);
                    break;
                }
                // Check for check
                if is_current_player_in_check(&game_state) {
                    game_state.game_status = GameStatus::Check(get_current_player(&game_state).color);
                    println!("Check!");
                }
                if is_in_checkmate(&game_state, get_current_player(&game_state).color) {
                    game_state.game_status = GameStatus::Checkmate(get_current_player(&game_state).color);
                    println!("Checkmate!");
                    break;
                }
            },
            Err(err) => {
                match err {
                    MoveError::MoveIsNotValid => println!("Move is not valid"),
                    MoveError::MoveDoesNotBlockCheck => println!("Move does not block check"),
                    MoveError::MoveIsNotPromotion => println!("Move is not a promotion"),
                    MoveError::MoveIsNotCapturePromotion => println!("Move is not a capture promotion"),
                    MoveError::Other(msg) => println!("Invalid move: {}", msg),
                }
            }
        }
    }
}
