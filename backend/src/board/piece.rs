use crate::board::{Position};
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
