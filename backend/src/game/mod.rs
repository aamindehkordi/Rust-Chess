pub mod game_state;
pub mod player;

use crate::board::piece::get_moves;
use crate::board::{display_board, Board, Position};
use crate::game::game_state::GameState;
use crate::game::player::{user_mv_idx, Color, Player};

#[derive(Clone)]
pub struct Game {
    pub board: Board, // Board struct
    pub game_state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_standard(),
            game_state: GameState::new(),
        }
    }

    pub fn play(&mut self) {
        let player = Player::new(Color::White);
        loop {
            display_board(&self.board);
            let mv_idx = user_mv_idx();
            let from: Position = (mv_idx.0, mv_idx.1);
            let to: Position = (mv_idx.2, mv_idx.3);
            let from_square = self.board.get(from);
            if let Some(piece) = from_square {
                if piece.color == player.color {
                    let moves = get_moves(self, &piece);
                    for mv in moves {
                        if mv.to == to {
                            self.board.make_move(mv);
                            self.game_state.next_turn();
                        }
                    }
                }
            }
        }
    }
}

pub fn is_attacked(game: Game, pos: Position, color: Color) -> bool {
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
