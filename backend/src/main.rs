pub mod board;
pub mod game;
pub mod rules;

use crate::game::*;

fn main() {
    let game = Game::new_standard();
    play(game);
}
