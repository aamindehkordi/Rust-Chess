use crate::piece::*;

/// The position is a number from 0 to 63.
pub type Position = usize;

#[derive(Debug, Copy, Clone)]
/// A square is a position on the board.
///
/// It contains a piece, a color, and rays that will be used for checks and pins.
pub struct Square {
    pub position: Position,
    pub tile_color: Color,
    pub piece: Piece,
    pub has_moved: bool,
    pub is_attacked: bool,
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
            tile_color: color,
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
