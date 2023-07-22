// src/controller/game_controller.rs
use crate::model::game::Game;
use crate::model::moves::r#move::MoveType;
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
            self.view.display_board(self.game.get_board()); // Display the board
            let (from_row, from_col, to_row, to_col) = self.view.get_move().expect("Invalid Move"); // Get the move from the user
            let from = (from_row, from_col);
            let to = (to_row, to_col);
            println!("Move from user: from: {from:?}, to: {to:?}");
            match self.game.make_move(from, to) {
                Ok(Some(MoveType::Promo)) => {
                    if let Some(piece_type) = self.view.get_promotion_piece() {
                        self.game.promote(from, to, piece_type);
                    } else { continue; }
                }
                Ok(None) => (),
                Err(e) => {
                    if e.description() == "Game Over" {
                        println!("Game Over");
                        break;
                    }
                    println!("{e}");
                    continue;
                },
                _ => {}
            }
        }
    }

}
