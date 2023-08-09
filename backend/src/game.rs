use crate::board::*;
use crate::piece::*;

/// A game is a board and a turn.
pub struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Default for Game {
    /// Returns the default value for a square.
    ///
    /// # Returns
    /// The default square.
    ///
    /// # Example
    /// ```rs
    ///     let square = Square::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Creates a new game.
    ///
    /// # Returns
    /// A new game with an initialized standard board and the starting turn set to white.
    ///
    /// # Example
    /// ```rs
    ///     let game = Game::new();
    /// ```
    pub fn new() -> Game {
        Game {
            board: Board::new_standard(),
            turn: Color::White,
        }
    }

    /// Plays a move on the chess board.
    ///
    /// This function prints the current state of the chess board.
    ///
    /// # Example
    /// ```rs
    ///     chess_board.play();
    /// ```
    pub fn play(&mut self) {
        println!("{}", self.board);
    }
}
