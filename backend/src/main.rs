pub mod board;
pub mod game;
pub mod rules;

use crate::board::*;
use crate::game::*;
use crate::rules::*;

fn main() {
    let mut game = Game::new();
    game.play();
}
