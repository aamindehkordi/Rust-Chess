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
                    generate_sliding_moves(board, start_square, &mut moves)
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
/// * `moves` - The list of moves to add to.
pub fn generate_sliding_moves(board: &Board, start_square: usize, moves: &mut Moves) {
    let piece = board.squares[start_square].piece;

    let start_dir_idx = if piece.type_ == PieceKind::Bishop { 4 } else { 0 };
    let end_dir_idx = if piece.type_ == PieceKind::Rook { 4 } else { 8 };

    let num_squares_to_edge = board.precomputed_move_data[start_square];
    for direction_idx in start_dir_idx..end_dir_idx {
        if direction_idx < start_dir_idx || direction_idx >= end_dir_idx {
            continue;
        }
        for num_squares in 1..num_squares_to_edge[direction_idx] {
            let end_square = (start_square as i8 + DIRECTION_OFFSETS[direction_idx] * (num_squares) as i8) as usize;
            let piece = board.squares[end_square].piece;
            if is_color(piece, board.turn) {
                break;
            }
            moves.push(Move::new(start_square, end_square));
            if is_color(piece, board.turn.other()) {
                break;
            }
        }
    }
}