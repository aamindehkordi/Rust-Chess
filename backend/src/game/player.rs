use crate::board::piece::PieceKind;
use crate::board::Board;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn other(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone)]
pub struct Timer {
    pub time: Duration,
    pub increment: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: Duration::new(0, 0),
            increment: Duration::new(0, 0),
        }
    }

    pub fn increment(&mut self) {
        self.time += self.increment;
    }

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
    pub fn new(color: Color) -> Self {
        Self {
            name: "".to_string(),
            kind: PlayerKind::Human,
            timer: Timer::new(),
            color,
        }
    }
}

pub fn ask_for_promotion() -> String {
    let mut input = String::new();
    println!("Enter your promotion: (Q, R, B, N) ");
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

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
pub fn parse_move(_input: &str) -> (u8, u8, u8, u8) {
    // Error handling

    // ... parse the move ...
    (0, 0, 0, 0)
}

// Function to get a move from an AI
pub fn ai_mv_idx(_player: &Player) -> (u8, u8, u8, u8) {
    // ... get a move from the AI ...
    (0, 1, 2, 2)
}
