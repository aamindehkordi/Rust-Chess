use crate::board::*;
use crate::piece::*;

pub const DIRECTION_OFFSETS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
pub type Moves = Vec<Move>;

#[derive(Debug, Copy, Clone)]
/// A move is a pair of positions.
/// The first position is the position of the piece to move.
/// The second position is the position to move the piece to.
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    /// Creates a new move.
    ///
    /// # Arguments
    /// * `from` - The position of the piece to move.
    /// * `to` - The position to move the piece to.
    ///
    /// # Example
    /// ```rs
    ///     let move_ = Move::new(0, 1);
    /// ```
    pub fn new(from: Position, to: Position) -> Move {
        Move { from, to }
    }
}

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
pub fn generate_moves(board: &Board) -> Moves {
    let mut moves = Moves::new();

    for start_square in 0..64 {
        let piece = board.squares[start_square].piece;
        if piece.type_ == PieceKind::None {
            continue;
        }

        if is_color(piece, board.turn) {
            match piece.type_ {
                //PieceKind::Pawn => generate_pawn_moves(board, start_square, &mut moves),
                //PieceKind::Knight => generate_knight_moves(board, start_square, &mut moves),
                PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                    let sliding_moves = generate_sliding_moves(board, start_square);
                    moves.extend(sliding_moves);
                }
                //PieceKind::King => generate_king_moves(board, start_square, &mut moves),
                _ => (),
            }
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
pub fn generate_sliding_moves(board: &Board, start_square: usize) -> Moves {
    // The list of moves to return.
    let mut moves = Moves::new();
    // The piece on the start square.
    let piece = board.squares[start_square].piece;

    // The index of the first direction to check.
    let start_dir_idx = if piece.type_ == PieceKind::Bishop { 4 } else { 0 };
    // The index of the last direction to check.
    let end_dir_idx = if piece.type_ == PieceKind::Rook { 4 } else { 8 };

    // For each direction.
    for direction_idx in start_dir_idx..end_dir_idx {
        // If the piece is a bishop and the direction is not diagonal.
        if direction_idx < start_dir_idx || direction_idx >= end_dir_idx {
            continue;
        }
        // For each square in the direction.
        for num_squares in 1..board.num_squares_to_edge[start_square][direction_idx] {
            // The end square.
            let end_square = (DIRECTION_OFFSETS[direction_idx] * num_squares as i8
                + start_square as i8) as usize;
            // The piece on the end square.
            let piece_on_end_square = board.squares[end_square].piece;

            // Blocked by a piece of the same color.
            if is_color(piece_on_end_square, board.turn) {
                break;
            }
            // Add the move.
            moves.push(Move::new(start_square, end_square));

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
pub fn generate_king_moves(board: &Board, start_square: usize) -> Moves {
    // The list of moves to return.
    let mut moves = Moves::new();

    // For each direction.
    for direction in DIRECTION_OFFSETS.iter() {
        // The end square.
        let end_square = (start_square as i8 + direction) as usize;
        // The piece on the end square.
        let piece_on_end_square = board.squares[end_square].piece;

        // Blocked by a piece of the same color.
        if is_color(piece_on_end_square, board.turn) {
            continue;
        }
        // Add the move.
        moves.push(Move::new(start_square, end_square));
    }

    moves
}
