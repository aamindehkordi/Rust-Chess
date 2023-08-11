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
    /// White is represented by 8.
    White = 8,
    /// Black is represented by 16.
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
    pub fn from(piece: PieceAsByte) -> Option<Color> {
        match piece & 24 {
            8 => Some(Color::White),
            16 => Some(Color::Black),
            0 => None,
            _ => panic!("Invalid color."),
        }
    }

    pub fn other(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A piece kind is an integer representing the type of a piece.
///
/// The piece kinds from 0 to 6: None, King, Pawn, Knight, Bishop, Rook, Queen
pub enum PieceKind {
    /// None is represented by 0.
    None = 0,
    /// King is represented by 1.
    King = 1,
    /// Pawn is represented by 2.
    Pawn = 2,
    /// Knight is represented by 3.
    Knight = 3,
    /// Bishop is represented by 4.
    Bishop = 4,
    /// Rook is represented by 5.
    Rook = 5,
    /// Queen is represented by 6.
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

#[derive(Debug, Copy, Clone, PartialEq)]
/// A piece is a combination of a color and a piece kind.
/// It is represented as a byte.
/// The first 3 bits are the piece kind. & 7 is 00000111.
/// The next 5 bits are the color. & 24 is 00011000.
pub struct Piece {
    pub color: Option<Color>,
    pub type_: PieceKind,
}
impl Piece {
    /// Creates a new piece.
    ///
    /// # Arguments
    /// * `color` - The color of the piece.
    /// * `type_` - The type of the piece.
    ///
    /// # Example
    /// ```rs
    ///     let piece = Piece::new(Color::White, PieceKind::King);
    /// ```
    pub fn new(piece_as_byte: PieceAsByte) -> Piece {
        if let Some(color) = Color::from(piece_as_byte) {
            return Piece {
                color: Some(color),
                type_: PieceKind::from(piece_as_byte),
            };
        }
        Piece {
            color: None,
            type_: PieceKind::from(piece_as_byte),
        }
    }

    /// Converts a piece to its corresponding byte.
    ///
    /// # Arguments
    /// * `piece` - The piece to convert.
    ///
    /// # Returns
    /// The corresponding byte.
    ///
    /// # Example
    /// ```rs
    ///     let piece = Piece::new(Color::White, PieceKind::King);
    ///     let byte = piece.to_byte();
    /// ```
    pub fn to_byte(&self) -> PieceAsByte {
        match self.color {
            Some(Color::White) => self.type_ as PieceAsByte | 8, // | adds the color to the piece
            Some(Color::Black) => self.type_ as PieceAsByte | 16, // 8 is 00001000, 16 is 00010000
            None => self.type_ as PieceAsByte,
        }
    }
}

pub fn is_color(piece: Piece, color: Color) -> bool {
    if let Some(piece_color) = piece.color {
        piece_color == color
    } else {
        false
    }
}

pub fn is_type(piece: Piece, type_: PieceKind) -> bool {
    piece.type_ == type_
}

pub fn is_sliding_piece(piece: Piece) -> bool {
    piece.type_ == PieceKind::Bishop
        || piece.type_ == PieceKind::Rook
        || piece.type_ == PieceKind::Queen
}
