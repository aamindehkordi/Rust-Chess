pub mod board;
pub mod game;
pub mod rules;

use crate::game::*;

/**
 * The main entry point of the program.
 *
 * This function initializes a standard game and starts playing.
 */
fn main() {
    let game = Game::new_standard();
    play(game);
}
