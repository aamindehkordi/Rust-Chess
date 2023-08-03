pub mod game_state;
pub mod player;

use crate::board::{Board, display_board, Position};
use crate::board::piece::get_moves;
use crate::game::game_state::GameState;
use crate::game::player::{Color, Player, user_mv_idx};


#[derive(Clone)]
pub struct Game {
    pub board: Board, // Board struct
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_standard(),
            game_state: GameState::new(),
        }
    }

    pub fn play(&mut self) {
        let mut player = Player::new(Color::White);
        loop {
            display_board(&self.board);
            let mv_idx = user_mv_idx();
            let from: Position = (mv_idx.0, mv_idx.1).into();
            let to: Position = (mv_idx.2, mv_idx.3).into();
            let from_square = self.board.get(from);
            let to_square = self.board.get(to);
            if let Some(piece) = from_square {
                if piece.color == player.color {
                    let moves = get_moves(&self, &piece);
                    for mv in moves {
                        if mv.to == to {
                            self.board.make_move(mv);
                        }
                    }
                }
            }

        }
    }
}

pub fn is_attacked(game: Game, pos: Position, color: Color) -> bool {
    let mut attacked = false;
    for square in game.board.squares.iter() {
        if let Some(piece) = square {
            if piece.color == color {
                let moves = get_moves(&game, piece);
                for mv in moves {
                    if mv.to == pos {
                        attacked = true;
                    }
                }
            }
        }
    }
    attacked
}



