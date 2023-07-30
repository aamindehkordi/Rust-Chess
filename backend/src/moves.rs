use std::cmp::{max, min};
// Import necessary modules and dependencies
use crate::board::{Board, Color, in_bounds, is_tile_empty, Piece, PieceKind};
use crate::game::{GameState, get_current_player, is_attacked, is_in_check};
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
}

impl MoveType {
    // Function to check if a move type is a promotion
    pub fn is_promotion(&self) -> bool {
        match self {
            MoveType::Promotion(_) => true,
            _ => false,
        }
    }

    // Function to check if a move type is a capture
    pub fn is_promo_capture(&self) -> bool {
        match self {
            MoveType::PromotionCapture(_) => true,
            _ => false,
        }
    }

    // Function to check if a move type is valid
    pub fn is_valid(&self) -> bool {
        match self {
            MoveType::Invalid | MoveType::PromotionCapture(_) | MoveType::Promotion(_) => false,
            _ => true,
        }
    }
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
            player: get_current_player(game_state),
            moves: Vec::new(),
        }
    }

    // Function to generate all moves for a given color
    pub fn generate_moves(&mut self, color: Color) -> Vec<Move> {
        for (x, y, piece) in self.board.iter_pieces(color) {
            // Depending on the type of the piece, generate possible moves
            match piece.kind {
                PieceKind::King => self.moves.extend(self.generate_king_moves(x, y, color)),
                PieceKind::Pawn => self.moves.extend(self.generate_pawn_moves(x, y, color)),
                PieceKind::Rook => self.moves.extend(self.generate_rook_moves(x, y, color)),
                PieceKind::Knight => self.moves.extend(self.generate_knight_moves(x, y, color)),
                PieceKind::Bishop => self.moves.extend(self.generate_bishop_moves(x, y, color)),
                PieceKind::Queen => self.moves.extend(self.generate_queen_moves(x, y, color)),
            }
        }
        self.moves.clone()
    }

    // Function to generate all legal moves for a given game state
    pub fn generate_current_moves(&mut self) -> Vec<Move> {
        // For each piece belonging to the player
        let color = self.player.color;
        self.moves = self.generate_moves(color);
        self.moves.clone()
    }

    // Function to generate all legal moves for a pawn at a given position
    fn generate_pawn_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let fmv = (x, y);

        if fmv == (1, 6) {
            println!("pawn at 1,6");
        }
        let direction = match color {
            Color::White => 1i8,
            Color::Black => -1i8,
        };
        let second_file = if color == Color::White { 1u8 } else { 6u8 };
        let seventh_file = if color == Color::White { 6u8 } else { 1u8 };

        if fmv.1 != seventh_file {
            // Moving one square forward
            let move_one_forward = (fmv.0, (fmv.1 as i8 + direction) as u8);
            if in_bounds(move_one_forward) && self.board.get(move_one_forward.0, move_one_forward.1).is_none() {
                moves.push(Move::new(fmv, move_one_forward, MoveType::Normal, color));
            }

            // Moving two squares forward on the pawn's first move
            let move_two_forward = (fmv.0, (fmv.1 as i8 + 2 * direction) as u8);
            if self.board.get(fmv.0, fmv.1).unwrap().moves_count == 0 && // first move
               in_bounds(move_two_forward) && // in bounds
               self.board.get(move_two_forward.0, move_two_forward.1).is_none() && // no piece in the way
               fmv.1 == second_file { // on the second file
                 moves.push(Move::new(fmv, move_two_forward, MoveType::DoublePawnPush, color));
            }

            // Capturing diagonally
            let capture_left = ((fmv.0 as i8 - 1) as u8, (fmv.1 as i8 + direction) as u8);
            if in_bounds(capture_left) {
                match self.board.get(capture_left.0, capture_left.1) {
                    Some(piece) => {
                        if piece.color != color && fmv.1 != seventh_file {
                            moves.push(Move::new(fmv, capture_left, MoveType::Capture, color));
                        }
                    },
                    None => {
                        // En passant: capturing an opponent's pawn in passing
                        if let Some(last_move) = self.game_state.move_history.last() {
                            if let MoveType::DoublePawnPush = last_move.mv.move_type {
                                if last_move.mv.to.1 == y &&
                                   (last_move.mv.to.0 as i8 - fmv.0 as i8).abs() == 1 {
                                    let en_passant_move = (last_move.mv.to.0, (last_move.mv.to.1 as i8 + direction) as u8);
                                    moves.push(Move::new(fmv, en_passant_move, MoveType::EnPassant, color));
                                }
                            }
                        }
                        moves.push(Move::new(fmv, capture_left, MoveType::Invalid, color));
                    }
                }
            }
            let capture_right = ((fmv.0 as i8 + 1) as u8, (fmv.1 as i8 + direction) as u8);
            if in_bounds(capture_right) {
                match self.board.get(capture_right.0, capture_right.1) {
                    Some(piece) => {
                        if piece.color != color && fmv.1 != seventh_file {
                            moves.push(Move::new(fmv, capture_right, MoveType::Capture, color));
                        }
                    },
                    None => {
                        // En passant: capturing an opponent's pawn in passing
                        if let Some(last_move) = self.game_state.move_history.last() {
                            if let MoveType::DoublePawnPush = last_move.mv.move_type {
                                let en_passant_moves = [(last_move.mv.to.0 + 1, last_move.mv.to.1), (last_move.mv.to.0 - 1, last_move.mv.to.1)];
                                if en_passant_moves.contains(&capture_right) {
                                    moves.push(Move::new(fmv, (last_move.mv.to.0, (last_move.mv.to.1 as i8+ direction)as u8), MoveType::EnPassant, color));
                                }
                            }
                        }
                        moves.push(Move::new(fmv, capture_right, MoveType::Invalid, color));
                    }
                }
            }
        }

        // Promotion: reaching the end of the board
        // generate promotion moves
        let promotion_moves = match color {
            Color::White => {
                [((fmv.0 as i8 + 1) as u8, 7), ((fmv.0 as i8 - 1) as u8, 7), (fmv.0, 7)]
            },
            Color::Black => {
                [((fmv.0 as i8 + 1) as u8, 0), ((fmv.0 as i8 - 1) as u8, 0), (fmv.0, 0)]
            }
        };

        for &pmv in &promotion_moves {
            // check if the position is valid
            if !in_bounds(pmv) {
                continue;
            }
            // check if the position is occupied
            if !is_tile_empty(self.board, pmv) {
                // check if the piece is an opponent's piece
                if self.board.get(pmv.0, pmv.1).unwrap().color != color {
                    // generate promotion moves
                    let promotion_attack_moves = self.promotion_attack_move(color, fmv, pmv);
                    // add promotion moves to the list of moves
                    moves.extend(promotion_attack_moves);
                }
            }
            else {
                // generate promotion moves
                let promotion_moves = self.promotion_move(color, fmv, pmv);
                moves.extend(promotion_moves);
            }
        }

        moves
    }


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
                // check if the position is attacked by an opponent's piece
                if is_attacked(self.game_state,move_to, color) {
                    continue;
                }
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
            if !is_in_check(self.game_state,color) {
                // 3. The rook with which the king is castling hasn't moved before
                let rook_positions = if color == Color::White { [(0, 7), (7, 7)] } else { [(0, 0), (7, 0)] };
                for &rook_pos in &rook_positions {
                    let rook = self.board.get(rook_pos.0, rook_pos.1);
                    if rook.is_some() && rook.unwrap().moves_count == 0 {
                        // 4. There are no pieces between the king and the rook
                        let min_x = min(pos.0, rook_pos.0) as usize;
                        let max_x = max(pos.0, rook_pos.0) as usize;
                        if (min_x..=max_x).all(|x| self.board.get(x as u8, pos.1).is_none() || (x as u8, pos.1) == pos || (x as u8, pos.1) == rook_pos) {
                            // 5. The king does not pass through a square that is attacked by an enemy piece
                            let castle_through = if rook_pos.0 == 0 { [(2, pos.1), (3, pos.1)] } else { [(5, pos.1), (6, pos.1)] };
                            if castle_through.iter().all(|&pos| !is_attacked(self.game_state, pos, color)) {
                                moves.push(Move::new(pos, if rook_pos.0 == 0 { (2, pos.1) } else { (6, pos.1) }, MoveType::Castle(if rook_pos.0 == 0 { CastleType::QueenSide } else { CastleType::KingSide }), color));
                            }
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
    pub fn promotion_move(&self, color: Color,fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Queen),color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Rook),color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Bishop),color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Knight),color));
        moves
    }

    // Pushes all promotion piece types attack moves to the list of moves
    pub fn promotion_attack_move(&self, color: Color,fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Queen),color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Rook),color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Bishop),color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Knight),color));
        moves
    }

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