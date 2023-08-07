use crate::board::piece::{Piece, PieceKind};
use crate::board::Position;
use crate::game::player::Color;
use std::fmt::Display;

// Enum to represent different types of castle moves
#[derive(Copy, Clone, PartialEq)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

// Enum to represent different types of moves
#[derive(Clone, PartialEq)]
pub enum MoveType {
    Normal,
    DoublePawnPush,
    Capture,
    Castle(CastleType),
    EnPassant,
    Promotion(PieceKind),
    PromotionCapture(PieceKind),
}

// Struct to represent a move
#[derive(Clone)]
pub struct Move {
    pub from_piece: Piece,
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
    pub color: Color,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_x1, _y1) = self.from;
        let (x2, y2) = self.to;
        let from_piece = self.from_piece;
        let move_type = self.move_type.clone();
        let mut mv = String::new();
        match move_type {
            MoveType::Normal => {
                mv.push_str(&format!("{} to ({}, {})", from_piece, x2, y2));
            }
            MoveType::DoublePawnPush => {
                mv.push_str(&format!("{} to ({}, {})", from_piece, x2, y2));
            }
            MoveType::Capture => {
                mv.push_str(&format!("{} x({}, {})", from_piece, x2, y2));
            }
            MoveType::Castle(castle_type) => {
                if castle_type == CastleType::KingSide {
                    mv.push_str("O-O");
                } else {
                    mv.push_str("O-O-O");
                }
            }
            MoveType::EnPassant => {
                mv.push_str(&format!("{} x({}, {})", from_piece, x2, y2));
            }
            MoveType::Promotion(promo_piece) => {
                mv.push_str(&format!(
                    "{} to ({}, {})={}",
                    from_piece, x2, y2, promo_piece
                ));
            }
            MoveType::PromotionCapture(promo_piece) => {
                mv.push_str(&format!("{} x({}, {})={}", from_piece, x2, y2, promo_piece));
            }
        }
        write!(f, "{}", mv)
    }
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

impl Move {
    pub fn new(from_piece: Piece, to: Position, move_type: MoveType, color: Color) -> Self {
        Self {
            from_piece,
            from: from_piece.position,
            to,
            move_type,
            color,
        }
    }

    pub fn is_capture(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Capture | MoveType::PromotionCapture(_)
        )
    }
}

#[cfg(test)]
mod tests {
    
    use crate::board::piece::{get_moves, Piece, PieceKind};
    use crate::board::{display_board, idx, in_bounds, Square};
    use crate::game::player::Color;
    use crate::game::{apply_move, Game, get_color_moves, get_current_moves};
    use crate::rules::r#move::Move;

    fn display_moves(game: &mut Game, moves: &[Move]) {
        for mv in moves.iter() {
            let mut gs = game.clone();
            gs = apply_move(gs, mv.from, mv.to);
            display_board(&gs.board);
            println!("Move: {:}", mv);
            println!("----------------------------------")
        }
    }

    fn recursive_mvgen_test(game: &Game, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut num_positions = 0usize;
        let moves = get_current_moves(game);
        let mut new_game = game.clone();

        for mv in moves {
            // apply the move
            new_game = apply_move(new_game.clone(), mv.from, mv.to);
            // switch color
            num_positions += recursive_mvgen_test(&new_game, depth - 1);
            // undo the move
            new_game = apply_move(new_game.clone(), mv.to, mv.from);
        }

        num_positions
    }
    fn queen_scenario(game: &mut Game, pos: (u8, u8), expected: usize, color: Color) {
        let queen = game.board.get(pos).unwrap();
        game.board.valid_moves = get_color_moves(game, color);
        let moves = get_moves(game, &queen);

        if moves.len() != expected {
            display_moves(game, &moves);
        }
        assert_eq!(moves.len(), expected);
    }

    fn game_with_piece_at(pos: (u8, u8), color: Color, kind: PieceKind) -> Game {
        let mut game = Game::new();
        game.game_state.turn = color.to_idx();
        game.board.squares[idx(pos)] = Some(Piece::new(kind, pos, color));
        let mut moves = get_color_moves(&game, color);
        game.board.valid_moves = moves;
        game
    }

    fn game_with_queen_at(pos: (u8, u8), color: Color) -> Game {
        game_with_piece_at(pos, color, PieceKind::Queen)
    }

    fn place_piece(sq: &mut [Square; 64], pos: (u8, u8), color: Color, kind: PieceKind) {
        sq[idx(pos)] = Some(Piece::new(kind, pos, color));
    }

    fn scattered_surround_by(
        gs: &mut Game,
        pos: (u8, u8),
        color: Color,
        kind: PieceKind,
        distance: i8,
    ) {
        let directions = [
            (-distance, -distance),
            (-distance, 0),
            (-distance, distance),
            (0, -distance),
            (0, distance),
            (distance, -distance),
            (distance, 0),
            (distance, distance),
        ];
        let positions = directions
            .iter()
            .map(|(x, y)| ((pos.0 as i8 + x) as u8, (pos.1 as i8 + y) as u8))
            .collect::<Vec<(u8, u8)>>();
        for pos in positions {
            if in_bounds(pos) {
                gs.board.squares[idx(pos)] =
                    Some(Piece::custom(kind, pos, color, true, None, None));
            }
        }
    }

    #[test]
    fn test_queen_moves() {
        let mut queen_pos = (3, 3);
        let color = Color::White;
        let mut game = game_with_queen_at(queen_pos, color);
        queen_scenario(&mut game, queen_pos, 27, color);
        println!("Passed queen test 1");

        // Now, add a white rook at (3, 5) and a black rook at (5, 3).
        game.board.squares[idx((3, 5))] = Some(Piece::new(PieceKind::Rook, (3, 5), color));
        game.board.squares[idx((5, 3))] = Some(Piece::new(PieceKind::Rook, (5, 3), color.other()));

        // The queen should now have 22 moves: 5 on the rank, 4 on the file, 7 on one diagonal, and 6 on the other diagonal.
        queen_scenario(&mut game, queen_pos, 22, color);
        println!("Passed queen test 2");

        // Test for edge cases
        // Place the queen at the edge of the board
        queen_pos = (7, 7);
        game = game_with_queen_at(queen_pos, color);

        // The queen should now have 21 moves: 7 on each rank, file, and diagonal.
        queen_scenario(&mut game, queen_pos, 21, color);
        println!("Passed queen test 3");

        // Place the queen at the corner of the board
        queen_pos = (0, 0);
        game = game_with_queen_at(queen_pos, color);

        // The queen should now have 21 moves: 7 on each rank, file, and diagonal.
        queen_scenario(&mut game, queen_pos, 21, color);
        println!("Passed queen test 4");

        // Test for moves blocked by friendly pieces
        // Place the queen at the center of the board
        queen_pos = (4, 4);
        game = game_with_queen_at(queen_pos, color);
        // Place a friendly pawns scattered around the queen's line of vision on the board
        scattered_surround_by(&mut game, queen_pos, color, PieceKind::Pawn, 2);

        // The queen should now have 8 moves: 2 on the rank, 2 on the file, 4 on diagonals.
        queen_scenario(&mut game, queen_pos, 8, color);
        println!("Passed queen test 5");

        // Test for moves blocked by enemy pieces
        game = game_with_queen_at(queen_pos, color);

        // Place an enemy pawns scattered around the queen's line of vision on the board
        scattered_surround_by(&mut game, queen_pos, color.other(), PieceKind::Pawn, 2);

        // The queen should now have 16 moves: 4 on the rank, 4 on the file, 4 on diagonals
        queen_scenario(&mut game, queen_pos, 16, color);
        println!("Passed queen test 6");

        // Test Queen pinned todo
        /*queen_pos = (6, 4);
        let mut game = game_with_queen_at(queen_pos, color);

        // Add an enemy rook at (7, 4) and the white king at (5, 4).
        game.board.squares[idx((7, 4))] = Some(Piece::new(PieceKind::Rook, (7, 4), color.other()));
        game.board.squares[idx((5, 4))] = Some(Piece::new(PieceKind::King, (5, 4), color));

        // The queen should now have 1 moves: (7, 4).
        queen_scenario(&mut game, queen_pos, 2, color);
        println!("Passed queen test 7");

        // Test Queen between king and enemy
        let mut game = game_with_queen_at(queen_pos, color);

        // Add an enemy rook at (7, 4) and the white king at (3, 4).
        game.board.squares[idx((7, 4))] = Some(Piece::new(PieceKind::Rook, (7, 4), color.other()));
        game.board.squares[idx((3, 4))] = Some(Piece::new(PieceKind::King, (3, 4), color));

        // The queen should now have 4 moves: to (4, 4), (5, 4), (6,4).
        queen_scenario(&mut game, queen_pos, 4, color);
        println!("Passed queen test 8");*/
    }

    #[test]
    fn test_move_generation_1() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 1);
        assert_eq!(num_positions, 20);
    }

    #[test]
    fn test_move_generation_2() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 2);
        assert_eq!(num_positions, 400);
    }

    #[test]
    fn test_move_generation_3() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 3);
        assert_eq!(num_positions, 8902);
    }

    #[test]
    fn test_move_generation_4() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 4);
        assert_eq!(num_positions, 197281); // actual: 197742 17s
    }

    #[test]
    fn test_move_generation_5() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 5);
        assert_eq!(num_positions, 4865609);
    }

    #[test]
    fn test_move_generation_6() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 6);
        assert_eq!(num_positions, 119060324);
    }

    #[test]
    fn test_move_generation_7() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 7);
        assert_eq!(num_positions, 3195901860);
    }

    #[test]
    fn test_move_generation_8() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 8);
        assert_eq!(num_positions, 84998978956);
    }

    #[test]
    fn test_move_generation_9() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 9);
        assert_eq!(num_positions, 2439530234167);
    }
}
