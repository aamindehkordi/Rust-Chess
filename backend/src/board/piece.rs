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

impl Display for PieceKind {
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

#[derive(Clone, Copy)]
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
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.color == other.color
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

pub fn get_moves(game: &Game, p: &Piece) -> Vec<Move> {
    let piece = *p;
    match piece.kind {
        PieceKind::Pawn => generate_pawn_moves(game.clone(), piece),
        PieceKind::Rook => generate_sliding_move(game.clone(), piece),
        PieceKind::Knight => generate_knight_moves(game.clone(), piece),
        PieceKind::Bishop => generate_sliding_move(game.clone(), piece),
        PieceKind::Queen => generate_sliding_move(game.clone(), piece),
        PieceKind::King => generate_king_moves(game.clone(), piece),
    }
}
