use crate::moves::*;
use crate::piece::*;
use std::fmt::Display;

/// The position is a number from 0 to 63.
pub type Position = usize;

#[derive(Debug, Copy, Clone)]
/// The position is a number from 0 to 63.
/// The piece is a byte representing the piece or the just the color of the empty square.
pub struct Square {
    pub position: Position,
    pub color: Color,
    pub type_: PieceKind,
    pub piece: PieceAsByte,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.type_ {
            PieceKind::None => "_",
            PieceKind::King => "K",
            PieceKind::Pawn => "P",
            PieceKind::Knight => "N",
            PieceKind::Bishop => "B",
            PieceKind::Rook => "R",
            PieceKind::Queen => "Q",
        };
        if self.color == Color::Black {
            let piece = piece.to_lowercase();
            write!(f, "{}", piece)
        } else {
            write!(f, "{}", piece)
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
    pub fn new(position: Position, color: Color, type_: PieceKind) -> Square {
        Square {
            position,
            color,
            type_,
            piece: color as PieceAsByte + type_ as PieceAsByte,
        }
    }

    pub fn set_piece(&mut self, piece: PieceAsByte) {
        self.color = Color::from(piece);
        self.type_ = PieceKind::from(piece);
    }
}

#[derive(Debug, Clone)]
/// The board is an array of 64 squares.
/// The move history is a list of moves.
pub struct Board {
    pub squares: [Square; 64],
    pub move_history: Moves,
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
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        let mut squares: [Square; 64] = [Square::new(0, Color::White, PieceKind::None); 64];
        for (i, square) in squares.iter_mut().enumerate() {
            if i % 2 == 0 {
                square.color = Color::Black;
            }
            square.position = i;
        }
        Board {
            squares,
            move_history: Moves::new(),
        }
    }

    pub fn new_standard() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        new_from_fen(fen)
    }

    pub fn set_piece(&mut self, position: Position, piece: PieceAsByte) {
        self.squares[position].set_piece(piece);
    }
}
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
            let mut piece_color = Color::White;
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
