// Import necessary modules and dependencies
use crate::board::{Board, Color, in_bounds, Piece, PieceKind};
use crate::game::GameState;
use crate::player::Player;

// Enum to represent different types of castle moves
#[derive(Copy, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

// Enum to represent different types of moves
#[derive(Copy, Clone)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    Capture,
    Castle(CastleType),
    EnPassant,
    Promotion(PieceKind),
    PromotionCapture(PieceKind),
    Invalid,
    // ... any other special move types ...
}

// Struct to represent a move
#[derive(Copy, Clone)]
pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub move_type: MoveType,
    pub color: Color,
}

impl Move {
    // Function to create a new move
    pub fn new(from: (u8, u8), to: (u8, u8), move_type: MoveType, color: Color) -> Self {
        Self {
            from,
            to,
            move_type,
            color
        }
    }

}

// Struct to represent a move history
#[derive(Clone)]
pub struct MoveHistory {
    pub mv: Move,
    pub captured_piece: Option<Piece>,
    pub notation: String,
}

impl MoveHistory {
    // Function to create a new move history
    pub fn new(mv: Move) -> Self {
        let notation = create_notation_for_move(&mv);
        Self {
            mv,
            captured_piece: None,
            notation,
        }
    }
}

pub struct MoveGenerator<'a> {
    game_state: &'a GameState,
    board: &'a Board,
    player: &'a Player,
    moves: Vec<Move>,
}

impl<'a> MoveGenerator<'a,> {
    pub fn new(game_state: &'a GameState) -> Self {
        Self {
            game_state,
            board: &game_state.board,
            player: game_state.get_current_player(),
            moves: Vec::new(),
        }
    }

    // Function to generate all moves for a given color
    pub fn generate_moves(&mut self, color: Color) -> Vec<Move> {
        for (x, y, piece) in self.board.iter_pieces(color) {
            // Depending on the type of the piece, generate possible moves
            match piece.kind {
                PieceKind::Pawn => self.moves.extend(self.generate_pawn_moves(x, y, color)),
                PieceKind::Rook => self.moves.extend(self.generate_rook_moves(x, y, color)),
                PieceKind::Knight => self.moves.extend(self.generate_knight_moves(x, y, color)),
                PieceKind::Bishop => self.moves.extend(self.generate_bishop_moves(x, y, color)),
                PieceKind::Queen => self.moves.extend(self.generate_queen_moves(x, y, color)),
                PieceKind::King => self.moves.extend(self.generate_king_moves(x, y, color)),
                // ... similar for other piece types ...
            }
        }
        self.moves.clone()
    }

    // Function to generate all legal moves for a given game state
    pub fn generate_current_moves(&mut self) -> Vec<Move> {
        // For each piece belonging to the player
        let color = self.player.color;
        for (x, y, piece) in self.board.iter_pieces(color) {
            // Depending on the type of the piece, generate possible moves
            match piece.kind {
                PieceKind::Pawn => self.moves.extend(self.generate_pawn_moves(x, y, color)),
                PieceKind::Rook => self.moves.extend(self.generate_rook_moves(x, y, color)),
                PieceKind::Knight => self.moves.extend(self.generate_knight_moves(x, y, color)),
                PieceKind::Bishop => self.moves.extend(self.generate_bishop_moves(x, y, color)),
                PieceKind::Queen => self.moves.extend(self.generate_queen_moves(x, y, color)),
                PieceKind::King => self.moves.extend(self.generate_king_moves(x, y, color)),
                // ... similar for other piece types ...
            }
        }
        self.moves.clone()
    }

    // Function to generate all legal moves for a pawn at a given position
    fn generate_pawn_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);
        let direction = match color {
            Color::White => -1i8,
            Color::Black => 1i8,
        };

        // Moving one square forward
        let move_one_forward = (pos.0, (pos.1 as i8 + direction) as u8);
        if in_bounds(move_one_forward) && self.board.get(move_one_forward.0, move_one_forward.1).is_none() {
            moves.push(Move::new(pos, move_one_forward, MoveType::Normal,color));
        }

        // Moving two squares forward on the pawn's first move
        let move_two_forward = (pos.0, (pos.1 as i8 + 2 * direction) as u8);
        if self.board.get(pos.0, pos.1).unwrap().moves_count == 0 &&
           in_bounds(move_two_forward) &&
           self.board.get(move_two_forward.0, move_two_forward.1).is_none() {
             moves.push(Move::new(pos, move_two_forward, MoveType::Normal,color));
        }

        // Capturing diagonally
        let capture_moves = [(pos.0 + 1, (pos.1 as i8 + direction)as u8),
                             (pos.0 - 1, (pos.1 as i8 + direction)as u8)];
        for capture_move in capture_moves {
            if in_bounds(capture_move) {
                match self.board.get(capture_move.0, capture_move.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, capture_move, MoveType::Capture,color));
                        }
                    },
                    None => continue,
                }
            }
        }


        // Promotion: reaching the end of the board
        if (color == Color::White && pos.1 == 6) || (color == Color::Black && pos.1 == 1) {
            let promotion_moves = [(pos.0, pos.1 + 1), (pos.0 - 1, pos.1 + 1), (pos.0 + 1, pos.1 + 1)];
            for &pmv in &promotion_moves {
                // check if the position is valid
                if !in_bounds(pmv) {
                    continue;
                }
                // check if the position is occupied
                if !self.board.is_tile_empty(pmv) {
                    // check if the piece is an opponent's piece
                    if self.board.get(pmv.0, pmv.1).unwrap().color != color {
                        // generate promotion moves
                        let promotion_attack_moves = self.promotion_attack_move(color, pmv);
                        // add promotion moves to the list of moves
                        moves.extend(promotion_attack_moves);
                    }
                }
                else {
                    // generate promotion moves
                    let promotion_moves = self.promotion_move(color, pmv);
                    moves.extend(promotion_moves);
                }
            }
        }

        // En passant: capturing an opponent's pawn in passing
        if let Some(last_move) = self.game_state.move_history.last() {
            if let MoveType::DoublePawnPush = last_move.mv.move_type {
                let en_passant_moves = [(last_move.mv.to.0 + 1, last_move.mv.to.1), (last_move.mv.to.0 - 1, last_move.mv.to.1)];
                if en_passant_moves.contains(&pos) {
                    moves.push(Move::new(pos, (last_move.mv.to.0, (last_move.mv.to.1 as i8+ direction)as u8), MoveType::EnPassant,color));
                }
            }
        }
        moves
    }

    // Function to generate all legal moves for a king at a given position
    fn generate_king_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);

        // The king can move in 8 directions: up, down, left, right, and the 4 diagonals.
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for direction in directions.iter() {
            let new_x = (pos.0 as i8 + direction.0) as u8;
            let new_y = (pos.1 as i8 + direction.1) as u8;
            let move_to = (new_x, new_y);

            if in_bounds(move_to) {
                match self.board.get(move_to.0, move_to.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, move_to, MoveType::Capture, color));
                        }
                    },
                    None => {
                        moves.push(Move::new(pos, move_to, MoveType::Normal, color));
                    }
                }
            }
        }

        // Castle move generation
        // 1. The king hasn't moved before
        if self.board.get(pos.0, pos.1).unwrap().moves_count == 0 {
            // 2. The king is not currently in check
            if !self.game_state.is_in_check(color) {
                // 3. The rook with which the king is castling hasn't moved before
                // get rook position based on the king's color
                let file:u8 = if color == Color::White { 0 } else { 7 };
                let rook = self.board.get(file, pos.1).unwrap();
                if rook.moves_count == 0 {
                    // 4. There are no pieces between the king and the rook
                    let mut is_empty = true;
                    let mut is_attacked = false;
                    let mut x = pos.0;
                    let mut y = pos.1;
                    while x != file {
                        x = if x > file { x - 1 } else { x + 1 };
                        if !self.board.is_tile_empty((x, y)) {
                            is_empty = false;
                            break;
                        }
                        if self.game_state.is_attacked((x, y), color) {
                                is_attacked = true;
                                break;
                        }
                    }
                    if is_empty && !is_attacked {
                        // 5. The king does not pass through a square that is attacked by an enemy piece
                        if file == 0 {
                            moves.push(Move::new(pos, (pos.0 - 2, pos.1), MoveType::Castle(CastleType::QueenSide),color));
                        }
                        else {
                            moves.push(Move::new(pos, (pos.0 + 2, pos.1), MoveType::Castle(CastleType::KingSide), color));
                        }
                    }
                }
            }
        }
        moves
    }

    // Function to generate all legal moves for a rook at a given position
    fn generate_rook_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);

        // The rook can move in 4 directions: up, down, left, and right.
        let directions = [(-1, 0), (0, -1), (0, 1), (1, 0)];

        for direction in directions.iter() {
            let mut new_x = (pos.0 as i8 + direction.0) as u8;
            let mut new_y = (pos.1 as i8 + direction.1) as u8;
            let mut move_to = (new_x, new_y);

            while in_bounds(move_to) {
                match self.board.get(move_to.0, move_to.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, move_to, MoveType::Capture, color));
                        }
                        break;
                    },
                    None => {
                        moves.push(Move::new(pos, move_to, MoveType::Normal, color));
                    }
                }
                new_x = (move_to.0 as i8 + direction.0) as u8;
                new_y = (move_to.1 as i8 + direction.1) as u8;
                move_to = (new_x, new_y);
            }
        }
        moves
    }

    // Function to generate all legal moves for a knight at a given position
    fn generate_knight_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);

        // The knight can move in 8 directions: up, down, left, right, and the 4 diagonals.
        let directions = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

        for direction in directions.iter() {
            let new_x = (pos.0 as i8 + direction.0) as u8;
            let new_y = (pos.1 as i8 + direction.1) as u8;
            let move_to = (new_x, new_y);

            if in_bounds(move_to) {
                match self.board.get(move_to.0, move_to.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, move_to, MoveType::Capture, color));
                        }
                    },
                    None => {
                        moves.push(Move::new(pos, move_to, MoveType::Normal, color));
                    }
                }
            }
        }
        moves
    }

    // Function to generate all legal moves for a bishop at a given position
    fn generate_bishop_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);

        // The bishop can move in 4 directions: up-left, up-right, down-left, and down-right.
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for direction in directions.iter() {
            let mut new_x = (pos.0 as i8 + direction.0) as u8;
            let mut new_y = (pos.1 as i8 + direction.1) as u8;
            let mut move_to = (new_x, new_y);

            while in_bounds(move_to) {
                match self.board.get(move_to.0, move_to.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, move_to, MoveType::Capture, color));
                        }
                        break;
                    },
                    None => {
                        moves.push(Move::new(pos, move_to, MoveType::Normal, color));
                    }
                }
                new_x = (move_to.0 as i8 + direction.0) as u8;
                new_y = (move_to.1 as i8 + direction.1) as u8;
                move_to = (new_x, new_y);
            }
        }
        moves
    }

    // Function to generate all legal moves for a queen at a given position
    fn generate_queen_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);

        // The queen can move in 8 directions: up, down, left, right, and the 4 diagonals.
        let directions = [(-1, 0), (0, -1), (0, 1), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

        for direction in directions.iter() {
            let mut new_x = (pos.0 as i8 + direction.0) as u8;
            let mut new_y = (pos.1 as i8 + direction.1) as u8;
            let mut move_to = (new_x, new_y);

            while in_bounds(move_to) {
                match self.board.get(move_to.0, move_to.1) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(Move::new(pos, move_to, MoveType::Capture, color));
                        }
                        break;
                    },
                    None => {
                        moves.push(Move::new(pos, move_to, MoveType::Normal, color));
                    }
                }
                new_x = (move_to.0 as i8 + direction.0) as u8;
                new_y = (move_to.1 as i8 + direction.1) as u8;
                move_to = (new_x, new_y);
            }
        }
        moves
    }

    // Pushes all promotion piece types moves to the list of moves
    pub fn promotion_move(&self, color: Color, pmv: (u8, u8)) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.push(Move::new(pmv, pmv, MoveType::Promotion(PieceKind::Queen),color));
        moves.push(Move::new(pmv, pmv, MoveType::Promotion(PieceKind::Rook),color));
        moves.push(Move::new(pmv, pmv, MoveType::Promotion(PieceKind::Bishop),color));
        moves.push(Move::new(pmv, pmv, MoveType::Promotion(PieceKind::Knight),color));
        moves
    }

    // Pushes all promotion piece types attack moves to the list of moves
    pub fn promotion_attack_move(&self, color: Color, pmv: (u8, u8)) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.push(Move::new(pmv, pmv, MoveType::PromotionCapture(PieceKind::Queen),color));
        moves.push(Move::new(pmv, pmv, MoveType::PromotionCapture(PieceKind::Rook),color));
        moves.push(Move::new(pmv, pmv, MoveType::PromotionCapture(PieceKind::Bishop),color));
        moves.push(Move::new(pmv, pmv, MoveType::PromotionCapture(PieceKind::Knight),color));
        moves
    }

}

pub fn validate_move(game_state: &GameState, pos: (u8,u8,u8,u8)) -> Result<(Move), String> {
    let mut move_generator = MoveGenerator::new(&game_state);
    let moves = move_generator.generate_current_moves();
    for mv in moves {
        if mv.from == (pos.0, pos.1) && mv.to == (pos.2, pos.3) {
            return Ok(mv);
        }
    }
    Err("Invalid move".to_string())
}


// Function to get a move from the user
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

pub fn create_notation_for_move(mv: &Move) -> String {
    let mut notation = String::new();
    let from = (mv.from.0 as char, mv.from.1 as char);
    let to = (mv.to.0 as char, mv.to.1 as char);
    notation.push(from.0);
    notation.push(from.1);
    notation.push('-');
    notation.push(to.0);
    notation.push(to.1);
    notation
}