pub mod game_state;
pub mod player;

use crate::board::piece::get_moves;
use crate::board::{display_board, Board, Position};
use crate::game::game_state::GameState;
use crate::game::player::{from_idx, user_mv_idx, Color};
use crate::rules::r#move::Move;

#[derive(Clone)]
pub struct Game {
    pub board: Board, // Board struct
    pub game_state: GameState,
}

impl Default for Game {
    /**
     * Creates a new instance of the current struct with default values.
     *
     * This function returns a new instance of the struct with default values. The default
     * implementation uses the `new_standard` function to create the instance.
     *
     * @return A new instance of the struct with default values.
     */
    fn default() -> Self {
        Self::new_standard()
    }
}

impl Game {
    /**
     * Creates a new instance of the Chessboard struct.
     *
     * This function initializes a new Chessboard struct by calling the constructor functions for the
     * associated structs `Board` and `GameState`.
     *
     * @return A newly created Chessboard instance.
     */
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            game_state: GameState::new(),
        }
    }

    /**
     * Creates a new Chessboard instance with a standard starting configuration.
     *
     * This function creates a new Chessboard instance with a board in the standard starting configuration
     * and initializes the game state.
     *
     * @return A new Chessboard instance with a standard starting configuration.
     */
    pub fn new_standard() -> Self {
        Self {
            board: Board::new_standard(),
            game_state: GameState::new(),
        }
    }
}
/**
 * Updates the game state after a move has been made.
 *
 * This function takes the current game state and a move as input, and updates the game state accordingly.
 * It performs the following steps:
 * 1. Creates a mutable copy of the game state.
 * 2. Sets the FEN string of the game state to the current board state.
 * 3. Makes the specified move on the board.
 * 4. Appends the move to the move history.
 * 5. Updates the 'white_in_check' flag based on whether the white player is in check.
 * 6. Updates the 'black_in_check' flag based on whether the black player is in check.
 * 7. Advances the turn to the next player.
 * 8. Updates the game state with the new game state.
 * 9. Updates the board state.
 * 10. Copies the move history from the board info to the game state.
 * 11. Returns the updated game state.
 *
 * @param game - The current game state.
 * @param mv - The move to be made.
 * @returns The updated game state.
 */
pub fn update(game: Game) -> Game {
    let mut game = game;
    let mut gs = &mut game.game_state;
    gs.fen = game.board.to_fen();
    gs.white_in_check = game.board.board_info.is_in_check(Color::White);
    gs.black_in_check = game.board.board_info.is_in_check(Color::Black);
    gs.next_turn();
    game.game_state = gs.clone();
    game.board.update();
    game.game_state.move_history = game.board.board_info.move_history.clone();
    game
}

/**
 * Plays the game by repeatedly getting user moves and applying them to the game board.
 *
 * This function implements the main game loop. It displays the current state of the board,
 * gets a move input from the user, applies the move to the game board, and updates the valid moves.
 * It continues this loop until the game is over.
 *
 * @param game - The game object representing the current state of the game.
 */
pub fn play(mut game: Game) {
    game.board.board_info.valid_moves = game.board.get_current_moves();
    loop {
        display_board(&game.board);
        let mv_idx = user_mv_idx();
        let from: Position = (mv_idx.0, mv_idx.1);
        let to: Position = (mv_idx.2, mv_idx.3);
        let result = apply_move(game.clone(), from, to);
        if let Ok(g) = result {
            game = g;
        } else {
            println!("Invalid move!");
        }
        game.board.board_info.valid_moves = game.board.get_current_moves();
    }
}

pub fn undo_move(game: Game) -> Game {
    let mut game = game;
    let last_move = game.game_state.move_history.pop();
    if last_move.is_some() {
        game.game_state.undo(&mut game.board);
        game.game_state.next_turn();
        game = update(game);
    }
    game
}

pub fn apply_move(game: Game, from: Position, to: Position) -> Result<Game, ()> {
    let mut game = game;
    let from_square = game.board.get_piece(from);
    let moves = game.board.board_info.valid_moves.clone();
    if from_square.is_some() {
        for mv in moves {
            if mv.to == to {
                game.board.make_move(mv.clone());
                game.game_state.move_history.push(mv);
                game = update(game);
                return Ok(game);
            }
        }
    }
    Err(())
}

pub fn get_color_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.squares.iter().flatten();
    for piece in pieces {
        if piece.color == color {
            moves.append(&mut get_moves(&board.board_info, piece));
        }
    }
    moves
}

/**
 * Retrieves all possible moves on the chessboard.
 *
 * This function iterates through all the squares on the board and retrieves the moves for each piece,
 * using the `get_moves` function. It then collects and returns these moves as a vector.
 *
 * @param board - The chessboard.
 * @return A vector containing all the possible moves on the chessboard.
 */
pub fn get_all_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for piece in board.squares.iter().flatten() {
        moves.append(&mut get_moves(&board.board_info, piece));
    }
    moves
}
