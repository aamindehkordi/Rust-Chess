use std::time::Duration;
// Import necessary modules and dependencies
use crate::board::{Board, Color, king_pos, Piece, PieceKind};
use crate::player::Player;
use crate::moves::{Move, MoveError, MoveGenerator, MoveHistory};
use crate::moves::MoveType::{Promotion, PromotionCapture};
use crate::player::PlayerKind::Human;

#[derive(Clone)]
pub enum GameStatus {
    InProgress,
    Check(Color),
    Checkmate(Color),
    Draw,
}impl GameStatus {
    pub fn in_check(&self) -> bool {
        match self {
            GameStatus::Check(_) => true,
            _ => false,
        }
    }
    pub fn is_checkmate(&self) -> bool {
        match self {
            GameStatus::Checkmate(_) => true,
            _ => false,
        }
    }
    pub fn is_draw(&self) -> bool {
        match self {
            GameStatus::Draw => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Timer {
    player1_time: Option<Duration>,
    player2_time: Option<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            player1_time: Some(Duration::new(0, 0)),
            player2_time: Some(Duration::new(0, 0)),
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub players: [Player; 2],
    pub current_player: usize,  // index into players array
    pub move_history: Vec<MoveHistory>,
    pub all_moves: Vec<Move>,
    pub game_status: GameStatus,
    pub timers: Timer,
}

impl GameState {
    // Function to create a new game state
    pub fn new() -> Self {
        let board = Board::new_standard();
        let players = [
            Player::new("Player 1".to_string(), Human, Color::White),
            Player::new("Player 2".to_string(), Human, Color::Black),
        ];
        let current_player = 0;
        let move_history = Vec::new();
        let mut all_moves = Vec::new();

        Self {
            board,
            players,
            current_player,
            move_history,
            all_moves,
            game_status: GameStatus::InProgress,
            timers: Timer::new(),
        }
    }

}

pub fn calculate_all_moves(game_state: &mut GameState) {
    game_state.all_moves.clear();
    game_state.all_moves.extend(calculate_black_moves(game_state));
    game_state.all_moves.extend(calculate_white_moves(game_state));
}

pub fn change_current_player(game_state: &mut GameState) {
    game_state.current_player = 1 - game_state.current_player;
}
pub fn calculate_white_moves(game_state: &GameState) -> Vec<Move> {

    let mut move_generator = MoveGenerator::new(&game_state);
    move_generator.generate_moves(Color::White)
}

pub fn calculate_black_moves(game_state: &GameState) -> Vec<Move> {
    let mut move_generator = MoveGenerator::new(&game_state);
    move_generator.generate_moves(Color::Black)
}

pub fn apply_move(game_state: &GameState, mv: &Move) -> GameState {
    let mut new_game_state = game_state.clone();
    new_game_state.board.make_move(mv);
    new_game_state.move_history.push(MoveHistory::new(mv.clone()));
    calculate_all_moves(&mut new_game_state);
    change_current_player(&mut new_game_state);
    new_game_state
}

pub fn get_current_player(game_state: &GameState) -> &Player {
    &game_state.players[game_state.current_player]
}

pub fn is_attacked(game_state: &GameState, pos: (u8, u8), color: Color) -> bool {
    // filter all opponent moves from self.all_moves
    let moves: Vec<Move> = game_state.all_moves.iter().filter(|mv| mv.color != color).map(|mv| mv.clone()).collect();
    // check if any of the opponent moves attack the given position
    for mv in moves {
        if mv.to == pos && mv.color != color {
            return true;
        }
    }
    false
}

pub fn is_in_check(game_state: &GameState, color: Color) -> bool {
    let king_pos = king_pos(&game_state.board, color);
    let moves: Vec<Move> = game_state.all_moves.iter().filter(|mv| mv.color != color).map(|mv| mv.clone()).collect();
    for mv in moves {
        if mv.to == king_pos {
            if mv.move_type.is_valid() {
                return true;
            }
        }
    }
    false
}

pub fn will_block_check(game_state: &GameState, mv: Move) -> bool {
    let mut gs_copy = game_state.clone();
    gs_copy = apply_move(&mut gs_copy, &mv);
    if is_in_check(&gs_copy, mv.color) {
        return false;
    }
    true
}

pub fn is_in_checkmate(game_state: &GameState, color:Color) -> bool {
    if is_in_check(game_state, color) {
        let mut move_generator = MoveGenerator::new(&game_state);
        let moves = move_generator.generate_current_moves();
        if moves.len() == 0 {
            return true;
        }
    }
    false
}

pub fn is_current_player_in_check(game_state: &GameState) -> bool {
    if is_in_check(game_state, get_current_player(game_state).color) {
        return true;
    }
    false
}

pub fn validate_move(game_state: &GameState, pos: (u8,u8,u8,u8)) -> Result<Move, MoveError> {
    let game_status = &game_state.game_status;
    let mut move_generator = MoveGenerator::new(&game_state);
    let moves = move_generator.generate_current_moves();
    for mv in moves {
        if mv.from == (pos.0, pos.1) && mv.to == (pos.2, pos.3) {
            if game_status.in_check() && !will_block_check(game_state, mv.clone()) {
                return Err(MoveError::MoveDoesNotBlockCheck);
            }
            if !mv.move_type.is_valid() {
                return Err(MoveError::MoveIsNotValid);
            }
            if mv.move_type.is_promotion() || mv.move_type.is_promo_capture() {
                let promotion = ask_for_promotion();
                let piece = parse_promotion(&promotion);
                let mv_type = if mv.move_type.is_promotion() {
                    Promotion(piece)
                } else {
                    PromotionCapture(piece)
                };
                let mv = Move::new(mv.from, mv.to, mv_type, mv.color);
                return Ok(mv);
            }
            return Ok(mv);
        }
    }
    Err(MoveError::Other("Invalid move".to_string()))
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
pub fn get_user_move() -> (u8, u8, u8, u8) {
    let mut input = String::new();
    println!("Enter your move: (e2-e4) ");
    std::io::stdin().read_line(&mut input).unwrap();

    // Temporarily Parse the move manually for now
    let chars = input.chars().collect::<Vec<char>>();
    let from = (chars[0] as u8 - 'a' as u8, chars[1] as u8 - '1' as u8);
    let to = (chars[3] as u8 - 'a' as u8, chars[4] as u8 - '1' as u8);

    (from.0, from.1, to.0, to.1)
}

// Function to parse a move from a string according to the algebraic notation
// For example, the move "e4" would be parsed as a the current turn's pawn on e2 moving to e4
// The move "Nf3" would be parsed as the current turn's knight on b1 moving to f3
// The move "Bxe5" would be parsed as the current turn's bishop on c3 capturing the opponent's pawn on e5
pub fn parse_move(input: &str) -> (u8, u8, u8, u8) {
    // Error handling
    let invalid = (0, 0, 0, 0);
    // ... parse the move ...
    invalid
}

// Function to get a move from an AI
pub fn get_ai_move(player: &Player) -> (u8, u8, u8, u8) {
    // ... get a move from the AI ...
    (0, 0, 0, 0)
}


// Function to display the game state
pub fn display_game_state(game_state: &GameState) {
    println!("{}", game_state.board);
}

pub fn is_game_over(game_state: &GameState) -> bool {
    if game_state.all_moves.len() == 0 {
        return true;
    }
    false
}