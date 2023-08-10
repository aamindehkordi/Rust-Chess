use crate::moves::*;
use crate::piece::Color::White;
use crate::piece::*;
use std::fmt::Display;

/// The position is a number from 0 to 63.
pub type Position = usize;

pub type NumSquaresToEdge = [[usize; 8]; 64];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

#[derive(Debug, Copy, Clone)]
/// The position is a number from 0 to 63.
/// The piece is a byte representing the piece or the just the color of the empty square.
pub struct Square {
    pub position: Position,
    pub color: Color,
    pub piece: Piece,
    pub has_moved: bool,
    pub is_attacked: bool,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.piece.type_ {
            PieceKind::None => "_",
            PieceKind::King => "K",
            PieceKind::Pawn => "P",
            PieceKind::Knight => "N",
            PieceKind::Bishop => "B",
            PieceKind::Rook => "R",
            PieceKind::Queen => "Q",
        };
        if self.piece.color.is_none() || self.piece.color.unwrap() == White {
            write!(f, "{}", piece)
        } else {
            write!(f, "{}", piece.to_lowercase())
        }
    }
}

impl Square {
    /// Creates a new square.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the square.
    /// * `color` - The color of the piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::*;
    /// use chess::piece::*;
    ///
    /// let square = Square::new(0, Color::White, PieceKind::King);
    /// ```
    pub fn new(position: Position, color: Color, piece_as_byte: PieceAsByte) -> Square {
        Square {
            position,
            color,
            piece: Piece::new(piece_as_byte),
            has_moved: false,
            is_attacked: false,
        }
    }

    pub fn from_str(position: &str, color: Color, piece_as_byte: PieceAsByte) -> Square {
        let position = match position {
            "a1" => 0,
            "b1" => 1,
            "c1" => 2,
            "d1" => 3,
            "e1" => 4,
            "f1" => 5,
            "g1" => 6,
            "h1" => 7,
            "a2" => 8,
            "b2" => 9,
            "c2" => 10,
            "d2" => 11,
            "e2" => 12,
            "f2" => 13,
            "g2" => 14,
            "h2" => 15,
            "a3" => 16,
            "b3" => 17,
            "c3" => 18,
            "d3" => 19,
            "e3" => 20,
            "f3" => 21,
            "g3" => 22,
            "h3" => 23,
            "a4" => 24,
            "b4" => 25,
            "c4" => 26,
            "d4" => 27,
            "e4" => 28,
            "f4" => 29,
            "g4" => 30,
            "h4" => 31,
            "a5" => 32,
            "b5" => 33,
            "c5" => 34,
            "d5" => 35,
            "e5" => 36,
            "f5" => 37,
            "g5" => 38,
            "h5" => 39,
            "a6" => 40,
            "b6" => 41,
            "c6" => 42,
            "d6" => 43,
            "e6" => 44,
            "f6" => 45,
            "g6" => 46,
            "h6" => 47,
            "a7" => 48,
            "b7" => 49,
            "c7" => 50,
            "d7" => 51,
            "e7" => 52,
            "f7" => 53,
            "g7" => 54,
            "h7" => 55,
            "a8" => 56,
            "b8" => 57,
            "c8" => 58,
            "d8" => 59,
            "e8" => 60,
            "f8" => 61,
            "g8" => 62,
            "h8" => 63,
            _ => 0,
        };

        Square {
            position,
            color,
            piece: Piece::new(piece_as_byte),
            has_moved: false,
            is_attacked: false,
        }
    }

    /// Sets the piece of the square.
    ///
    /// # Arguments
    /// * `piece` - The piece as a byte.
    ///
    /// # Example
    /// ```rs
    ///     let mut square = Square::new(4, Color::Black, PieceKind::King);
    ///     square.set_piece(27);
    /// ```
    pub fn set_piece(&mut self, piece: PieceAsByte) {
        self.piece = Piece::new(piece);
    }

    pub fn is_empty(&self) -> bool {
        self.piece.type_ == PieceKind::None
    }

    pub fn is_occupied(&self) -> bool {
        self.piece.type_ != PieceKind::None
    }
}

#[derive(Debug, Clone)]
/// The board is an array of 64 squares.
/// The move history is a list of moves.
pub struct Board {
    pub squares: [Square; 64],
    pub move_history: SimpleMoves,
    pub num_squares_to_edge: NumSquaresToEdge,
    pub turn: Color,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for i in 0..64 {
            board.push_str(&format!("{} ", self.squares[i]));
            if i % 8 == 7 {
                board.push('\n');
            }
        }
        write!(f, "{}", board)
    }
}
impl Default for Board {
    /// Returns the default value for the given type.
    ///
    /// # Returns
    /// The default value.
    ///
    /// # Example
    /// ```rs
    ///     let default = Square::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
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
                square.color = Color::Black;
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

    pub fn get_square(&self, position: Position) -> Square {
        self.squares[position]
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
