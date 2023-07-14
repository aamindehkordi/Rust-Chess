// src/controller/game_controller.rs
use crate::model::game::Game;
use crate::view::console_view::ConsoleView;

pub struct GameController {
    game: Game,
    view: ConsoleView,
}

impl GameController {
    pub fn new() -> Self {
        let game = Game::new();
        let view = ConsoleView::new();

        Self { game, view }
    }

    pub fn start_game(&mut self) {
        loop {
            self.view.display_board(&self.game); // Display the board
            let (from_row, from_col, to_row, to_col) = self.view.get_move().unwrap(); // Get the move from the user
            let from = (from_row, from_col);
            let to = (to_row, to_col);
            println!("Move from user: from: {:?}, to: {:?}", from, to);
            if let Err(e) = self.game.make_move(from, to) { // Make the move
                println!("{}", e);
                continue;
            }
        }
    }
}
