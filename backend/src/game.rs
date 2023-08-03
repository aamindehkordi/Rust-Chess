use std::collections::HashMap;
// Import necessary modules and dependencies
use crate::board::{Board, Color, from_fen, in_bounds, king_pos, make_move, Piece, PieceKind, unmake_move};
use crate::player::Player;
use crate::moves::{Move, MoveError, MoveGenerator, MoveHistory, MoveType};
use crate::moves::MoveType::{Normal, Promotion, PromotionCapture};
use crate::player::PlayerKind::Human;

#[derive(Clone)]
pub struct CheckState {
    pub white_in_check: bool,
    pub black_in_check: bool,
    pub attacked_by_white: HashMap<(u8, u8), Vec<(u8, u8)>>,
    pub attacked_by_black: HashMap<(u8, u8), Vec<(u8, u8)>>,
    pub king_rays: HashMap<(u8, u8), Vec<(u8, u8)>>,
}

impl CheckState {
    fn new() -> Self {
        Self {
            white_in_check: false,
            black_in_check: false,
            attacked_by_white: HashMap::new(),
            attacked_by_black: HashMap::new(),
            king_rays: HashMap::new(),


        }
    }
    pub fn in_check(&self) -> bool {
        self.white_in_check || self.black_in_check
    }
}
pub fn calculate_king_rays(game_state: &GameState, color: Color) -> HashMap<(u8, u8), Vec<(u8, u8)>> {
    let mut king_rays = HashMap::new();
    let king_pos = king_pos(&game_state.board, color);
    // get all theoretical lines that the king can get attacked from without move generator
    let file: Vec<(u8, u8)> = (0..8).map(|x| (x, king_pos.1)).collect();
    let rank: Vec<(u8, u8)> = (0..8).map(|y| (king_pos.0, y)).collect();
    let diag1: Vec<(u8, u8)> = (0..8).map(|i| (king_pos.0 + i, king_pos.1 + i)).take_while(|(x, y)| in_bounds(*x, *y)).collect();
    let diag2: Vec<(u8, u8)> = (0..8).map(|i| (king_pos.0 + i, king_pos.1 - i)).take_while(|(x, y)| in_bounds(*x, *y)).collect();
    let diag3: Vec<(u8, u8)> = (0..8).map(|i| (king_pos.0 - i, king_pos.1 + i)).take_while(|(x, y)| in_bounds(*x, *y)).collect();
    let diag4: Vec<(u8, u8)> = (0..8).map(|i| (king_pos.0 - i, king_pos.1 - i)).take_while(|(x, y)| in_bounds(*x, *y)).collect();

    king_rays.insert(king_pos, file);
    king_rays.insert(king_pos, rank);
    king_rays.insert(king_pos, diag1);
    king_rays.insert(king_pos, diag2);
    king_rays.insert(king_pos, diag3);
    king_rays.insert(king_pos, diag4);

    king_rays
}
pub fn calculate_check_state(game_state: &GameState) -> CheckState {
    let mut check_state = CheckState::new();
    let color = get_current_player(&game_state).color;
    check_state.king_rays = calculate_king_rays(game_state, color);
    // Loop through all pieces on the board
    for (x, y, piece) in game_state.board.iter_pieces(color) {
        let mut gen = MoveGenerator::new(game_state);
        let mut moves = match piece.kind {
            PieceKind::Pawn => gen.generate_pawn_moves(x, y, piece.color),
            PieceKind::Knight => gen.generate_knight_moves(x, y, piece.color),
            PieceKind::Bishop => gen.generate_sliding_move(x, y, piece.color),
            PieceKind::Rook => gen.generate_sliding_move(x, y, piece.color),
            PieceKind::Queen => gen.generate_sliding_move(x, y, piece.color),
            PieceKind::King => gen.generate_king_moves(x, y, piece.color),
        };

        // Loop through all moves for the piece
        for mv in moves {
            if mv.piece.kind == PieceKind::Pawn && (mv.move_type.is_promotion() || mv.move_type.is_normal()) {
                continue;
            }
            if mv.color == Color::White {
                check_state.attacked_by_white.entry(mv.to).or_insert(Vec::new()).push((x, y));
            }
            if mv.color == Color::Black {
                check_state.attacked_by_black.entry(mv.to).or_insert(Vec::new()).push((x, y));
            }
            // If the move is a capture, check if it is a check
            if mv.is_capture() {
                let (x2, y2) = mv.to;
                let piece2 = game_state.board.get(x2, y2).unwrap();
                if piece2.kind == PieceKind::King {
                    match piece2.color {
                        Color::White => check_state.white_in_check = true,
                        Color::Black => check_state.black_in_check = true,
                    }
                }
            }
        }
    }

    check_state
}

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub players: [Player; 2],
    pub current_player: usize,  // index into players array
    pub move_history: Vec<MoveHistory>,
    pub all_moves: Vec<Move>,
    pub check_state: CheckState,
}


impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        let players = [
            Player::new("Player 1".to_string(), Human, Color::White),
            Player::new("Player 2".to_string(), Human, Color::Black),
        ];
        let current_player = 0;
        let move_history = Vec::new();
        let all_moves = Vec::new();

        Self {
            board,
            players,
            current_player,
            move_history,
            all_moves,
            check_state: CheckState::new(),
        }
    }

    // Function to create a new game state
    pub fn new_standard() -> Self {
        let board = Board::new_standard();
        let players = [
            Player::new("Player 1".to_string(), Human, Color::White),
            Player::new("Player 2".to_string(), Human, Color::Black),
        ];
        let current_player = 0;
        let move_history = Vec::new();
        let all_moves = Vec::new();

        Self {
            board,
            players,
            current_player,
            move_history,
            all_moves,
            check_state: CheckState::new(),
        }
    }

    pub fn new_from_fen(fen: &str) -> Self {
        let board = from_fen(fen);
        let players = [
            Player::new("Player 1".to_string(), Human, Color::White),
            Player::new("Player 2".to_string(), Human, Color::Black),
        ];
        let current_player = 0;
        let move_history = Vec::new();
        let all_moves = Vec::new();

        Self {
            board,
            players,
            current_player,
            move_history,
            all_moves,
            check_state: CheckState::new(),
        }
    }

    pub fn temp(board: Board, players: [Player; 2], current_player: usize, move_history: Vec<MoveHistory>, all_moves: Vec<Move>, check_state: CheckState) -> Self {
        Self {
            board,
            players,
            current_player,
            move_history,
            all_moves,
            check_state,
        }
    }
}

pub fn change_current_player(game_state: &mut GameState) {
    game_state.current_player = 1 - game_state.current_player;
}

pub fn calculate_all_moves(game_state: &mut GameState) {
    game_state.all_moves.clear();
    calculate_black_moves(game_state);
    calculate_white_moves(game_state);
}
pub fn calculate_white_moves(game_state: &mut GameState) {
    let mut move_generator = MoveGenerator::new(game_state);
    let moves = move_generator.generate_moves(Color::White);
    game_state.all_moves.extend(moves);
}

pub fn calculate_black_moves(game_state: &mut GameState) {
    let mut move_generator = MoveGenerator::new(game_state);
    let moves = move_generator.generate_moves(Color::Black);
    game_state.all_moves.extend(moves);
}

pub fn apply_move(game_state: GameState, mv: &Move) -> GameState {
    let mut new_game_state = game_state.clone();
    new_game_state.board = make_move(new_game_state.board, mv);
    new_game_state.move_history.push(MoveHistory::new(*mv));
    change_current_player(&mut new_game_state);
    new_game_state
}

pub fn undo_move(game_state: GameState) -> GameState {
    let mut new_game_state = game_state.clone();
    let last_move = new_game_state.move_history.pop().unwrap();
    new_game_state.board = unmake_move(new_game_state.board, &last_move.mv);
    change_current_player(&mut new_game_state);
    new_game_state
}

pub fn get_current_player(game_state: &GameState) -> &Player {
    &game_state.players[game_state.current_player]
}

pub fn is_attacked(game_state: &GameState, pos: (u8, u8), color: Color) -> bool {
    let mut moves = game_state.all_moves.clone();
    // filter all opponent moves from self.all_moves
    moves.retain(|mv| mv.color != color);
    // check if any of the opponent moves attack the given position
    for mv in moves {
        if mv.to == pos {
            return true;
        }
    }

    false
}

pub fn is_in_check(game_state: &GameState, color: Color) -> bool {
    let king_pos = king_pos(&game_state.board, color);
    let moves: Vec<Move> = game_state.all_moves.iter().filter(|mv| mv.color != color).copied().collect();
    for mv in moves {
        if mv.to == king_pos {
            // if pawn moving forward or promoting, then not in check
            if let Some(piece) = game_state.board.get(mv.from.0, mv.from.1) {
                if piece.kind == PieceKind::Pawn && (mv.to.1 as i8 - mv.from.1 as i8).abs() == 1 && (mv.move_type.is_promotion() || mv.move_type.is_normal()){
                    continue;
                }
            }
            return true;
        }
    }
    false
}

pub fn mv_will_block_check(game_state: &GameState, mv: Move) -> bool {
    let mut gs_copy = game_state.clone();
    gs_copy = apply_move(gs_copy, &mv);
    gs_copy.check_state = calculate_check_state(&mut gs_copy);
    if gs_copy.check_state.in_check() {
        return false;
    }
    true
}

pub fn is_in_checkmate(game_state: &GameState, color:Color) -> bool {
    if is_in_check(game_state, color) {
        let mut move_generator = MoveGenerator::new(game_state);
        let moves = move_generator.generate_current_moves();
        if moves.is_empty() {
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

pub fn is_draw(game_state: &GameState) -> bool {
    let mut move_generator = MoveGenerator::new(game_state);
    let moves = move_generator.generate_current_moves();
    if moves.is_empty() {
        return true;
    }
    false
}

pub fn validate_move(game_state: &GameState, pos: (u8,u8,u8,u8)) -> Result<Move, MoveError> {
    let moves = game_state.all_moves.clone();
    let piece = game_state.board.get(pos.0, pos.1).unwrap();
    for mv in moves {
        if mv.from == (pos.0, pos.1) && mv.to == (pos.2, pos.3) {
            if !mv_will_block_check(game_state, mv) {
                return Err(MoveError::MoveDoesNotBlockCheck);
            }
            if !in_bounds(mv.to.0, mv.to.1) {
                return Err(MoveError::MoveIsNotValid);
            }
            if mv.move_type.is_promotion() || mv.move_type.is_promo_capture() {
                let promotion = ask_for_promotion();
                let piece_kind = parse_promotion(&promotion);
                let mv_type = if mv.move_type.is_promotion() {
                    Promotion(piece_kind)
                } else {
                    PromotionCapture(piece_kind)
                };
                let mv = Move::new(mv.from, mv.to, mv_type, piece, mv.color);
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


// Function to display the game state
pub fn display_game_state(game_state: &GameState) {
    println!("{}", game_state.board);
}

pub fn is_game_over(game_state: &GameState) -> bool {
    if game_state.all_moves.is_empty() {
        return true;
    }
    false
}