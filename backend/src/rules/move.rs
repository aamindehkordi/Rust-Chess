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
    /**
     * Checks if the move type is a normal move.
     *
     * This function checks if the current move type is a normal move. It returns true if the move type is MoveType::Normal, false otherwise.
     *
     * @return True if the move type is normal, false otherwise.
     */
    pub fn is_normal(&self) -> bool {
        matches!(self, MoveType::Normal)
    }

    // Function to check if a move type is a promotion
    /**
     * Checks if a move is a promotion.
     *
     * This function determines whether the move type is a promotion or not.
     * It returns true if the move type is MoveType::Promotion, false otherwise.
     *
     * @return - True if the move is a promotion, false otherwise.
     */
    pub fn is_promotion(&self) -> bool {
        matches!(self, MoveType::Promotion(_))
    }

    // Function to check if a move type is a capture
    /**
     * Checks if the move type is a promotion capture.
     *
     * This function checks if the move type is a promotion capture in the current Chessboard
     * state. It returns true if the move type is MoveType::PromotionCapture, otherwise it returns false.
     *
     * @return true if the move is a promotion capture, false otherwise.
     */
    pub fn is_promo_capture(&self) -> bool {
        matches!(self, MoveType::PromotionCapture(_))
    }
}

impl Move {
    /**
     * Creates a new Move struct.
     *
     * This function initializes and returns a new Move struct with the provided parameters.
     *
     * @param from_piece - The piece being moved.
     * @param to - The target position of the move.
     * @param move_type - The type of move.
     * @param color - The color of the player making the move.
     * @returns A new Move struct with the specified parameters.
     */
    pub fn new(from_piece: Piece, to: Position, move_type: MoveType, color: Color) -> Self {
        Self {
            from_piece,
            from: from_piece.position,
            to,
            move_type,
            color,
        }
    }

    /**
     * Checks if the move is a capture.
     *
     * This function determines whether the move is a capture by checking the move type in the Chessboard struct.
     * Returns true if the move is a capture, and false otherwise.
     *
     * @return true if the move is a capture, false otherwise.
     */
    pub fn is_capture(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Capture | MoveType::PromotionCapture(_) | MoveType::EnPassant
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::{get_moves, Piece, PieceKind};
    use crate::board::{display_board, idx, in_bounds, Square};
    use crate::game::player::Color;
    use crate::game::{apply_move, get_color_moves, get_current_moves, Game};
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

    /**
     * Performs a recursive move generation test.
     *
     * This function recursively generates and tests moves for the specified game up to the given depth.
     * It counts the number of positions evaluated during the test.
     *
     * @param game - The game for which moves are to be generated and tested.
     * @param depth - The maximum depth of recursion for move generation and testing.
     *
     * @return The number of positions evaluated during the test.
     */
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

    /**
     * Executes the queen scenario in a chess game.
     *
     * This function calculates the valid moves for the specified queen on the game board and compares it with the expected number of moves.
     * If the calculated moves do not match the expected number, it displays the moves for debugging purposes.
     *
     * @param game - A mutable reference to the Game struct representing the chess game.
     * @param pos - The position of the queen on the game board specified as a tuple of (row, column).
     * @param expected - The expected number of valid moves for the queen.
     * @param color - The color of the player owning the queen.
     */
    fn queen_scenario(game: &mut Game, pos: (u8, u8), expected: usize, color: Color) {
        let queen = game.board.get(pos).unwrap();
        game.board.board_info.valid_moves = get_color_moves(&game.board, color);
        let moves = get_moves(&game.board.board_info, &queen);

        if moves.len() != expected {
            display_moves(game, &moves);
        }
        assert_eq!(moves.len(), expected);
    }

    /**
     * Creates a new game with a piece placed at the specified position.
     *
     * This function creates a new game instance and sets the turn to the specified player color. It
     * then places a piece of the specified kind and color at the given position on the game board.
     * After placing the piece, it calculates the valid moves for the specified player color and sets
     * them in the game board info.
     *
     * @param pos - The position on the board where the piece will be placed.
     * @param color - The color of the piece to be placed.
     * @param kind - The kind of the piece to be placed.
     * @returns A new game instance with the specified piece placed on the board.
     */
    fn game_with_piece_at(pos: (u8, u8), color: Color, kind: PieceKind) -> Game {
        let mut game = Game::new();
        game.game_state.turn = color.to_idx();
        game.board.squares[idx(pos)] = Some(Piece::new(kind, pos, color));
        let moves = get_color_moves(&game.board, color);
        game.board.board_info.valid_moves = moves;
        game
    }

    /**
     * Create a new game with a queen at the specified position and color.
     *
     * This function creates a new game by invoking the `game_with_piece_at` function with the specified
     * position, color, and PieceKind::Queen.
     *
     * @param pos - The position where the queen will be placed on the game board, represented as a tuple
     *              of two u8 values (row, column).
     * @param color - The color of the queen to be placed.
     * @return A new Game struct with a queen at the specified position and color.
     */
    fn game_with_queen_at(pos: (u8, u8), color: Color) -> Game {
        game_with_piece_at(pos, color, PieceKind::Queen)
    }

    /**
     * Places a piece on the specified square.
     *
     * This function adds a new piece of the specified kind and color to the given square array at the
     * specified position.
     *
     * @param sq - The array of squares to place the piece on.
     * @param pos - The position of the square where the piece is to be placed.
     * @param color - The color of the piece.
     * @param kind - The kind of the piece.
     */
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
    /**
     * Tests the movement of the queen.
     *
     * This function tests various scenarios to ensure that the queen's movement is correctly implemented.
     * It verifies the number of available moves for the queen in different positions on the board.
     */
    // TODO: Implement tests for Queen pinned and Queen between king and enemy scenarios.
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
    /**
     * Performs a test for move generation on a Chess game.
     *
     * This function creates a new standard Chess game and invokes the recursive_mvgen_test function to generate moves
     * with a maximum depth of 1. It then asserts that the number of generated positions is equal to 20.
     */
    fn test_move_generation_1() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 1);
        assert_eq!(num_positions, 20);
    }

    #[test]
    /**
     * Performs a move generation test with a depth of 2.
     *
     * This function initializes a new Game instance and calls the recursive_mvgen_test function
     * to generate all possible moves up to a depth of 2. It then asserts that the total number
     * of positions generated is equal to 400.
     *
     */
    fn test_move_generation_2() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 2);
        assert_eq!(num_positions, 400);
    }

    #[test]
    /**
     * Performs a test of move generation with a specified depth limit.
     *
     * This function creates a new standard Chess game and calls the recursive_mvgen_test function
     * to generate all possible moves up to the given depth limit. It then asserts that the number
     * of generated positions matches the expected value.
     *
     * @see recursive_mvgen_test
     */
    fn test_move_generation_3() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 3);
        assert_eq!(num_positions, 8902); // 8000 62ms
    }

    #[test]
    /**
     * Performs a test for move generation with a depth of 4.
     *
     * This function creates a new standard game and then calls the recursive_mvgen_test function
     * to generate moves for the specified depth. It then compares the number of positions generated
     * with the expected value and asserts their equality.
     *
     * Note: This function assumes the recursive_mvgen_test function is implemented.
     *
     */
    fn test_move_generation_4() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 4);
        assert_eq!(num_positions, 197281); // actual: 160000 989ms
    }

    #[test]
    /**
     * Test the move generation algorithm for a specific depth.
     *
     * This function generates moves for the current state of the game using a recursive move generation algorithm.
     * It then counts the total number of positions generated and compares it against the expected value.
     *
     * @Test
     * @precondition - The game object has been initialized with a standard chess setup.
     * @expected_result - The total number of generated positions matches the expected value.
     */
    fn test_move_generation_5() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 5);
        assert_eq!(num_positions, 4865609); // actual: 3200000 14 sec
    }

    #[test]
    /**
     * Test function to generate and count all possible positions up to the given depth.
     *
     * This function generates and counts all possible positions up to the specified depth using a recursive move generation strategy. It initializes a new standard game and calls the recursive_mvgen_test function.
     * The number of generated positions is then compared with the expected number.
     */
    fn test_move_generation_6() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 6);
        assert_eq!(num_positions, 119060324);
    }

    #[test]
    /**
     * Test case for move generation using a maximum recursion depth of 7.
     *
     * This function creates a new standard game, calls the recursive_mvgen_test function
     * with a maximum recursion depth of 7, and verifies that the number of generated positions
     * matches the expected value.
     */
    fn test_move_generation_7() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 7);
        assert_eq!(num_positions, 3195901860);
    }

    #[test]
    /**
     * Test move generation for a specific depth.
     *
     * This function tests the move generation functionality for a specific depth in the game.
     * It creates a new standard game, performs move generation recursively up to the specified depth,
     * and then compares the number of positions generated with the expected value.
     *
     * @param depth - The depth to test move generation for.
     */
    fn test_move_generation_8() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 8);
        assert_eq!(num_positions, 84998978956);
    }

    #[test]
    /**
     * Test move generation for a specific depth.
     *
     * This function tests the move generation functionality for a specific depth in the game. It initializes a new standard
     * chess game and performs a recursive move generation test with the specified depth. The number of positions generated
     * is then compared with the expected value.
     *
     * @note The expected number of positions for depth 9 is 2439530234167.
     */
    fn test_move_generation_9() {
        let mut game = Game::new_standard();
        let num_positions = recursive_mvgen_test(&mut game, 9);
        assert_eq!(num_positions, 2439530234167);
    }
}
