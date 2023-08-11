use crate::board::bb::Bitboards;
use crate::board::square::Square;
use crate::board::Board;
use crate::game::Game;
use crate::piece::{Color, Piece, PieceKind};
use std::fmt::Display;

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
        if self.piece.color.is_none() || self.piece.color.unwrap() == Color::White {
            write!(f, "{}", piece)
        } else {
            write!(f, "{}", piece.to_lowercase())
        }
    }
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
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "{}", color)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.type_ {
            PieceKind::King => "King",
            PieceKind::Pawn => "Pawn",
            PieceKind::Knight => "Knight",
            PieceKind::Bishop => "Bishop",
            PieceKind::Rook => "Rook",
            PieceKind::Queen => "Queen",
            _ => "None",
        };

        let color = match self.color {
            Some(Color::White) => "White",
            Some(Color::Black) => "Black",
            None => "None",
        };
        write!(f, "{} {}", color, piece)
    }
}

impl Default for Bitboards {
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
impl Default for Game {
    /// Returns the default value for a square.
    ///
    /// # Returns
    /// The default square.
    ///
    /// # Example
    /// ```rs
    ///     let square = Square::default();
    /// ```
    fn default() -> Self {
        Self::new_standard()
    }
}
