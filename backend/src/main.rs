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
    let game_state = GameState::new_standard();

    // Main game loop
    main_loop(Some(game_state));

}

fn main_loop(gs: Option<GameState>) {
    let mut game_state: GameState;
    if let Some(gs) = gs {
        game_state = gs;
    } else {
        game_state = GameState::new_standard();
    }
    loop {
        calculate_black_moves(&mut game_state);
        // Display the current game state
        display_game_state(&game_state);
        // Get the current player from the game state
        let current_player = get_current_player(&game_state);

        // If the current player is human, get a move from the user input
        // If the current player is an AI, get a move from the AI's "brain"
        let mv_pos = match current_player.kind {
            PlayerKind::Human => user_mv_idx(),
            PlayerKind::Computer(_) => ai_mv_idx(current_player),
        };
        calculate_all_moves(&mut game_state);
        // Validate the move
        match validate_move(&game_state, mv_pos) {
            Ok(mv) => {
                game_state.game_status = GameStatus::InProgress;
                // If the move is valid, apply it to the game state
                game_state = apply_move(&game_state, &mv);
                // Check if the game is over
                if is_game_over(&game_state) {
                    // If the game is over, display the game state and break out of the loop
                    display_game_state(&game_state);
                    break;
                }
            },
            Err(err) => {
                match err {
                    MoveError::MoveIsNotValid => println!("Move is not valid"),
                    MoveError::MoveDoesNotBlockCheck => println!("Move does not block check"),
                    MoveError::Other(msg) => println!("Invalid move: {}", msg),
                }
            }
        }
    }
}
