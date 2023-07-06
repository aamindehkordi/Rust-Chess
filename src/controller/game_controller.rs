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
            let (from, to) = self.view.get_move(); // Get the move from the user
            self.game.move_piece(from, to); // Move the piece
        }
    }
}
