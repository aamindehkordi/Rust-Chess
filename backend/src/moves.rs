use crate::board::*;
use crate::piece::*;

pub const DIRECTION_OFFSETS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
pub type Moves = Vec<SimpleMove>;

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
pub fn generate_moves(board: &Board) -> Moves {
    let mut moves = Moves::new();

    for start_square in 0..64 {
        let piece = board.squares[start_square].piece;
        if piece.type_ == PieceKind::None {
            continue;
        }

        if is_color(piece, board.turn) {
            match piece.type_ {
                PieceKind::Pawn => { let pawn_moves = generate_pawn_moves(board, start_square); moves.extend(pawn_moves); },
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
                },
                _ => (),
            }
        }
    }

    moves
}

/// Generates all possible moves for a Pawn on a given board.
///
/// # Arguments
/// * `board` - The board to generate moves for.
/// * `start_square` - The square the piece is on.
///
/// # Returns
/// A list of all possible moves for the given piece.
pub fn generate_pawn_moves(board: &Board, from: usize) -> Moves {
    let mut moves = Moves::new();
    let pawn_color = board.squares[from].color;

    // Determine the direction in which the pawn moves
    let direction = if pawn_color == Color::White { 8 } else { -8 };

    // Potential single square move
    let one_square_ahead = (from as i8 + direction) as usize;
    if board.squares[one_square_ahead].piece.type_ == PieceKind::None {
        moves.push((from, one_square_ahead));

        // Potential double square move if the pawn is on its starting rank
        let is_starting_rank = (pawn_color == Color::White && from / 8 == 1)
            || (pawn_color == Color::Black && from / 8 == 6);
        let two_squares_ahead = (from as i8 + 2 * direction) as usize;
        if is_starting_rank && board.squares[two_squares_ahead].piece.type_ == PieceKind::None {
            moves.push((from, two_squares_ahead));
        }
    }

    // Potential captures
    let capture_directions = if pawn_color == Color::White { [-9, -7] } else { [9, 7] };
    for &d in &capture_directions {
        let capture_square = (from as i8 + d) as usize;
        if capture_square < 64 && board.squares[capture_square].piece.type_ != PieceKind::None
            && board.squares[capture_square].color != pawn_color {
            moves.push((from, capture_square));
        }
    }

    // En Passant: Check if the last move was a two-square pawn advance
    if let Some(last_move) = board.move_history.last() {
        let last_from = last_move.0;
        let last_to = last_move.1;
        // Check if the move was a two-square pawn advance
        if board.squares[last_from].piece.type_ == PieceKind::Pawn
            && (last_to as i8 - last_from as i8).abs() == 16 // 2 * 8 = 2 squares
        {
            // Determine the en passant target square
            let en_passant_square = (last_to as i8 + direction / 2) as usize;
            // If the pawn can capture en passant
            if en_passant_square == one_square_ahead {
                moves.push((from, en_passant_square));
            }
        }
    }

    // Promotion: If the pawn reaches the opposite side of the board
    if one_square_ahead / 8 == if pawn_color == Color::White { 7 } else { 0 } {
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
pub fn generate_knight_moves(board: &Board, from: usize) -> Moves {
    // The list of moves to return.
    let mut moves = Moves::new();

    // Knight's possible movement offsets.
    let knight_offsets = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

    for (dx, dy) in knight_offsets.iter() {
        // Calculate the new position.
        let x = (from % 8) as isize + dx;
        let y = (from / 8) as isize + dy;

        // Check if the new position is within the board.
        if !(0..=7).contains(&x) || !(0..=7).contains(&y) {
            continue;
        }

        // Convert the x, y coordinates back to a position.
        let to = (y * 8 + x) as usize;

        // Check if the target square is occupied by a piece of the same color.
        if is_color(board.squares[to].piece, board.turn) {
            continue;
        }

        // Add the move to the list.
        moves.push((from, to));
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
pub fn generate_king_moves(board: &Board, from: usize) -> Moves {
    // The list of moves to return.
    let mut moves = Moves::new();

    // For each direction.
    for direction in DIRECTION_OFFSETS.iter() {
        // The end square.
        let end_square = (from as i8 + direction) as usize;
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
    if board.can_castle_kingside(board.turn) {
        moves.push((from, from + 2));
    }
    // Check for Queenside castling
    if board.can_castle_queenside(board.turn) {
        moves.push((from, from - 2));
    }

    moves
}
