use std::cmp::{max, min};
// Import necessary modules and dependencies
use crate::board::{Board, Color, in_bounds, make_move, Piece, PieceKind};
use crate::game::{GameState, get_current_player, is_attacked, is_in_check};
use crate::player::Player;

#[derive(Debug)]
pub enum MoveError {
    MoveIsNotValid,
    MoveDoesNotBlockCheck,
    Other(String),
}

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
}

impl MoveType {
    pub fn is_normal(&self) -> bool {
        matches!(self, MoveType::Normal)
    }

    // Function to check if a move type is a promotion
    pub fn is_promotion(&self) -> bool {
        matches!(self, MoveType::Promotion(_))
    }

    // Function to check if a move type is a capture
    pub fn is_promo_capture(&self) -> bool {
        matches!(self, MoveType::PromotionCapture(_))
    }
}

// Struct to represent a move
#[derive(Copy, Clone)]
pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub move_type: MoveType,
    pub piece: Piece,
    pub color: Color,
}

impl Move {
    // Function to create a new move
    pub fn new(from: (u8, u8), to: (u8, u8), move_type: MoveType, piece: Piece, color: Color) -> Self {
        Self {
            from,
            to,
            move_type,
            piece,
            color
        }
    }

    pub fn is_capture(&self) -> bool {
        matches!(self.move_type, MoveType::Capture | MoveType::PromotionCapture(_))
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
                PieceKind::Rook => self.moves.extend(self.generate_sliding_move(x, y, color)),
                PieceKind::Knight => self.moves.extend(self.generate_knight_moves(x, y, color)),
                PieceKind::Bishop => self.moves.extend(self.generate_sliding_move(x, y, color)),
                PieceKind::Queen => self.moves.extend(self.generate_sliding_move(x, y, color)),
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

    pub fn generate_pawn_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new(); // Vector to store moves
        let piece = self.board.get(x, y).unwrap(); // Get the piece at the given position
        let direction = match color { // Get the direction of the pawn
            Color::White => 1,
            Color::Black => -1,
        };

        // Normal move forward
        let forward_square = (x, (y as i8 + direction) as u8); // Get the square in front of the pawn
        if in_bounds(forward_square.0, forward_square.1) && self.board.get(forward_square.0, forward_square.1).is_none() { // Check if the square is in bounds and empty
            let mvs = if forward_square.1 == 0 || forward_square.1 == 7 {
                self.promotion_move(color, (x, y), forward_square)
            } else {
                vec![Move::new((x, y), forward_square, MoveType::Normal, piece, color)]
            };
            moves.extend(mvs);
        }


        // Double move forward
        let single_forward_square = (x, (y as i8 + direction) as u8);
        let double_forward_square = (x, (y as i8 + 2 * direction) as u8);
        if (y == 1 || y == 6) && in_bounds(double_forward_square.0, double_forward_square.1) && self.board.get(single_forward_square.0, single_forward_square.1).is_none() && self.board.get(double_forward_square.0, double_forward_square.1).is_none() {
            moves.push(Move::new((x, y), double_forward_square, MoveType::DoublePawnPush, piece, color));
        }

        // Captures
        for &dx in [-1, 1].iter() {
            let capture_square = ((x as i8 + dx) as u8, (y as i8 + direction) as u8);
            if in_bounds(capture_square.0, capture_square.1) {
                match self.board.get(capture_square.0, capture_square.1) {
                    Some(piece) if piece.color != color => {
                        let mvs = if capture_square.1 == 0 || capture_square.1 == 7 {
                            self.promotion_attack_move(color, capture_square, capture_square)
                        } else {
                            vec![Move::new((x, y), capture_square, MoveType::Capture, piece, color)]

                        };
                        moves.extend(mvs);
                    },
                    _ => (),
                }
            }
        }

        // En Passant
        if let Some(last_move) = self.game_state.move_history.last() {
            if let MoveType::DoublePawnPush = last_move.mv.move_type {
                if last_move.mv.to.1 == y && (last_move.mv.to.0 as i8 - x as i8).abs() == 1 {
                    let en_passant_move = (last_move.mv.to.0, (last_move.mv.to.1 as i8 + direction) as u8);
                    moves.push(Move::new((x, y), en_passant_move, MoveType::EnPassant, piece, color));
                }
            }
        }

        moves
    }

    pub fn generate_king_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = (x, y);
        let piece = self.board.get(x, y).unwrap();
        let from_pos = (x, y);

        // The king can move in 8 directions: up, down, left, right, and the 4 diagonals.
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for &(dx, dy) in directions.iter() {
            let to_pos = ((x as i8 + dx) as u8, (y as i8 + dy) as u8);
            self.normal_capture(color, from_pos, to_pos, &mut moves);
        }


        // Castle move generation
        // 1. The king hasn't moved before
        if piece.moves_count == 0 {
            // 2. The king is not currently in check
            if !is_in_check(self.game_state, color) {
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
                                // 6. The king is not in check after the castle move
                                let castle_to = if rook_pos.0 == 0 { (2, pos.1) } else { (6, pos.1) };
                                let mut temp_gs = self.game_state.clone();
                                temp_gs.board = make_move(temp_gs.board, &Move::new(pos, castle_to, MoveType::Castle(if rook_pos.0 == 0 { CastleType::QueenSide } else { CastleType::KingSide }), piece, color));
                                if !is_in_check(&temp_gs, color) {
                                    moves.push(Move::new(pos, castle_to, MoveType::Castle(if rook_pos.0 == 0 { CastleType::QueenSide } else { CastleType::KingSide }), piece, color));
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    // Function to generate all legal moves for a knight at a given position
    pub fn generate_knight_moves(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let from_pos = (x, y);
        let piece = self.board.get(x, y).unwrap();

        // The knight can move in 8 directions: up up left/right, down down left/right, left left up/down, right right up/down
        let directions = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

        for direction in &directions {
            let to_pos = ((x as i8 + direction.0) as u8, (y as i8 + direction.1) as u8);
            self.normal_capture(color, from_pos, to_pos, &mut moves);
        }
        moves
    }

    // Pushes all promotion piece types moves to the list of moves
    pub fn promotion_move(&self, color: Color,fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
        let mut moves= vec![];
        let piece = self.board.get(fmv.0, fmv.1).unwrap();
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Queen), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Rook), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Bishop), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::Promotion(PieceKind::Knight), piece,color));
        moves
    }

    // Pushes all promotion piece types attack moves to the list of moves
    pub fn promotion_attack_move(&self, color: Color,fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
        let mut moves= vec![];
        let piece = self.board.get(fmv.0, fmv.1).unwrap();
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Queen), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Rook), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Bishop), piece,color));
        moves.push(Move::new(fmv, pmv, MoveType::PromotionCapture(PieceKind::Knight), piece,color));
        moves
    }

    pub fn normal_capture(&self, color: Color, from_pos: (u8, u8), to_pos: (u8, u8), moves: &mut Vec<Move>) -> bool{
        let piece = self.board.get(from_pos.0, from_pos.1).unwrap();
        if in_bounds(to_pos.0, to_pos.1) {
            match self.board.get(to_pos.0, to_pos.1) {
                Some(to_piece) if to_piece.color != color => {
                    moves.push(Move::new(from_pos, to_pos, MoveType::Capture, to_piece, color));
                    return true;
                },
                None => {
                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal, piece, color));
                    return false;
                }
                _ => (),
            }
        }
        true
    }

    // Function to generate all legal moves for a sliding piece at a given position
    pub fn generate_sliding_move(&self, x: u8, y: u8, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let from_pos = (x, y);
        let piece = self.board.get(from_pos.0, from_pos.1).unwrap();
        let directions: Vec<(i8,i8)>;
        // horizontal and vertical directions
        let hdirections = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        let ddirections = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        // diagonal directions
        if piece.kind == PieceKind::Queen {
            directions = hdirections.iter().chain(ddirections.iter()).cloned().collect();
        } else if piece.kind == PieceKind::Rook {
            directions = hdirections.to_vec();
        } else { // Bishop
            directions = ddirections.to_vec();
        }

        for direction in directions.iter() {
            let mut distance = 1;
            loop {
                let new_x = (from_pos.0 as i8 + distance * direction.0) as u8;
                let new_y = (from_pos.1 as i8 + distance * direction.1) as u8;
                let to_pos = (new_x, new_y);

                if !in_bounds(new_x, new_y) {
                    break;
                }

                match self.board.get(to_pos.0, to_pos.1) {
                    Some(to_piece) => {
                        if to_piece.color != color {
                            moves.push(Move::new(from_pos, to_pos, MoveType::Capture, to_piece, color));
                        }
                        break;
                    },
                    None => {
                        moves.push(Move::new(from_pos, to_pos, MoveType::Normal, piece, color));
                    }
                }
                else if self.normal_capture(color, from_pos, to_pos, &mut moves) { break; }
                distance += 1;
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{apply_move, undo_move};
    use super::*;
    fn queen_gs(pos: (u8, u8), color: Color) -> GameState {
        let mut game_state = GameState::new();
        // Place the queen in the middle of an otherwise empty board
        game_state.board.set(pos.0, pos.1, Some(Piece::new(color,PieceKind::Queen)));
        game_state
    }

    fn queen_scenario(gs: &mut GameState, pos: (u8, u8), expected: usize, color: Color) {
        let mut move_generator = MoveGenerator::new(gs);
        let mut moves = move_generator.generate_moves(color);
        moves.retain(|m| m.from == pos);

    fn recursive_mvgen_test(game_state: &mut GameState, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        if moves.len() != expected {
            display_moves(gs, &moves);
        }
    }

    fn place_piece(gs: &mut GameState, pos: (u8, u8), color: Color, kind: PieceKind) {
        gs.board.set(pos.0, pos.1, Some(Piece::new(color, kind)));
    }

    fn scattered_surround_by(gs: &mut GameState, pos: (u8, u8), color: Color, kind: PieceKind, distance: i8) {
        let directions = [(-distance, -distance), (-distance, 0), (-distance, distance), (0, -distance), (0, distance), (distance, -distance), (distance, 0), (distance, distance)];
        let positions = directions.iter().map(|(x, y)| (pos.0 as i8 + x, pos.1 as i8 + y)).collect::<Vec<(i8, i8)>>();
        for (x, y) in positions {
            if in_bounds(x as u8, y as u8) {
                place_piece(gs, (x as u8, y as u8), color, kind);
            }
        }
    }

    #[test]
    fn test_queen_moves() {
        let mut queen_pos = (3, 3);
        let color = Color::White;
        let mut game_state = queen_gs(queen_pos, color);
        queen_scenario(&mut game_state, queen_pos, 27, color);
        // Now, add a white rook at (3, 5) and a black rook at (5, 3).
        place_piece(&mut game_state, (3, 5), color, PieceKind::Rook);
        place_piece(&mut game_state, (5, 3), color.other(), PieceKind::Rook);

        // The queen should now have 22 moves: 5 on the rank, 4 on the file, 7 on one diagonal, and 6 on the other diagonal.
        queen_scenario(&mut game_state, queen_pos, 22, color);

        // Test for edge cases
        // Place the queen at the edge of the board
        queen_pos = (7, 7);
        game_state = queen_gs(queen_pos, color);

        // The queen should now have 21 moves: 7 on each rank, file, and diagonal.
        queen_scenario(&mut game_state, queen_pos, 21, color);

        // Place the queen at the corner of the board
        queen_pos = (0, 0);
        game_state = queen_gs(queen_pos, color);

        // The queen should now have 21 moves: 7 on each rank, file, and diagonal.
        queen_scenario(&mut game_state, queen_pos, 21, color);

        // Test for moves blocked by friendly pieces
        // Place the queen at the center of the board
        queen_pos = (4, 4);
        game_state = queen_gs(queen_pos, color);
        // Place a friendly pawns scattered around the queen's line of vision on the board
        scattered_surround_by(&mut game_state, queen_pos, color, PieceKind::Pawn, 2);

        // The queen should now have 8 moves: 2 on the rank, 2 on the file, 4 on diagonals.
        queen_scenario(&mut game_state, queen_pos, 8, color);

        // Test for moves blocked by enemy pieces
        game_state = queen_gs(queen_pos, color);

        // Place an enemy pawns scattered around the queen's line of vision on the board
        scattered_surround_by(&mut game_state, queen_pos, color.other(), PieceKind::Pawn, 2);

        // The queen should now have 16 moves: 4 on the rank, 4 on the file, 4 on diagonals
        queen_scenario(&mut game_state, queen_pos, 16, color);

        // Test Queen pinned
        queen_pos = (6, 4);
        let mut game_state = queen_gs(queen_pos, color);

        // Add an enemy rook at (7, 4) and the white king at (5, 4).
        place_piece(&mut game_state, (7, 4), color.other(), PieceKind::Rook);
        place_piece(&mut game_state, (5, 4), color, PieceKind::King);

        game_state.check_state = calculate_check_state(&game_state);
        // The queen should now have 1 moves: (7, 4).
        queen_scenario(&mut game_state, queen_pos, 2, color);

        // Test Queen between king and enemy
        let mut game_state = queen_gs(queen_pos, color);

        // Add an enemy rook at (7, 4) and the white king at (3, 4).
        place_piece(&mut game_state, (7, 4), color.other(), PieceKind::Rook);
        place_piece(&mut game_state, (3, 4), color, PieceKind::King);

        game_state.check_state = calculate_check_state(&game_state);
        // The queen should now have 4 moves: to (4, 4), (5, 4), (6,4).
        queen_scenario(&mut game_state, queen_pos, 4, color);
    }

    #[test]
    fn test_move_generation_1() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 1);
        assert_eq!(num_positions, 20);
    }

    #[test]
    fn test_move_generation_2() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 2);
        assert_eq!(num_positions, 400);
    }

    #[test]
    fn test_move_generation_3() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 3);
        assert_eq!(num_positions, 8902);
    }

    #[test]
    fn test_move_generation_4() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 4);
        assert_eq!(num_positions, 197281); // actual: 197742 17s
    }

    #[test]
    fn test_move_generation_5() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 5);
        assert_eq!(num_positions, 4865609);
    }

    #[test]
    fn test_move_generation_6() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 6);
        assert_eq!(num_positions, 119060324);
    }

    #[test]
    fn test_move_generation_7() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 7);
        assert_eq!(num_positions, 3195901860);
    }

    #[test]
    fn test_move_generation_8() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 8);
        assert_eq!(num_positions, 84998978956);
    }

    #[test]
    fn test_move_generation_9() {
        let mut game_state = GameState::new_standard();
        let num_positions = recursive_mvgen_test(&mut game_state, 9);
        assert_eq!(num_positions, 2439530234167);
    }

    fn display_moves(game_state: &GameState, moves: &[Move]) {
        for mv in moves.iter() {
            let mut gs = game_state.clone();
            gs = apply_move(gs, mv);
            display_game_state(&gs);
        }
    }

    fn recursive_mvgen_test(game_state: &GameState, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut num_positions = 0usize;
        let mut move_generator = MoveGenerator::new(game_state);
        let moves = move_generator.generate_current_moves();
        let mut new_game_state = game_state.clone();

        for mv in moves {
            // apply the move
            new_game_state = apply_move(new_game_state.clone(), &mv);
            new_game_state.check_state = calculate_check_state(&new_game_state);
            calculate_all_moves(&mut new_game_state);
            // switch color
            num_positions += recursive_mvgen_test(&new_game_state, depth - 1);
            // undo the move
            new_game_state = undo_move(new_game_state);
        }

        num_positions
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

