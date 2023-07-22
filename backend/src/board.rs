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
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

pub struct Board(Vec<Option<Piece>>);

impl Board {
    pub fn new() -> Self {
        Board(vec![None; 8 * 8])
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
}