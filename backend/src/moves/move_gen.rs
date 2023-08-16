use crate::board::*;
use crate::moves::CastleSide::{KingSide, QueenSide};
use crate::moves::{SimpleMoves, DIRECTION_OFFSETS};
use crate::piece::*;

/// Generates all possible moves for a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
///
/// # Returns
/// A list of all possible moves for the given board.
///
/// # Example
/// ```rs
///    let moves = generate_moves(&board);
/// ```
pub fn generate_all_moves(board: &Board) -> SimpleMoves {
    let mut moves = SimpleMoves::new();

    for start_square in 0..64 {
        let piece = board.squares[start_square].piece;
        if piece.type_ == PieceKind::None {
            continue;
        }

        if is_color(piece, board.turn) {
            match piece.type_ {
                PieceKind::Pawn => {
                    let pawn_moves = generate_pawn_moves(board, start_square);
                    moves.extend(pawn_moves);
                }
                PieceKind::Knight => {
                    let knight_moves = generate_knight_moves(board, start_square);
                    moves.extend(knight_moves);
                }
                PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                    let sliding_moves = generate_sliding_moves(board, start_square);
                    moves.extend(sliding_moves);
                }
                PieceKind::King => {
                    let king_moves = generate_king_moves(board, start_square);
                    moves.extend(king_moves);
                }
                _ => (),
            }
        }
    }

    moves
}

/// Filters out all moves that are not legal.
/// A move is legal if it does not leave the king in check.
///
/// # Arguments
/// * `board` - The board to generate moves for.
///
/// # Returns
/// A list of all legal moves for the given board.
pub fn generate_legal_moves(board: &Board) -> SimpleMoves {
    // Generate all possible moves
    let psuedo_legal_moves = generate_all_moves(board);
    // Filter out all moves that leave the king in check
    let mut legal_moves = SimpleMoves::new();

    // For each move, make the move on a copy of the board and check if the king is in check
    for mv in psuedo_legal_moves {
        // Make a copy of the board
        let mut board_copy = board.clone();

        // Make the move temporarily
        board_copy.make_simple_move(mv);
        // Get the position of the king
        let king_pos = board_copy.get_king_position(board_copy.turn.other());
        //Get opponent response moves
        let response_moves = generate_all_moves(&board_copy);

        // If the opponent can take our king, it is not a valid move
        if response_moves.iter().any(|&mv| mv.1 == king_pos) {
            continue;
        }

        // Push the move
        legal_moves.push(mv)
    }

    legal_moves
}

/// Generates all possible moves for a Pawn on a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
/// * `start_square` - The square the piece is on.
///
/// # Returns
/// A list of all possible moves for the given piece.
pub fn generate_pawn_moves(board: &Board, from: usize) -> SimpleMoves {
    let mut moves = SimpleMoves::new();
    let pawn_color = board.squares[from].piece.color;

    // Determine the direction in which the pawn moves
    let direction = if pawn_color == Some(Color::White) {
        8
    } else {
        -8
    };

    // One Square Ahead: Check if the square in front of the pawn is empty
    let one_square_ahead = (from as i8 + direction) as usize;
    if one_square_ahead < 64 && board.squares[one_square_ahead].piece.type_ == PieceKind::None {
        moves.push((from, one_square_ahead));

        // Two Squares Ahead: Check if the pawn is on its starting square
        let two_squares_ahead = (from as i8 + 2 * direction) as usize;
        if two_squares_ahead < 64
            && board.squares[two_squares_ahead].piece.type_ == PieceKind::None
            && (from / 8
                == if pawn_color == Some(Color::White) {
                    1
                } else {
                    6
                })
        {
            moves.push((from, two_squares_ahead));
        }
    }

    // Capture: Check if the pawn can capture a piece
    let capture_directions = if pawn_color == Some(Color::White) {
        [7, 9]
    } else {
        [-7, -9]
    };
    for direction in capture_directions.iter() {
        let capture_square = (from as i8 + direction) as usize;
        if capture_square < 64
            && is_color(
                board.squares[capture_square].piece,
                pawn_color.expect("Pawn has no color").other(),
            )
        {
            moves.push((from, capture_square));
        }
    }

    // En Passant: Check if the last move was a two-square pawn advance
    if let Some(last_move) = board.move_history.last() {
        let last_from = last_move.0;
        let last_to = last_move.1;
        // Check if the move was a two-square pawn advance
        if board.squares[last_from].piece.type_ == PieceKind::Pawn
            && (last_to as i8 - last_from as i8).abs() == 16
        // 2 * 8 = 2 squares
        {
            // Determine the en passant target square
            let en_passant_square = (last_to as i8 + direction / 2) as usize;
            if en_passant_square < 64 && en_passant_square == one_square_ahead {
                moves.push((from, en_passant_square));
            }
        }
    }

    // Promotion: If the pawn reaches the opposite side of the board
    if one_square_ahead / 8
        == if pawn_color == Some(Color::White) {
            7
        } else {
            0
        }
    {
        //for promo_piece in &[PieceKind::Queen, PieceKind::Rook, PieceKind::Bishop, PieceKind::Knight]
        //{
        // Create a special move for the promotion, or handle it differently as needed
        moves.push((from, one_square_ahead));
        //}
    }

    moves
}

/// Generates all possible moves for a Knight on a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
/// * `start_square` - The square the piece is on.
///
/// # Returns
/// A list of all possible moves for the given piece.
pub fn generate_knight_moves(board: &Board, from: usize) -> SimpleMoves {
    // The list of moves to return.
    let mut moves = SimpleMoves::new();

    // The piece on the start square.
    let from_square = board.squares[from];
    let piece = from_square.piece;
    // The color of the piece.
    let color = piece.color;

    // The directions to check.
    let directions = [-17, -15, -10, -6, 6, 10, 15, 17];

    // For each direction, check if the move is valid.
    for direction in directions.iter() {
        // The square to check.
        let to = (from as i8 + direction) as usize;

        if !(0..=63).contains(&to) {
            continue;
        }

        let to_square = board.squares[to];

        // Knight moves end up on the opposite tile color as the start square.
        if to_square.tile_color == from_square.tile_color {
            continue;
        }
        // If the square is on the board and the piece on the square is not the same color as the piece, the move is valid.
        if !is_color(to_square.piece, color.expect("Knight has no color")) {
            moves.push((from, to));
        }
    }

    moves
}

/// Generates all possible moves for a sliding piece on a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
/// * `start_square` - The square the piece is on.
///
/// # Returns
/// A list of all possible moves for the given piece.
pub fn generate_sliding_moves(board: &Board, start_square: usize) -> SimpleMoves {
    // The list of moves to return.
    let mut moves = SimpleMoves::new();
    // The piece on the start square.
    let piece = board.squares[start_square].piece;
    let num_squares_to_edge = board.num_squares_to_edge[start_square];

    // The index of the first direction to check.
    let start_dir_idx = if piece.type_ == PieceKind::Bishop {
        4
    } else {
        0
    };
    // The index of the last direction to check.
    let end_dir_idx = if piece.type_ == PieceKind::Rook { 4 } else { 8 };

    // For each direction.
    for direction_idx in start_dir_idx..end_dir_idx {
        let direction = DIRECTION_OFFSETS[direction_idx];
        // For each square in the direction.
        for num_squares in 1..num_squares_to_edge[direction_idx] {
            // The end square offset.
            let end_square_offset = start_square as i8 + direction * num_squares as i8;
            if !(0..=63).contains(&end_square_offset) {
                break;
            }
            // The end square.
            let end_square = end_square_offset as usize;
            // The piece on the end square.
            let piece_on_end_square = board.squares[end_square].piece;

            // Blocked by a piece of the same color.
            if is_color(piece_on_end_square, board.turn) {
                break;
            }
            // Add the move.
            moves.push((start_square, end_square));

            // Blocked by a piece of the opposite color.
            if is_color(piece_on_end_square, board.turn.other()) {
                break;
            }
        }
    }

    moves
}

/// Generates all possible moves for a king on a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
/// * `start_square` - The square the piece is on.
///
/// # Returns
/// A list of all possible moves for the given piece.
pub fn generate_king_moves(board: &Board, from: usize) -> SimpleMoves {
    // The list of moves to return.
    let mut moves = SimpleMoves::new();

    // For each direction.
    for direction in DIRECTION_OFFSETS.iter() {
        // The end square.
        let mut end_square_pos = from as i8 + direction;
        if !(0..=63).contains(&end_square_pos) {
            continue;
        }
        let end_square_pos = end_square_pos as usize;
        let end_square = board.squares[end_square_pos];
        // The piece on the end square.
        let piece_on_end_square = end_square.piece;

        // Blocked by a piece of the same color.
        if is_color(piece_on_end_square, board.turn) {
            continue;
        }
        if end_square.is_attacked {
            continue;
        }
        // Add the move.
        moves.push((from, end_square_pos));
    }

    // Check for Kingside castling
    if board.can_castle(board.turn, KingSide) {
        moves.push((from, from + 2));
    }
    // Check for Queenside castling
    if board.can_castle(board.turn, QueenSide) {
        moves.push((from, from - 2));
    }

    moves
}

#[cfg(test)]
mod tests {
    use crate::board::square::*;

    use crate::game::*;
    use crate::moves::move_gen::*;

    /// Recursively generates all possible moves for a given board.
    ///
    /// # Arguments
    /// * `board` - The board to generate moves for.
    /// * `depth` - The depth to generate moves to.
    /// * `expected` - The expected number of moves.
    ///
    /// # Returns
    /// The number of moves generated.
    pub fn recursive_move_gen_test(board: &Board, depth: u8, expected: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut num_moves = 0;

        for from in 0..64 {
            if board.squares[from].piece.type_ == PieceKind::None
                || board.squares[from].piece.color != Some(board.turn)
            {
                continue;
            }

            let piece = board.squares[from].piece;
            let moves = match piece.type_ {
                PieceKind::Pawn => generate_pawn_moves(board, from),
                PieceKind::Knight => generate_knight_moves(board, from),
                PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                    generate_sliding_moves(board, from)
                }
                PieceKind::King => generate_king_moves(board, from),
                _ => SimpleMoves::new(),
            };

            let mut new_board = board.clone();
            for mv in moves {
                new_board.make_simple_move(mv);
                num_moves += recursive_move_gen_test(&new_board, depth - 1, expected);
                new_board.unmake_simple_move()
            }
        }

        if num_moves != expected {
            displays_moves(board);
        }

        num_moves
    }

    /// Places a piece on a given board.
    ///
    /// # Arguments
    /// * `board` - The board to place the piece on.
    /// * `piece` - The piece to place.
    /// * `pos` - The position to place the piece at.
    pub fn place_piece(board: &mut Board, piece: Piece, pos: Position) {
        board.squares[pos].piece = piece;
    }

    /// Surrounds a square with a given piece.
    ///
    /// # Arguments
    /// * `board` - The board to place the piece on.
    /// * `pos` - The position to surround.
    /// * `piece` - The piece to surround the square with.
    /// * `radius` - How far from the position to place the pieces.
    /// * `scattered` - Whether to place the pieces in a scattered pattern. (A space between each piece)
    pub fn surround_square(board: &mut Board, pos: Position, piece: Piece, radius: usize) {
        // The positions to place the piece at.
        let mut positions = Vec::new();

        // Get the positions of the scattered pieces.
        for direction in DIRECTION_OFFSETS.iter() {
            // The position to place the piece at.
            let position = pos as i8 + direction * radius as i8;
            // If the position is on the board.
            if (0..=63).contains(&position) {
                positions.push(position as usize);
            }
        }

        // For each position.
        for pos in positions {
            place_piece(board, piece, pos);
        }
    }

    /// Creates a game with a piece at a given position.
    ///
    /// # Arguments
    /// * `pos` - The position to place the piece at.
    /// * `kind` - The kind of piece to place.
    /// * `color` - The color of the piece to place.
    ///
    /// # Returns
    /// The game with the piece at the given position.
    pub fn game_with_piece_at(pos: Position, kind: PieceKind, color: Color) -> Game {
        let mut game = Game::new();
        let piece = Piece::new(kind as u8 | color as u8);
        place_piece(&mut game.board, piece, pos);
        game
    }

    /// Displays all possible moves for a given board.
    ///
    /// # Arguments
    /// * `board` - The board to display the moves for.
    pub fn displays_moves(board: &Board) {
        let moves = generate_legal_moves(board);
        for mv in moves {
            let mut b = board.clone();
            b.make_simple_move(mv);
            let sq = b.squares[mv.1];
            let piece = sq.piece;
            println!("{} from {} to {}", piece, mv.0, mv.1);
            b.unmake_simple_move();
        }
    }

    pub fn get_piece_moves(board: &Board, from: Position, piece: Piece) -> SimpleMoves {
        let mut moves: SimpleMoves = SimpleMoves::new();

        match piece.type_ {
            PieceKind::Pawn => moves = generate_pawn_moves(board, from),
            PieceKind::Knight => moves = generate_knight_moves(board, from),
            PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                moves = generate_sliding_moves(board, from)
            }
            PieceKind::King => moves = generate_king_moves(board, from),
            _ => {}
        }

        moves
    }

    #[test]
    fn move_gen_test() {
        use crate::board::Board;

        let board = new_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        let num_moves_1 = recursive_move_gen_test(&board, 1, 20);
        assert_eq!(num_moves_1, 20);

        let num_moves_2 = recursive_move_gen_test(&board, 2, 400);
        assert_eq!(num_moves_2, 400);

        let num_moves_3 = recursive_move_gen_test(&board, 3, 8902);
        assert_eq!(num_moves_3, 8902); // 8009
    }

    #[test]
    /// A comprehensive test of the move generation for all pieces.
    ///
    /// 1. Place a piece on a square.
    /// 2. Surround the piece with pieces of the same color.
    /// 3. Generate all possible moves for that piece.
    /// 4. Compare the number of moves generated to the expected number of moves.
    /// 5. Scatter the pieces around the piece and make them hostile.
    /// 6. Generate all possible moves for that piece.
    /// 7. Compare the number of moves generated to the expected number of moves.
    /// 8. Repeat for the next square.
    /// 9. Repeat for the next piece.
    fn piece_movement_test() {
        let colors = [Color::White, Color::Black];
        let kinds = [
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
            PieceKind::King,
        ];
        let board_positions = 00..63usize;
        let radius = 2;

        for color in colors.iter() {
            for kind in kinds.iter() {
                for pos in board_positions.clone() {
                    let game = game_with_piece_at(pos, *kind, *color);
                    let mut board = game.board.clone();
                    let piece = board.squares[pos].piece;
                    println!("{}\nTesting {} at {}", board, piece, pos);
                    surround_square(&mut board, pos, piece, radius);
                    let moves = get_piece_moves(&board, pos, piece);
                    let num_moves = moves.len();
                    print!("{} moves: {}", piece, num_moves);

                    surround_square(&mut board, pos, piece, radius);
                    for sq in board.squares.iter_mut() {
                        if sq.piece.color != Some(*color) {
                            sq.piece.color = Some(*color);
                        }
                    }
                    let moves = generate_legal_moves(&board);
                    let num_moves = moves.len();
                    println!(" | {} moves: {}\n{}", piece, num_moves, board);
                }
            }
        }
    }
}
