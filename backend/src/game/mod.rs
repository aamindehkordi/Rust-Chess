pub mod game_state;
pub mod player;

use crate::board::piece::get_moves;
use crate::board::{display_board, update_board, Board, Position};
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
    game.board.make_move(mv);
    gs.white_in_check = game.board.in_check(Color::White);
    gs.black_in_check = game.board.in_check(Color::Black);
    gs.next_turn();
    game.board = update_board(&game);
    game
}

pub fn play(mut game: Game) {
    loop {
        display_board(&game.board);
        let mv_idx = user_mv_idx();
        let from: Position = (mv_idx.0, mv_idx.1);
        let to: Position = (mv_idx.2, mv_idx.3);
        game = apply_move(game, from, to);
    }
}

pub fn apply_move(game: Game, from: Position, to: Position) -> Game {
    let mut game = game;
    let from_square = game.board.get(from);
    if let Some(piece) = from_square {
        let moves = get_moves(&game, &piece);
        for mv in moves {
            if mv.to == to {
                game = update(game, mv);
            }
        }
    }
    game.game_state.next_turn();
    game.clone()
}

pub fn is_attacked_not_bb(game: Game, pos: Position, color: Color) -> bool {
    let mut attacked = false;
    for square in game.board.squares.iter().flatten() {
        let piece = square;
        if piece.color == color {
            let moves = get_moves(&game, piece);
            for mv in moves {
                if mv.to == pos {
                    attacked = true;
                }
            }
        }
    }
    attacked
}

pub fn get_current_moves(game: &Game) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let color = from_idx(game.game_state.turn);

    for piece in game.board.squares.iter().flatten() {
        if piece.color == color {
            moves.append(&mut get_moves(game, piece));
        }
    }
    moves
}

pub fn get_all_moves(game: &Game) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for piece in game.board.squares.iter().flatten() {
        moves.append(&mut get_moves(game, piece));
    }
    moves
}
