use crate::board::Position;
use crate::game::player::Color;

use crate::board::board_info::BoardInfo;

use crate::rules::r#move::Move;
use crate::rules::{
    generate_king_moves, generate_knight_moves, generate_pawn_moves, generate_sliding_move,
};
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Display for PieceKind {
    /**
     * Formats the PieceKind enum variant as a string.
     *
     * This function formats the PieceKind enum variant as a string representation using the standard fmt::Formatter.
     *
     * @param f - A mutable reference to the fmt::Formatter used for formatting the output.
     *
     * @returns Result - Ok if formatting was successful, Err if an error occurred during formatting.
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceKind::Pawn => write!(f, "Pawn"),
            PieceKind::Rook => write!(f, "Rook"),
            PieceKind::Knight => write!(f, "Knight"),
            PieceKind::Bishop => write!(f, "Bishop"),
            PieceKind::Queen => write!(f, "Queen"),
            PieceKind::King => write!(f, "King"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub position: Position,
    pub color: Color,
    pub has_moved: bool,
    pub first_move: bool,
    pub en_passant: Option<bool>,
    pub can_castle: Option<bool>,
}

impl Piece {
    pub fn custom(
        kind: PieceKind,
        position: Position,
        color: Color,
        moved: bool,
        en_passant: Option<bool>,
        can_castle: Option<bool>,
    ) -> Self {
        match kind {
            PieceKind::Pawn => Self {
                kind,
                position,
                color,
                has_moved: moved,
                first_move: !moved,
                en_passant,
                can_castle,
            },
            PieceKind::King => Self {
                kind,
                position,
                color,
                has_moved: moved,
                first_move: !moved,
                en_passant,
                can_castle,
            },
            _ => Self {
                kind,
                position,
                color,
                has_moved: moved,
                first_move: !moved,
                en_passant,
                can_castle,
            },
        }
    }

    pub fn new(kind: PieceKind, position: Position, color: Color) -> Self {
        match kind {
            PieceKind::Pawn => Self {
                kind,
                position,
                color,
                has_moved: false,
                first_move: true,
                en_passant: Some(false),
                can_castle: None,
            },
            PieceKind::King => Self {
                kind,
                position,
                color,
                has_moved: false,
                first_move: true,
                en_passant: None,
                can_castle: Some(false),
            },
            _ => Self {
                kind,
                position,
                color,
                has_moved: false,
                first_move: true,
                en_passant: None,
                can_castle: None,
            },
        }
    }
}

/**
 * Converts the given piece to a character representation.
 *
 * This function takes a piece and returns its corresponding character representation. The character
 * representation is a single uppercase letter for white pieces and a single lowercase letter for black pieces.
 *
 * @param piece - The piece to convert to a character representation.
 * @return The character representation of the piece.
 */
pub fn to_char(piece: Piece) -> char {
    let mut char: String = String::new();
    char = match piece.kind {
        PieceKind::Pawn => "P",
        PieceKind::Rook => "R",
        PieceKind::Knight => "N",
        PieceKind::Bishop => "B",
        PieceKind::Queen => "Q",
        PieceKind::King => "K",
    }
    .parse()
    .unwrap();
    if piece.color == Color::Black {
        char = char.to_lowercase();
    }
    char.chars().next().unwrap()
}

impl PartialEq for Piece {
    /**
     * Checks if this Piece is equal to another Piece.
     *
     * This function compares the kind and color properties of this Piece with another Piece
     * to determine if they are equal.
     *
     * @param other - The other Piece to compare with.
     * @return true if the pieces are equal, false otherwise.
     */
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.color == other.color
    }
}

impl Display for Piece {
    /**
     * Formats the chess piece for display.
     *
     * This function formats the chess piece based on its kind and color. It returns a formatted string representation of the piece.
     *
     * @param f - The formatter to write the formatted string.
     * @return Result - Ok if the formatting is successful, Err otherwise.
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut char: String = String::new();
        char = match self.kind {
            PieceKind::Pawn => "P",
            PieceKind::Rook => "R",
            PieceKind::Knight => "N",
            PieceKind::Bishop => "B",
            PieceKind::Queen => "Q",
            PieceKind::King => "K",
        }
        .parse()
        .unwrap();
        if self.color == Color::Black {
            char = char.to_lowercase();
        }
        write!(f, "{}", char)
    }
}

/**
 * Returns a list of possible moves for the given piece on the chessboard.
 *
 * This function takes a reference to the chessboard information and a reference to the piece
 * and returns a vector of possible moves that the piece can make. The possible moves are generated
 * based on the kind of the piece: pawn, rook, knight, bishop, queen, or king.
 *
 * @param board_info - A reference to the chessboard information.
 * @param p - A reference to the piece for which moves are to be generated.
 * @return A vector of possible moves for the given piece.
 */
pub fn get_moves(board_info: &BoardInfo, p: &Piece) -> Vec<Move> {
    let piece = *p;
    match piece.kind {
        PieceKind::Pawn => generate_pawn_moves(board_info.clone(), piece),
        PieceKind::Rook => generate_sliding_move(board_info.clone(), piece),
        PieceKind::Knight => generate_knight_moves(board_info.clone(), piece),
        PieceKind::Bishop => generate_sliding_move(board_info.clone(), piece),
        PieceKind::Queen => generate_sliding_move(board_info.clone(), piece),
        PieceKind::King => generate_king_moves(board_info.clone(), piece),
    }
}
