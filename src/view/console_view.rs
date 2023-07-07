use std::io;
use std::io::Write;
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
        for i in (0..8).rev() {
            for j in 0..8 {
                let tile = board.get_tile((i, j));
                if let Some(piece) = tile.get_piece() {
                    print!("{}", piece);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    /// Prompts the user to enter a move in chess notation.
    /// Example: e2-e4
    pub fn get_move(&self) -> Result<(usize, usize, usize, usize), &'static str> {
        print!("Enter your move: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.notation_to_coords(&input.trim())
    }

    /// Converts chess notation to coordinates.
    /// Example: e2-e4 -> (6, 4, 4, 4)
    fn notation_to_coords(&self, notation: &str) -> Result<(usize, usize, usize, usize), &'static str> {
        if notation.len() != 5 || &notation[2..3] != "-" { // if the notation is not 5 characters long or the 3rd character is not a dash
            return Err("Invalid notation");
        }
        let from = &notation[0..2]; // get the first two characters
        let to = &notation[3..5]; // get the last two characters
        let from_coords = self.notation_to_coord(from)?; // convert the first two characters to coordinates
        let to_coords = self.notation_to_coord(to)?;    // convert the last two characters to coordinates
        Ok((from_coords.0, from_coords.1, to_coords.0, to_coords.1)) // return the coordinates
    }

    /// Converts individual chess notation to coordinates.
    /// Example: e2 -> (6, 4)
    fn notation_to_coord(&self, notation: &str) -> Result<(usize, usize), &'static str> {
        if notation.len() != 2 { // if the notation is not 2 characters long
            return Err("Invalid notation");
        }
        let file = notation.chars().nth(0).unwrap(); // get the first character
        let rank = notation.chars().nth(1).unwrap(); // get the second character
        let file = match file {
            'a'..='h' => file as usize - 'a' as usize, // convert the file to a number
            _ => return Err("Invalid file"), // if the file is not a-h, return an error
        };
        let rank = match rank.to_digit(10) {
            Some(n) if n >= 1 && n <= 8 => n as usize - 1, // convert the rank to a number
            _ => return Err("Invalid rank"), // if the rank is not 1-8, return an error
        };
        Ok((rank, file)) // return the coordinates
    }

    pub fn display_check(&self, color: &Color) {
        match color {
            Color::White => println!("White is in check!"),
            Color::Black => println!("Black is in check!"),
        }
    }

    pub fn display_checkmate(&self, color: &Color) {
        match color {
            Color::White => println!("White is in checkmate!"),
            Color::Black => println!("Black is in checkmate!"),
        }
    }

    pub fn display_stalemate(&self) {
        println!("Stalemate!");
    }

    pub fn display_invalid_move(&self) {
        println!("Invalid move!");
    }

}
