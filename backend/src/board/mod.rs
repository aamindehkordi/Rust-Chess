pub mod bb;
pub mod square;

use crate::board::bb::*;
use crate::board::square::*;
use crate::moves::move_gen::*;
use crate::moves::*;
use crate::piece::*;

/// Precomputed values for the number of squares to the edge of the board from any square.
pub type NumSquaresToEdge = [[usize; 8]; 64];

#[derive(Debug, Clone)]
/// The board is an array of 64 squares.
/// The move history is a list of moves.
pub struct Board {
    /// The squares of the board.
    pub squares: [Square; 64],
    /// The bitboards of the board.
    pub bb: Bitboards,
    /// The move history of the board.
    pub move_history: Moves,
    /// The legal moves of the board.
    pub legal_moves: Moves,
    /// Precomputed values for the number of squares to the edge of the board from any square.
    pub num_squares_to_edge: NumSquaresToEdge,
    /// The turn of the board.
    pub turn: Color,
}
impl Board {
    /// Creates a new empty board.
    ///
    /// # Returns
    /// A new board with the squares initialized.
    ///
    /// # Example
    /// ```rs
    ///    let board = Board::new();
    /// ```
    pub fn new() -> Board {
        let mut squares: [Square; 64] = [Square::new(0, Color::White, 0); 64];
        for (i, square) in squares.iter_mut().enumerate() {
            if i % 2 == 0 {
                square.tile_color = Color::Black;
            }
            square.position = i;
        }
        Board {
            squares,
            bb: Bitboards::new(),
            legal_moves: Vec::new(),
            move_history: Moves::new(),
            num_squares_to_edge: precomputed_move_data(),
            turn: Color::White,
        }
    }

    /// Creates a new standard chess board.
    ///
    /// # Returns
    /// A new standard chess board.
    ///
    /// # Example
    /// ```rs
    ///     let board = Board::new_standard();
    /// ```
    pub fn new_standard() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        new_board_from_fen(fen)
    }

    /// Creates a new custom chess board.
    ///
    /// # Arguments
    /// * `fen` - The fen string.
    ///
    /// # Returns
    /// A new custom chess board.
    pub fn new_custom(fen: &str) -> Board {
        new_board_from_fen(fen)
    }

    /// Returns the square at the given position.
    ///
    /// # Arguments
    /// * `position` - The position of the square.
    ///
    /// # Returns
    /// The square at the given position.
    pub fn get_square(&self, position: Position) -> Square {
        self.squares[position]
    }

    /// Moves a piece from one square to another.
    ///
    /// # Arguments
    /// * `from` - The position of the square to move from.
    /// * `to` - The position of the square to move to.
    pub fn move_piece(&mut self, from: Position, to: Position) {
        let piece = self.squares[from].piece;
        self.squares[from].set_piece(0);
        self.squares[to].set_piece(piece.to_byte());
        self.squares[to].piece.has_moved = true;
    }

    /// Checks if the color can castle on the given side.
    ///
    /// # Arguments
    /// * `color` - The color of the player.
    /// * `side` - The side to castle on.
    ///
    /// # Returns
    /// True if the player can castle on the given side.
    pub fn can_castle(&self, color: Color, side: CastleSide) -> bool {
        if !self.bb.can_castle(side, color) {
            return false;
        }

        // Get the columns of the rook, and castle squares.
        let cols: [usize; 3] = match side {
            CastleSide::KingSide => [7, 5, 6],
            CastleSide::QueenSide => [0, 3, 2],
        };

        // Get the rank of the king.
        let rank = if color == Color::White { 0 } else { 7 };

        // Get the king and Rook squares.
        let king_square = self.squares[idx(rank, 4)];
        let rook_square = self.squares[idx(rank, cols[0])];

        // Check if the king or rook has moved.
        if king_square.piece.has_moved
            || rook_square.piece.has_moved
            || king_square.is_attacked
            || rook_square.is_attacked
        {
            return false;
        }

        // Check if any of the castle squares are attacked.
        let castle_squares = [
            self.get_square(idx(rank, cols[1])),
            self.get_square(idx(rank, cols[2])),
        ];
        for square in castle_squares.iter() {
            if square.is_attacked {
                return false;
            }
        }
        // Last castle square for queen side.
        if side == CastleSide::QueenSide {
            let square = self.get_square(idx(rank, 1));
            if square.is_attacked {
                return false;
            }
        }

        // Check if any of the castle squares are occupied.
        for square in castle_squares.iter() {
            if square.is_occupied() {
                return false;
            }
        }

        true
    }

    /// Gets the position of the king of the given color.
    ///
    /// # Arguments
    /// * `color` - The color of the king.
    ///
    /// # Returns
    /// The position of the king.
    pub fn get_king_position(&self, color: Color) -> Position {
        let mut king_position = 0;
        for (i, square) in self.squares.iter().enumerate() {
            if square.is_occupied()
                && square.piece.type_ == PieceKind::King
                && square.piece.color == Some(color)
            {
                king_position = i;
            }
        }
        king_position
    }

    /// Updates the attacked squares of the board by iterating through the legal moves of the current turn.
    fn update_attacked_squares(&mut self) {
        self.bb.reset();
        let mut position;

        // For each square on the board.
        for square in self.squares.iter_mut() {
            square.is_attacked = false;
            position = square.position;
            // If the square is occupied.
            if square.is_occupied() {
                // Update the attacked squares.
                for mv in self.legal_moves.iter() {
                    if mv.simple.1 == position {
                        square.is_attacked = true;
                    }
                }
            }
        }

        self.bb.update(&self.squares);
    }

    pub fn make_move(&mut self, mv: Move) {
        match mv.move_type {
            MoveType::Quiet => self.make_simple_move(mv.simple),
            MoveType::DoublePush => {
                self.make_simple_move(mv.simple);
                self.bb.en_passant_square = mv.simple.1;
            }
            MoveType::Capture => {
                self.make_simple_move(mv.simple);
            }
            MoveType::EnPassant => {
                self.make_simple_move(mv.simple);
                let (_, to) = mv.simple;
                self.squares[to].set_piece(0);
            }
            MoveType::Castle(side) => {
                let rank = if self.turn == Color::White { 0 } else { 7 };
                let cols: [usize; 3] = match side {
                    CastleSide::KingSide => [7, 5, 6],
                    CastleSide::QueenSide => [0, 3, 2],
                };
                let king_square = self.squares[idx(rank, 4)];
                let rook_square = self.squares[idx(rank, cols[0])];
                self.move_piece(king_square.position, idx(rank, cols[2]));
                self.move_piece(rook_square.position, idx(rank, cols[1]));
            }
            MoveType::Promotion(piece_kind) => {
                self.make_simple_move(mv.simple);
                let (_, to) = mv.simple;
                let piece_color = self.turn;
                let piece = Piece::new(piece_color as u8 + piece_kind as u8);
                self.squares[to].set_piece(piece.to_byte());
            }
            MoveType::PromotionCapture(piece_kind) => {
                self.make_simple_move(mv.simple);
                let (_, to) = mv.simple;
                let piece_color = self.turn;
                let piece = Piece::new(piece_color as u8 + piece_kind as u8);
                self.squares[to].set_piece(piece.to_byte());
            }
        }
        self.move_history.push(mv);
        self.turn = self.turn.other();
        self.update_attacked_squares();
    }

    pub fn undo_move(&mut self) {
        let mv = self.move_history.pop();
        if mv.is_none() {
            return;
        }
        let mv = mv.unwrap();
        match mv.move_type {
            MoveType::Quiet => self.undo_simple_move(mv.simple),
            MoveType::DoublePush => {
                self.undo_simple_move(mv.simple);
                self.bb.en_passant_square = mv.simple.1;
            }
            MoveType::Capture => {
                self.undo_simple_move(mv.simple);
            }
            MoveType::EnPassant => {
                self.undo_simple_move(mv.simple);
                let (_, to) = mv.simple;
                let piece_color = self.turn.other();
                let piece = Piece::new(piece_color as u8 + PieceKind::Pawn as u8);
                self.squares[to].set_piece(piece.to_byte());
            }
            MoveType::Castle(side) => {
                let rank = if self.turn == Color::White { 0 } else { 7 };
                let cols: [usize; 3] = match side {
                    CastleSide::KingSide => [7, 5, 6],
                    CastleSide::QueenSide => [0, 3, 2],
                };
                let king_square = self.squares[idx(rank, cols[2])];
                let rook_square = self.squares[idx(rank, cols[1])];
                self.move_piece(king_square.position, idx(rank, 4));
                self.move_piece(rook_square.position, idx(rank, cols[0]));
            }
            MoveType::Promotion(_) => {
                self.undo_simple_move(mv.simple);
                let (_, to) = mv.simple;
                let piece_color = self.turn.other();
                let piece = Piece::new(piece_color as u8 + PieceKind::Pawn as u8);
                self.squares[to].set_piece(piece.to_byte());
            }
            MoveType::PromotionCapture(_) => {
                self.undo_simple_move(mv.simple);
                let (_, to) = mv.simple;
                let piece_color = self.turn.other();
                let piece = Piece::new(piece_color as u8 + PieceKind::Pawn as u8);
                self.squares[to].set_piece(piece.to_byte());
            }
        }
        self.turn = self.turn.other();
        self.update_attacked_squares();
    }

    /// Makes a simple move.
    ///
    /// # Arguments
    /// * `mv` - The move to make
    /// .
    pub fn make_simple_move(&mut self, mv: FromTo) {
        let from = mv.0;
        let to = mv.1;
        self.move_piece(from, to);
    }

    pub fn undo_simple_move(&mut self, mv: FromTo) {
        let from = mv.1;
        let to = mv.0;
        self.move_piece(to, from);
    }

    pub fn is_check(&self) -> bool {
        let king_position = self.get_king_position(self.turn);
        self.squares[king_position].is_attacked
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_check() && generate_legal_moves(self).is_empty()
    }
}

#[inline]
/// Returns the index of the square.
///
/// # Arguments
/// * `row` - The row of the square.
/// * `col` - The column of the square.
///
/// # Returns
/// The index of the square.
pub fn idx(row: usize, col: usize) -> usize {
    row * 8 + col
}

/// Precomputes the number of squares to the edge of the board.
/// This is used for move generation.
///
/// # Returns
/// A 2d array of the number of squares to the edge of the board.
///
/// # Example
/// ```rs
///    let precomputed_move_data = Board::precomputed_move_data();
/// ```
fn precomputed_move_data() -> NumSquaresToEdge {
    let mut num_squares_to_edge: NumSquaresToEdge = [[0; 8]; 64];
    for file in 0..8 {
        for rank in 0..8 {
            let north = 7 - rank;
            let south = rank;
            let east = 7 - file;
            let west = file;

            let square = rank * 8 + file;

            num_squares_to_edge[square] = [
                north,
                south,
                east,
                west,
                north.min(east),
                north.min(west),
                south.min(east),
                south.min(west),
            ];
        }
    }
    num_squares_to_edge
}

/// Creates a new board from a fen string.
///
/// # Arguments
/// * `fen` - The fen string.
///
/// # Returns
/// A new board.
///
/// # Example
/// ```rs
///    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
///    let board = Board::new_from_fen(fen);
/// ```
pub fn new_board_from_fen(fen: &str) -> Board {
    let mut board = Board::new();

    // Dictionary of piece kinds.
    let piece_kind_from_symbol = [
        ('K', PieceKind::King),
        ('Q', PieceKind::Queen),
        ('R', PieceKind::Rook),
        ('B', PieceKind::Bishop),
        ('N', PieceKind::Knight),
        ('P', PieceKind::Pawn),
    ];

    // Split the fen into parts.
    let fen_board: Vec<&str> = fen.split(' ').collect();

    // Parse the turn.
    board.turn = match fen_board[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => panic!("Invalid turn."),
    };

    // Parse the castling rights.
    let castling_rights = fen_board[2];
    for symbol in castling_rights.chars() {
        match symbol {
            'K' => board.bb.castling_rights[0] = true,
            'Q' => board.bb.castling_rights[1] = true,
            'k' => board.bb.castling_rights[2] = true,
            'q' => board.bb.castling_rights[3] = true,
            '-' => (),
            _ => panic!("Invalid castling rights."),
        }
    }

    // Parse the en passant square.
    let en_passant_square = fen_board[3];
    if en_passant_square != "-" {
        let file = en_passant_square.chars().next().unwrap() as u8 - 97;
        let rank = en_passant_square.chars().nth(1).unwrap() as u8 - 49;
        board.bb.en_passant_square = idx(rank as usize, file as usize);
    }

    // Parse the half move clock.
    //board.bb.half_move_clock = fen_board[4].parse::<u8>().unwrap();

    // Parse the full move number.
    //board.bb.full_move_number = fen_board[5].parse::<u8>().unwrap();

    // Set the file and rank to 0. (a1)
    let mut file = 0;
    let mut rank = 7;

    // Parse the board.
    for symbol in fen_board[0].chars() {
        if symbol == ' ' {
            // End of board.
            break;
        } else if symbol == '/' {
            // End of rank.
            file = 0;
            rank -= 1;
        } else if symbol.is_ascii_digit() {
            // Empty squares.
            file += symbol.to_digit(10).unwrap();
        } else {
            // Piece.
            let mut piece_color = Color::White;
            if symbol.is_lowercase() {
                // Black piece.
                piece_color = Color::Black;
            }
            let mut piece_kind = PieceKind::None;
            // Get the piece kind from the symbol.
            for (piece_symbol, pk) in piece_kind_from_symbol.iter() {
                if symbol.to_ascii_uppercase() == *piece_symbol {
                    // Piece found.
                    piece_kind = *pk;
                }
            }
            // Get the piece and position.
            let pos = idx(rank as usize, file as usize);
            let piece = piece_color as u8 + piece_kind as u8;
            // Get the bitboard type.
            let bitboard_type = match piece_color {
                Color::White => match piece_kind {
                    PieceKind::Pawn => BitboardType::WhitePawns,
                    PieceKind::Knight => BitboardType::WhiteKnights,
                    PieceKind::Bishop => BitboardType::WhiteBishops,
                    PieceKind::Rook => BitboardType::WhiteRooks,
                    PieceKind::Queen => BitboardType::WhiteQueens,
                    PieceKind::King => BitboardType::WhiteKing,
                    _ => BitboardType::WhiteOccupied,
                },
                Color::Black => match piece_kind {
                    PieceKind::Pawn => BitboardType::BlackPawns,
                    PieceKind::Knight => BitboardType::BlackKnights,
                    PieceKind::Bishop => BitboardType::BlackBishops,
                    PieceKind::Rook => BitboardType::BlackRooks,
                    PieceKind::Queen => BitboardType::BlackQueens,
                    PieceKind::King => BitboardType::BlackKing,
                    _ => BitboardType::BlackOccupied,
                },
            };
            // Set the bitboard and piece.
            board.bb.set_bit(bitboard_type, pos);
            board.squares[pos].set_piece(piece);
            // Increment the file.
            file += 1;
        }
    }
    // Generate the legal moves.
    board.legal_moves = generate_legal_moves(&board);
    // Update the attacked squares.
    board.update_attacked_squares();
    board
}
