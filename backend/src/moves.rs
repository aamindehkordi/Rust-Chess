use crate::board::CastleSide::{KingSide, QueenSide};
use crate::board::*;
use crate::piece::*;

/// A list of offsets for each direction.
/// The directions are in the following order:
/// North West, North, North East, West, East, South West, South, South East.
pub const DIRECTION_OFFSETS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];

/// A list of moves.
/// Each move is a pair of positions.
pub type SimpleMoves = Vec<SimpleMove>;

/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub type SimpleMove = (Position, Position);

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

        // Get the position of the king
        let king_pos = board_copy.get_king_position(board_copy.turn);
        // Make the move temporarily
        board_copy.make_simple_move(mv);
        //Get opponent response moves
        let response_moves = generate_all_moves(&board_copy);

        // If the opponent can take our king, it is not a valid move
        if response_moves.iter().any(|&mv| mv.1 == king_pos) {
            continue;
        }

        // Undo the move
        board_copy.unmake_simple_move();

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
    let piece = board.squares[from].piece;
    // The color of the piece.
    let color = piece.color;

    // The directions to check.
    let directions = [-17, -15, -6, 6, 15, 17];

    // For each direction, check if the move is valid.
    for direction in directions.iter() {
        // The square to check.
        let to = (from as i8 + direction) as usize;
        // If the square is on the board and the piece on the square is not the same color as the piece, the move is valid.
        if to < 64 && !is_color(board.squares[to].piece, color.expect("Knight has no color")) {
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
        // For each square in the direction.
        for num_squares in 1..board.num_squares_to_edge[start_square][direction_idx] {
            let direction = DIRECTION_OFFSETS[direction_idx];
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
        let end_square_pos = from as i8 + direction;
        if !(0..=63).contains(&end_square_pos) {
            continue;
        }
        let end_square = end_square_pos as usize;
        // The piece on the end square.
        let piece_on_end_square = board.squares[end_square].piece;

        // Blocked by a piece of the same color.
        if is_color(piece_on_end_square, board.turn) {
            continue;
        }
        // Add the move.
        moves.push((from, end_square));
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

        for mv in moves {
            let mut new_board = board.clone();
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
#[cfg(test)]
mod tests {
    use crate::moves::recursive_move_gen_test;

    #[test]
    fn move_gen_test() {
        use crate::board::Board;

        let board = Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        let num_moves_1 = recursive_move_gen_test(&board, 1, 20);
        assert_eq!(num_moves_1, 20);

        let num_moves_2 = recursive_move_gen_test(&board, 2, 400);
        assert_eq!(num_moves_2, 400);

        let num_moves_3 = recursive_move_gen_test(&board, 3, 8902);
        assert_eq!(num_moves_3, 8902); // 8149
    }
}
