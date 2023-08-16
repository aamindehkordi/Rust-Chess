use crate::board::Board;
use crate::moves::{CastleSide, FromTo, MoveType};

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
        pub has_moved: bool,
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
                has_moved: false,
            };
        }
        Piece {
            color: None,
            type_: PieceKind::None,
            has_moved: false,
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

    pub fn get_move_type(&self, mv: FromTo, board: &Board) -> MoveType {
        let (from, to) = mv;
        let from_square = board.squares[from];
        let piece = from_square.piece;
        let piece_type = piece.type_;
        let piece_color = piece.color.unwrap();

        let to_square = board.squares[to];
        let to_piece = to_square.piece;
        let to_piece = to_piece;
        let to_piece_type = to_piece.type_;
        let to_piece_color = to_piece.color;

        let delta = (to as i8 - from as i8).abs();

        let is_possible_capture = to_piece_type != PieceKind::None;
        let is_double_push = piece_type == PieceKind::Pawn
            && !piece.has_moved
            && (delta == 16 || delta == 32);
        let is_en_passant =
            piece_type == PieceKind::Pawn
            && (delta == 7 || delta == 9)
            && to_piece_type == PieceKind::None
            && to_piece_color == Some(piece_color.other());
        let is_castle = piece_type == PieceKind::King
            && !piece.has_moved
            && (to == 2 || to == 6 || to == 58 || to == 62);
        let is_promotion = piece_type == PieceKind::Pawn
            && (to < 8 || to > 55)
            && to_piece_type == PieceKind::None;
        let is_promotion_capture = piece_type == PieceKind::Pawn
            && (to < 8 || to > 55)
            && is_possible_capture
            && to_piece_type != PieceKind::None;
        if is_en_passant {
            MoveType::EnPassant
        } else if is_castle {
            if to == 2 || to == 58 {
                MoveType::Castle(CastleSide::QueenSide)
            } else {
                MoveType::Castle(CastleSide::KingSide)
            }
        } else if is_promotion {
            MoveType::Promotion(PieceKind::Queen)
        } else if is_promotion_capture {
            MoveType::PromotionCapture(PieceKind::Queen)
        } else if is_possible_capture {
            MoveType::Capture
        }else if is_double_push {
            MoveType::DoublePush
        } else {
            MoveType::Quiet
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
