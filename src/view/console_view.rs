use crate::model::game::Game;
use crate::model::pieces::piece::{Color, PieceType};

pub struct ConsoleView;

impl ConsoleView {
    pub fn new() -> Self {
        Self {}
    }

    /// Displays the board to the console Facing the current player.
    pub fn display_board(&self, game: &Game) {
        let board = game.get_board();
        let current_player = game.get_current_player();
        let mut board_string = String::new();
        board_string.push_str(&format!("Current player: {:?}\n", current_player));
        board_string.push_str("  A B C D E F G H\n");
        for (i, row) in board.get_tiles().iter().enumerate() {
            board_string.push_str(&format!("{} ", i + 1));
            for tile in row {
                board_string.push_str(&format!("{} ", tile.to_string()));
            }
            board_string.push_str(&format!("{}\n", i + 1));
        }
        board_string.push_str("  A B C D E F G H\n");
        println!("{}", board_string);
    }

    /// Prompts the user to enter a move in chess notation.
    pub fn get_move(&self) -> ((usize, usize), (usize, usize)) {
        println!("Enter a move in chess notation (e.g. A2 A4):");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input = input.split_whitespace();
        let from = input.next().unwrap();
        let to = input.next().unwrap();
        let from = self.notation_to_coords(from);
        let to = self.notation_to_coords(to);
        (from, to)
    }

    fn notation_to_coords(&self, notation: &str) -> (usize, usize) {
        let mut chars = notation.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();
        let file = (file as usize - 'A' as usize) as usize;
        let rank = (rank.to_digit(10).unwrap() - 1) as usize;
        (rank, file)
    }


    pub fn display_check(&self, color: &Color) {
        match color {
            Color::White => println!("White is in check!"),
            Color::Black => println!("Black is in check!"),
        }
    }


}
