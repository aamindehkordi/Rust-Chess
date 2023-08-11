pub mod bb;
pub mod square;

use crate::board::square::{Position, Square};
use crate::moves::move_gen::*;
use crate::moves::{CastleSide, SimpleMove, SimpleMoves};
use crate::piece::Color::White;
use crate::piece::*;

/// Precomputed values for the number of squares to the edge of the board from any square.
pub type NumSquaresToEdge = [[usize; 8]; 64];

#[derive(Debug, Clone)]
/// The board is an array of 64 squares.
/// The move history is a list of moves.
pub struct Board {
    pub squares: [Square; 64],
    pub move_history: SimpleMoves,
    pub num_squares_to_edge: NumSquaresToEdge,
    pub turn: Color,
}
impl Board {
    /// Creates a new board.
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
            move_history: SimpleMoves::new(),
            num_squares_to_edge: precomputed_move_data(),
            turn: White,
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
        Board::new_from_fen(fen)
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
    pub fn new_from_fen(fen: &str) -> Board {
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
        let mut file = 0;
        let mut rank = 7;

        // Parse the board.
        for symbol in fen_board[0].chars() {
            if symbol == '/' {
                file = 0;
                rank -= 1;
            } else if symbol.is_ascii_digit() {
                file += symbol.to_digit(10).unwrap();
            } else {
                let mut piece_color = White;
                if symbol.is_lowercase() {
                    piece_color = Color::Black;
                }
                let mut piece_kind = PieceKind::None;
                for (piece_symbol, piece_kind_) in piece_kind_from_symbol.iter() {
                    if symbol.to_ascii_uppercase() == *piece_symbol {
                        piece_kind = *piece_kind_;
                    }
                }
                let pos: Position = rank * 8usize + file as usize;
                let piece = piece_color as u8 + piece_kind as u8; // Rook
                board.set_piece(pos, piece);
                file += 1;
            }
        }

        board
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

    /// Sets a piece on the square at the given position.
    ///
    /// # Arguments
    /// * `position` - The position of the square.
    /// * `piece` - The piece to set on the square.
    ///
    /// # Example
    /// ```rs
    ///     let mut board = Board::new();
    ///     board.set_piece(4, PieceAsByte::King);
    /// ```
    pub fn set_piece(&mut self, position: Position, piece_byte: PieceAsByte) {
        self.squares[position].set_piece(piece_byte);
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
        // Get the columns of the rook, and castle squares.
        let cols: [usize; 3] = match side {
            CastleSide::KingSide => [7, 5, 6],
            CastleSide::QueenSide => [0, 3, 2],
        };

        // Get the rank of the king.
        let rank = if color == White { 0 } else { 7 };

        // Get the king and Rook squares.
        let king_square = self.squares[idx(rank, 4)];
        let rook_square = self.squares[idx(rank, cols[0])];

        // Check if the king or rook has moved.
        if king_square.has_moved
            || rook_square.has_moved
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

    /// Makes a simple move.
    ///
    /// # Arguments
    /// * `mv` - The move to make.
    pub fn make_simple_move(&mut self, mv: SimpleMove) {
        let from = mv.0;
        let to = mv.1;
        let piece = self.squares[from].piece;
        self.squares[from].set_piece(0);
        self.squares[to].set_piece(piece.to_byte());
        self.turn = self.turn.other();
        self.move_history.push(mv);
    }

    /// Unmakes a simple move.
    pub fn unmake_simple_move(&mut self) {
        let mv = self.move_history.pop();
        if mv.is_none() {
            return;
        }
        let mv = mv.unwrap();
        let from = mv.0;
        let to = mv.1;
        let piece = self.squares[to].piece;
        self.squares[to].set_piece(0);
        self.squares[from].set_piece(piece.to_byte());
        self.turn = self.turn.other();
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
