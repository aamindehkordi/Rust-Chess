use crate::board::piece::PieceKind;
use crate::board::Board;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
impl Color {
    /**
     * Returns the opposite color of the current color.
     *
     * This function returns the opposite color of the current color. If the current color is `Color::White`,
     * it will return `Color::Black`. If the current color is `Color::Black`, it will return `Color::White`.
     *
     * @return The opposite color of the current color.
     */
    pub fn other(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /**
     * Converts the Color enum variant to its corresponding index.
     *
     * This function returns the index representation of the Color enum variant.
     *
     * @returns The index representation of the Color enum variant.
     */
    pub fn to_idx(self) -> u8 {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

/**
 * Creates a Color enum value from an index value.
 *
 * This function takes an index value and returns the corresponding Color enum value. The index
 * should be 0 for white color, 1 for black color, otherwise it panics with an "Invalid color index" error.
 *
 * @param idx - The index value representing the color.
 * @returns The corresponding Color enum value.
 */
pub fn from_idx(idx: u8) -> Color {
    match idx {
        0 => Color::White,
        1 => Color::Black,
        _ => panic!("Invalid color index"),
    }
}

#[derive(Clone)]
pub struct Timer {
    pub time: Duration,
    pub increment: Duration,
}

impl Timer {
    /**
     * Creates a new instance of the ChessClock struct.
     *
     * This function initializes and returns a new instance of the ChessClock struct with default values for time and increment.
     *
     * @return A new instance of the ChessClock struct.
     */
    pub fn new() -> Self {
        Self {
            time: Duration::new(0, 0),
            increment: Duration::new(0, 0),
        }
    }

    /**
     * Increment the current time by the specified increment value.
     *
     * This function increments the current time value in the Timer struct by the specified increment value.
     * The updated time is stored in the 'time' field of the Timer struct.
     */
    pub fn increment(&mut self) {
        self.time += self.increment;
    }

    /**
     * Resets the Chess Clock.
     *
     * This function resets the time on the Chess Clock to zero.
     */
    pub fn reset(&mut self) {
        self.time = Duration::new(0, 0);
    }
}

#[derive(Clone)]
pub enum PlayerKind {
    Human,
    Computer(Brain),
}

#[derive(Clone)]
pub struct Brain {
    pub board: Board,
    pub color: Color,
}

impl Brain {
    /**
     * Creates a new instance of the `ChessGame` struct.
     *
     * This function initializes a new `ChessGame` object with the specified initial board state and player color.
     *
     * @param board - The initial board state for the chess game.
     * @param color - The player color for the chess game.
     *
     * @return A new instance of the `ChessGame` struct.
     */
    pub fn new(board: Board, color: Color) -> Self {
        Self { board, color }
    }
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub timer: Timer,
    pub color: Color,
}

impl Player {
    /**
     * Creates a new instance of the Player struct.
     *
     * This function initializes a new Player struct with the specified color.
     *
     * @param color - The color of the player.
     * @return A new instance of the Player struct.
     */
    pub fn new(color: Color) -> Self {
        Self {
            name: "".to_string(),
            kind: PlayerKind::Human,
            timer: Timer::new(),
            color,
        }
    }
}

/**
 * Prompts the user to enter a promotion piece and returns the input.
 *
 * This function displays a prompt asking the user to enter a promotion piece (Queen, Rook, Bishop, Knight)
 * and reads the user's input from the standard input. The input is returned as a String.
 *
 * @return The user's input as a String.
 */
pub fn ask_for_promotion() -> String {
    let mut input = String::new();
    println!("Enter your promotion: (Q, R, B, N) ");
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

/**
 * Parses the input string to determine the piece kind for promotion.
 *
 * This function takes an input string and returns the corresponding PieceKind enum value
 * based on the string. If the input string matches one of the promotion options, the
 * corresponding PieceKind enum value is returned. Otherwise, PieceKind::Queen is returned
 * as the default option.
 *
 * @param input - The input string representing the desired promotion piece.
 * @return The PieceKind enum value corresponding to the input string.
 */
pub fn parse_promotion(input: &str) -> PieceKind {
    match input {
        "Q" | "q" => PieceKind::Queen,
        "R" | "r" => PieceKind::Rook,
        "B" | "b" => PieceKind::Bishop,
        "N" | "n" => PieceKind::Knight,
        _ => PieceKind::Queen,
    }
}

// Function to get a move from the user and parse it into a Move struct
/**
 * Reads user input and returns the move indices.
 *
 * This function prompts the user to enter their move in the format "e2-e4" and reads their input
 * from the standard input. It then parses the input and extracts the indices for the 'from' and 'to'
 * squares of the move.
 *
 * @return A tuple containing the 'from' and 'to' indices as (from_file, from_rank, to_file, to_rank).
 */
pub fn user_mv_idx() -> (u8, u8, u8, u8) {
    let mut input = String::new();
    println!("Enter your move: (e2-e4) ");
    std::io::stdin().read_line(&mut input).unwrap();

    // Temporarily Parse the move manually for now
    let chars = input.chars().collect::<Vec<char>>();
    let from = (chars[0] as u8 - b'a', chars[1] as u8 - b'1');
    let to = (chars[3] as u8 - b'a', chars[4] as u8 - b'1');

    (from.0, from.1, to.0, to.1)
}

// Function to parse a move from a string according to the algebraic notation
// For example, the move "e4" would be parsed as a the current turn's pawn on e2 moving to e4
// The move "Nf3" would be parsed as the current turn's knight on b1 moving to f3
// The move "Bxe5" would be parsed as the current turn's bishop on c3 capturing the opponent's pawn on e5
/**
 * Parses a chess move from an input string.
 *
 * This function takes an input string and attempts to parse a chess move from it. It returns
 * a tuple representing the origin and destination squares of the move.
 *
 * @param _input - The input string to parse the chess move from.
 * @return A tuple containing the origin and destination squares of the move.
 *
 * TODO: Implement the parsing logic for the chess move.
 */
pub fn parse_move(_input: &str) -> (u8, u8, u8, u8) {
    // Error handling

    // ... parse the move ...
    (0, 0, 0, 0)
}

// Function to get a move from an AI
/**
 * AI Move Index
 *
 * This function determines the move to be made by the AI player and returns the indices of the
 * source and destination squares as a tuple of four `u8` values.
 *
 * @param _player - The player for which the AI move is being calculated.
 * @return A tuple containing the indices of the source and destination squares in the form
 *         (src_row, src_col, dest_row, dest_col).
 */
pub fn ai_mv_idx(_player: &Player) -> (u8, u8, u8, u8) {
    // ... get a move from the AI ...
    (0, 1, 2, 2)
}
