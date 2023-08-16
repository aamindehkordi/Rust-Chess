use crate::board::*;
use crate::moves::move_gen::*;
use crate::moves::FromTo;

/// A game is a board and a turn.
pub struct Game {
    pub board: Board,
}
impl Game {
    /// Creates a new game.
    ///
    /// # Returns
    /// A new game with an empty initialized board and the starting turn set to white.
    pub fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    /// Creates a new game.
    ///
    /// # Returns
    /// A new game with an initialized standard board and the starting turn set to white.
    ///
    /// # Example
    /// ```rs
    ///     let game = Game::new();
    /// ```
    pub fn new_standard() -> Game {
        Game {
            board: Board::new_standard(),
        }
    }

    pub fn custom(fen: &str) -> Game {
        Game {
            board: new_board_from_fen(fen),
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
        loop {
            println!("{}", self.board);
            println!("{}'s turn. \nEnter a move (eg. e2 e4):\n", self.board.turn);
            let input = get_user_input();
            let parsed_input = parse_user_input(input);
            match parsed_input {
                Some((from, to)) => {
                    let piece = self.board.squares[from].piece;
                    if piece.to_byte() == 0 {
                        println!("No piece at {}.", from);
                        continue;
                    }
                    let piece = piece;
                    if piece.color.expect("how") != self.board.turn {
                        println!("It is not {}'s turn.", piece.color.unwrap());
                        continue;
                    }
                    let moves = generate_legal_moves(&self.board);
                    if !moves.contains(&(from, to)) {
                        println!("Illegal move.");
                        continue;
                    }
                    self.board.make_simple_move((from, to));
                    if self.is_over() {
                        println!("Game over.");
                        break;
                    }
                }
                None => {
                    println!("Invalid input.");
                    continue;
                }
            }
        }
    }

    /// Checks if the game is over.
    ///
    /// # Returns
    /// True if the game is over, false otherwise.
    pub fn is_over(&self) -> bool {
        self.board.is_checkmate()
    }
}

/// Gets user input from the command line.
///
/// # Returns
/// The user input.
pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

/// Parses user input into a simple move.
///
/// # Arguments
/// * `input` - The user input.
///
/// # Returns
/// A simple move if the input is valid, None otherwise.
pub fn parse_user_input(input: String) -> Option<FromTo> {
    let mut input = input.split_whitespace(); // a2 a4
    let from = input.next()?; // a2
    let to = input.next()?; // a4
    let from = from.chars().collect::<Vec<char>>(); // a2 -> [a, 2]
    let to = to.chars().collect::<Vec<char>>(); // a4 -> [a, 4]
    let from = (from[0] as u8 - 97, from[1] as u8 - 49); // a1 -> (0, 0)
    let to = (to[0] as u8 - 97, to[1] as u8 - 49); // a4 -> (0, 3)
    let from = idx(from.1 as usize, from.0 as usize); // (0, 0) -> 0
    let to = idx(to.1 as usize, to.0 as usize); // (0, 3) -> 3
    Some((from, to))
}
