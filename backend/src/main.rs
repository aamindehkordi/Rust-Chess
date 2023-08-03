pub mod board;
pub mod game;
pub mod rules;


use crate::game::*;


fn main() {
    let mut game = Game::new();
    game.play();
}
