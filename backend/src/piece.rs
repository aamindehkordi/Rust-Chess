/// A piece is represented as a byte: 8 bits.
///
/// The first 3 bits are the piece kind. & 7 is 00000111.
/// The next 5 bits are the color. & 24 is 00011000.
pub type PieceAsByte = u8;

#[derive(Debug, Copy, Clone, PartialEq)]
/// A color is an integer representing the color of a piece.
///
/// 8 is white, 16 is black.
pub enum Color {
    White = 8,
    Black = 16,
}
impl Color {
    /// Converts a piece to its corresponding color.
    ///
    /// # Arguments
    /// * `piece` - The piece value.
    ///
    /// # Returns
    /// The color of the corresponding piece.
    ///
    /// # Panics
    /// Panics if the piece does not have a valid color.
    ///
    /// # Example
    /// ```rs
    ///     let color = Square::from(16);
    /// ```
    pub fn from(piece: PieceAsByte) -> Color {
        match piece & 24 {
            8 => Color::White,
            16 => Color::Black,
            _ => panic!("Invalid color."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A piece kind is an integer representing the type of a piece.
///
/// The piece kinds from 0 to 6: None, King, Pawn, Knight, Bishop, Rook, Queen
pub enum PieceKind {
    None = 0,
    King = 1,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
}
impl PieceKind {
    /// Converts a piece represented as a byte to a PieceKind.
    ///
    /// # Arguments
    /// * `piece` - The piece to convert.
    ///
    /// # Returns
    /// The corresponding PieceKind.
    ///
    /// # Panics
    /// Panics if the piece kind is invalid.
    ///
    /// # Example
    /// ```rs
    ///     let piece_kind = Square::from(2);
    /// ```
    pub fn from(piece: PieceAsByte) -> PieceKind {
        match piece & 7 {
            0 => PieceKind::None,
            1 => PieceKind::King,
            2 => PieceKind::Pawn,
            3 => PieceKind::Knight,
            4 => PieceKind::Bishop,
            5 => PieceKind::Rook,
            6 => PieceKind::Queen,
            _ => panic!("Invalid piece kind."),
        }
    }
}
