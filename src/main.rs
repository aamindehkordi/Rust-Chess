mod controller;
mod model;
mod view;

use controller::game_controller::GameController;

/// Chess game
/// Here is the main function which contains the main game loop
fn main() {

    let mut controller = GameController::new();
    controller.start_game();

    }
