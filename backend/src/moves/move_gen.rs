use crate::board::*;
use crate::moves::CastleSide::{KingSide, QueenSide};
use crate::moves::MoveType::{Capture, Castle, DoublePush, Quiet};
use crate::moves::{Move, MoveType, Moves, DIRECTION_OFFSETS};
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
pub fn generate_all_moves(board: &Board) -> Moves {
    let mut moves = Moves::new();

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
pub fn generate_legal_moves(board: &Board) -> Moves {
    // Generate all possible moves
    let psuedo_legal_moves = generate_all_moves(board);
    // Filter out all moves that leave the king in check
    let mut legal_moves = Moves::new();

    // For each move, make the move on a copy of the board and check if the king is in check
    for mv in psuedo_legal_moves {
        // Make a copy of the board
        let mut board_copy = board.clone();

        // Make the move temporarily
        board_copy.make_move(mv);
        // Get the position of the king
        let king_pos = board_copy.get_king_position(board_copy.turn.other());
        if king_pos.is_none() {
            legal_moves.push(mv);
            continue;
        }
        let king_pos = king_pos.unwrap();
        //Get opponent response moves
        let response_moves = generate_all_moves(&board_copy);

        // If the opponent can take our king, it is not a valid move
        if response_moves.iter().any(|&mv| mv.simple.1 == king_pos) {
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
pub fn generate_pawn_moves(board: &Board, from: usize) -> Moves {
    let mut moves = Moves::new();
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
        let simple = (from, one_square_ahead);
        let mv = Move::new(simple, Quiet);
        moves.push(mv);

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
            let simple = (from, two_squares_ahead);
            let mv = Move::new(simple, DoublePush);
            moves.push(mv);
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
            let simple = (from, capture_square);
            let mv = Move::new(simple, Capture);
            moves.push(mv);
        }
    }

    // En Passant: Check if the last move was a two-square pawn advance
    if board.bb.en_passant_square != 0 {
        let en_passant_square = board.bb.en_passant_square;
        let en_passant_direction = if pawn_color == Some(Color::White) {
            8
        } else {
            -8
        };
        let en_passant_square = (en_passant_square as i8 + en_passant_direction) as usize;
        if en_passant_square == from {
            let simple = (from, board.bb.en_passant_square);
            let mv = Move::new(simple, MoveType::EnPassant);
            moves.push(mv);
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
        let promotion_pieces = [
            PieceKind::Queen,
            PieceKind::Rook,
            PieceKind::Bishop,
            PieceKind::Knight,
        ];
        for promotion_piece in promotion_pieces.iter() {
            let promotion = MoveType::Promotion(*promotion_piece);
            let pro_capture = MoveType::PromotionCapture(*promotion_piece);
            let simple = (from, one_square_ahead);
            // Check if the square in front of the pawn is empty
            if board.squares[one_square_ahead].piece.type_ != PieceKind::None {
                if board.squares[one_square_ahead].piece.color != pawn_color {
                    let mv = Move::new(simple, pro_capture);
                    moves.push(mv);
                }
            } else {
                let mv = Move::new(simple, promotion);
                moves.push(mv);
            }
        }
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

    // The piece on the start square.
    let from_square = board.squares[from];
    let piece = from_square.piece;
    // The color of the piece.
    let color = piece.color.unwrap();

    // The directions to check.
    let directions = [-17, -15, -10, -6, 6, 10, 15, 17];

    // For each direction, check if the move is valid.
    for direction in directions.iter() {
        // The square to check.
        let to_pos = (from as i8 + direction) as usize;

        if !(0..=63).contains(&to_pos) {
            continue;
        }

        let to_square = board.squares[to_pos];

        // Knight moves end up on the opposite tile color as the start square.
        if to_square.tile_color == from_square.tile_color {
            continue;
        }

        // The piece on the end square.
        let to_piece = to_square.piece;

        if to_piece.type_ == PieceKind::None {
            let simple = (from, to_pos);
            let mv = Move::new(simple, Quiet);
            moves.push(mv);
        } else if to_piece.color != Some(color) {
            let simple = (from, to_pos);
            let mv = Move::new(simple, Capture);
            moves.push(mv);
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
    let num_squares_to_edge = board.num_squares_to_edge[start_square];

    // The index of the first direction to check.
    let start_dir_idx = if piece.type_ == PieceKind::Bishop {
        4
    } else {
        0
    };
    // The index of the last direction to check.
    let end_dir_idx = if piece.type_ == PieceKind::Rook { 4 } else { 7 };

    // For each direction.
    for direction_idx in start_dir_idx..end_dir_idx {
        let direction = DIRECTION_OFFSETS[direction_idx];
        // For each square in the direction.
        for num_squares  in 0..num_squares_to_edge[direction_idx] {
            // The end square offset.
            let end_square_off = start_square as i8 + direction * (num_squares + 1) as i8;
            if !(0..=63).contains(&end_square_off) {
                break;
            }
            // The end square.
            let end_square_pos = end_square_off as usize;
            // The piece on the end square.
            let piece_on_end_square = board.squares[end_square_pos].piece;

            let simple = (start_square, end_square_pos);
            // Blocked by a piece of the same color.
            if piece_on_end_square.type_ != PieceKind::None {
                if is_color(piece_on_end_square, piece.color.unwrap()) {
                    break;
                } else { // Blocked by a piece of the opposite color.
                    let mv = Move::new(simple, Capture);
                    moves.push(mv);
                    break;
                }
            }
            else { // Not blocked.
                let mv = Move::new(simple, Quiet);
                moves.push(mv);
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
        let end_square_pos = from as i8 + direction;
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

        let simple = (from, end_square_pos);
        let mv = Move::new(simple, Quiet);

        // Blocked by a piece of the opposite color.
        if is_color(piece_on_end_square, board.turn.other()) {
            let mv = Move::new(simple, Capture);
            moves.push(mv);
            continue;
        }
        moves.push(mv);
    }

    // Check for Kingside castling
    if board.can_castle(board.turn, KingSide) {
        let simple = (from, from + 2);
        let mv = Move::new(simple, Castle(KingSide));
        moves.push(mv);
    }
    // Check for Queenside castling
    if board.can_castle(board.turn, QueenSide) {
        let simple = (from, from - 2);
        let mv = Move::new(simple, Castle(QueenSide));
        moves.push(mv);
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
                _ => Moves::new(),
            };

            let mut new_board = board.clone();
            for mv in moves {
                new_board.make_move(mv);
                num_moves += recursive_move_gen_test(&new_board, depth - 1, expected);
                new_board.undo_move();
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
            b.make_move(mv);
            let sq = b.squares[mv.simple.1];
            let piece = sq.piece;
            println!("{}\n{} from {} to {}", b, piece, mv.simple.0, mv.simple.1);
            b.undo_move();
        }
    }

    pub fn get_piece_moves(board: &Board, from: Position, piece: Piece) -> Moves {
        let mut moves: Moves = Moves::new();

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
        let board = new_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let num_moves_1 = recursive_move_gen_test(&board, 1, 20);
        assert_eq!(num_moves_1, 20);

        let num_moves_2 = recursive_move_gen_test(&board, 2, 400);
        assert_eq!(num_moves_2, 400);

        let num_moves_3 = recursive_move_gen_test(&board, 3, 8902);
        assert_eq!(num_moves_3, 8902); // 7800
    }

    #[test]
    fn queen_test() {
        let board = new_board_from_fen("8/8/8/8/8/8/8/Q7 w - - 0 1");
        println!("{}", board);
        let num_moves = recursive_move_gen_test(&board, 1, 21);
        assert_eq!(num_moves, 21);
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
