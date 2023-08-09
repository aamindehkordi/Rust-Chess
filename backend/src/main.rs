pub mod board;
pub mod game;
pub mod moves;
pub mod piece;

use crate::game::*;

/**
 * The main entry point of the program.
 *
 * This function initializes a standard game and starts playing.
 */
fn main() {
    let mut game = Game::new();
    game.play();
}
