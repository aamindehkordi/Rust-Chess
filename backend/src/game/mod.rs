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
    fn default() -> Self {
        Self::new_standard()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            game_state: GameState::new(),
        }
    }

    pub fn new_standard() -> Self {
        Self {
            board: Board::new_standard(),
            game_state: GameState::new(),
        }
    }
}
pub fn update(game: Game, mv: Move) -> Game {
    let mut game = game;
    let mut gs = &mut game.game_state;
    gs.fen = game.board.to_fen();
    game.board.make_move(mv.clone());
    gs.move_history.push(mv);
    gs.white_in_check = game.board.board_info.is_in_check(Color::White);
    gs.black_in_check = game.board.board_info.is_in_check(Color::Black);
    gs.next_turn();
    game.game_state = gs.clone();
    game.board.update();
    game.game_state.move_history = game.board.board_info.move_history.clone();
    game
}

pub fn play(mut game: Game) {
    game.board.board_info.valid_moves = get_current_moves(&game);
    loop {
        display_board(&game.board);
        let mv_idx = user_mv_idx();
        let from: Position = (mv_idx.0, mv_idx.1);
        let to: Position = (mv_idx.2, mv_idx.3);
        game = apply_move(game, from, to);
        game.board.board_info.valid_moves = get_current_moves(&game);
    }
}

pub fn apply_move(game: Game, from: Position, to: Position) -> Game {
    let mut game = game;
    let from_square = game.board.get(from);
    let moves = game.board.board_info.valid_moves.clone();
    if from_square.is_some() {
        for mv in moves {
            if mv.to == to {
                game = update(game, mv);
            }
        }
    }
    game.game_state.next_turn();
    game.clone()
}

pub fn get_color_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for piece in board.squares.iter().flatten() {
        if piece.color == color {
            moves.append(&mut get_moves(&board.board_info, piece));
        }
    }
    moves
}

pub fn get_current_moves(game: &Game) -> Vec<Move> {
    let color = from_idx(game.game_state.turn);
    get_color_moves(&game.board, color)
}

pub fn get_all_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for piece in board.squares.iter().flatten() {
        moves.append(&mut get_moves(&board.board_info, piece));
    }
    moves
}
