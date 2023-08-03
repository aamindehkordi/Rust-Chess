use crate::board::Position;
use crate::game::player::Color;
use crate::game::Game;
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

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub position: Position,
    pub color: Color,
    pub has_moved: bool,
    pub en_passant: Option<bool>,
    pub can_castle: Option<bool>,
}

impl Piece {
    pub fn new(kind: PieceKind, position: Position, color: Color) -> Self {
        match kind {
            PieceKind::Pawn => Self {
                kind,
                position,
                color,
                has_moved: false,
                en_passant: Some(false),
                can_castle: None,
            },
            PieceKind::King => Self {
                kind,
                position,
                color,
                has_moved: false,
                en_passant: None,
                can_castle: Some(false),
            },
            _ => Self {
                kind,
                position,
                color,
                has_moved: false,
                en_passant: None,
                can_castle: None,
            },
        }
    }
}

pub fn get_moves(game: &Game, piece: &Piece) -> Vec<Move> {
    match piece.kind {
        PieceKind::Pawn => generate_pawn_moves(game.clone(), piece.position, piece.color),
        PieceKind::Rook => generate_sliding_move(game.clone(), piece.position, piece.color),
        PieceKind::Knight => generate_knight_moves(game.clone(), piece.position, piece.color),
        PieceKind::Bishop => generate_sliding_move(game.clone(), piece.position, piece.color),
        PieceKind::Queen => generate_sliding_move(game.clone(), piece.position, piece.color),
        PieceKind::King => generate_king_moves(game.clone(), piece.position, piece.color),
    }
}

impl Display for Piece {
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

pub const COLOR_IDX: [i8; 2] = [0, 1]; // [White, Black]
pub const PIECE_KIND_IDX: [u8; 6] = [0, 1, 2, 3, 4, 5]; // [King, Pawn, Knight, Bishop, Rook, Queen]
pub const PIECE_IDX: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // [White King, White Pawn, White Knight, White Bishop, White Rook, White Queen, Black King, Black Pawn, Black Knight, Black Bishop, Black Rook, Black Queen]

pub fn idx(kind: PieceKind, color: Color) -> usize {
    let mut idx: usize = 0;
    match kind {
        PieceKind::Pawn => idx = 1,
        PieceKind::Knight => idx = 2,
        PieceKind::Bishop => idx = 3,
        PieceKind::Rook => idx = 4,
        PieceKind::Queen => idx = 5,
        PieceKind::King => idx = 0,
    }
    if color == Color::Black {
        idx += 6;
    }
    idx
}

pub fn color_idx(color: Color) -> usize {
    let mut idx: usize = 0;
    if color == Color::Black {
        idx = 1;
    }
    idx
}
