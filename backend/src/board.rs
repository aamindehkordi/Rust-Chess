use std::fmt;

use crate::moves::{CastleType, Move, MoveType};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub moves_count: u8,
}
impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self {
            color,
            kind,
            moves_count: 0,
        }
    }

}

pub fn piece_to_char(piece: Piece) -> char {
    let color = piece.color;
    let kind = piece.kind;
    match color {
        Color::White => match kind {
            PieceKind::Pawn => 'P',
            PieceKind::Rook => 'R',
            PieceKind::Knight => 'N',
            PieceKind::Bishop => 'B',
            PieceKind::Queen => 'Q',
            PieceKind::King => 'K',
        },
        Color::Black => match kind {
            PieceKind::Pawn => 'p',
            PieceKind::Rook => 'r',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        },
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board(Vec<Option<Piece>>);
impl Board {

    pub fn new() -> Self {
        Board(vec![None; 8 * 8])
    }

    pub fn new_standard() -> Self {
        
        from_fen("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr")
    }

    #[inline]
    fn idx(x: u8, y: u8) -> usize {
        (y * 8 + x) as usize
    }

    pub fn get(&self, x: u8, y: u8) -> Option<Piece> {
        self.0[Self::idx(x, y)]
    }

    pub fn set(&mut self, x: u8, y: u8, piece: Option<Piece>) {
        self.0[Self::idx(x, y)] = piece;
    }

    pub fn iter_pieces(&self, color: Color) -> impl Iterator<Item=(u8, u8, Piece)> + '_ {
        self.0.iter().enumerate().filter_map(move |(i, piece)| {
            if let Some(piece) = piece {
                if piece.color == color {
                    let x = (i % 8) as u8;
                    let y = (i / 8) as u8;
                    Some((x, y, *piece))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

pub fn from_fen(fen: &str) -> Board {
    let mut board = Board::new();
    let mut x = 0;
    let mut y = 0;
    for c in fen.chars() {
        match c {
            '/' => {
                x = 0;
                y += 1;
            },
            '1'..='8' => {
                let n = c as u8 - b'0';
                for _ in 0..n {
                    board.set(x, y, None);
                    x += 1;
                }
            },
            'p' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::Pawn)));
                x += 1;
            },
            'r' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::Rook)));
                x += 1;
            },
            'n' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::Knight)));
                x += 1;
            },
            'b' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::Bishop)));
                x += 1;
            },
            'q' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::Queen)));
                x += 1;
            },
            'k' => {
                board.set(x, y, Some(Piece::new(Color::Black, PieceKind::King)));
                x += 1;
            },
            'P' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::Pawn)));
                x += 1;
            },
            'R' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::Rook)));
                x += 1;
            },
            'N' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::Knight)));
                x += 1;
            },
            'B' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::Bishop)));
                x += 1;
            },
            'Q' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::Queen)));
                x += 1;
            },
            'K' => {
                board.set(x, y, Some(Piece::new(Color::White, PieceKind::King)));
                x += 1;
            }
            _ => panic!("Invalid FEN string"),
        }
    }
    board
}

pub fn to_fen(board: &Board) -> String {
    let mut fen = String::new();
    let mut empty_count = 0;

    for y in 0..8 {
        for x in 0..8 {
            match board.get(x, y) {
                Some(piece) => {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece_to_char(piece));
                },
                None => {
                    empty_count += 1;
                },
            }
        }
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            empty_count = 0;
        }
        if y < 7 {
            fen.push('/');
        }
    }
    fen
}

pub fn find_pieces_pos(board: &Board, kind: PieceKind) -> Vec<(u8, u8)> {
    let mut pieces = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board.get(x, y) {
                if kind == piece.kind {
                    pieces.push((x, y));
                };
            }
        }
    }
    pieces
}

pub fn find_piece_pos(board: &Board, color: Color, kind: PieceKind) -> Option<(u8, u8)> {
    let mut pieces = find_pieces_pos(board, kind);
    for (x, y) in pieces.drain(..) {
        if let Some(piece) = board.get(x, y) {
            if color == piece.color {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn king_pos(board: &Board, color: Color) -> (u8, u8) {
    find_piece_pos(board, color, PieceKind::King).unwrap_or_else(|| panic!("King not found"))
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                match self.get(x, y) {
                    Some(piece) => write!(f, "{}", piece_to_char(piece))?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn increment_piece_move_count(board: &mut Board, from: (u8, u8)) {
    if let Some(piece) = board.get(from.0, from.1) {
        board.set(from.0, from.1, Some(Piece { moves_count: piece.moves_count + 1, ..piece }));
    }
}

pub fn in_bounds(pos: &(u8, u8)) -> bool {
    pos.0 < 8 && pos.1 < 8
}

pub fn make_move(b: Board, mv: &Move) -> Board {
    let mut board = b;
    match mv.move_type {
        MoveType::Normal => {
            let piece = board.get(mv.from.0, mv.from.1).unwrap();
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
        },
        MoveType::DoublePawnPush => {
            let piece = board.get(mv.from.0, mv.from.1).unwrap();
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
        },
        MoveType::Capture => {
            let piece = board.get(mv.from.0, mv.from.1).unwrap();
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
        },
        MoveType::Castle(castle_type) => {
            let king = board.get(mv.from.0, mv.from.1).unwrap();
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(king));
            match castle_type {
                CastleType::KingSide => {
                    let rook = board.get(7, mv.from.1).unwrap();
                    board.set(7, mv.from.1, None);
                    board.set(5, mv.from.1, Some(rook));
                },
                CastleType::QueenSide => {
                    let rook = board.get(0, mv.from.1).unwrap();
                    board.set(0, mv.from.1, None);
                    board.set(3, mv.from.1, Some(rook));
                },
            }
        },
        MoveType::EnPassant => {
            let piece = board.get(mv.from.0, mv.from.1).unwrap();
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
            board.set(mv.to.0, mv.from.1, None);
        },
        MoveType::Promotion(piece_kind) => {
            let piece = Piece { color: board.get(mv.from.0, mv.from.1).unwrap().color, kind: piece_kind, moves_count: 0 };
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
        },
        MoveType::PromotionCapture(piece_kind) => {
            let piece = Piece { color: board.get(mv.from.0, mv.from.1).unwrap().color, kind: piece_kind, moves_count: 0 };
            board.set(mv.from.0, mv.from.1, None);
            board.set(mv.to.0, mv.to.1, Some(piece));
        }
    };
    // update piece moves_count
    increment_piece_move_count(&mut board, mv.from);
    board
}
