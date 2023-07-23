use std::fmt;

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
impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
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

    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => match self.kind {
                PieceKind::Pawn => 'P',
                PieceKind::Rook => 'R',
                PieceKind::Knight => 'N',
                PieceKind::Bishop => 'B',
                PieceKind::Queen => 'Q',
                PieceKind::King => 'K',
            },
            Color::Black => match self.kind {
                PieceKind::Pawn => 'p',
                PieceKind::Rook => 'r',
                PieceKind::Knight => 'n',
                PieceKind::Bishop => 'b',
                PieceKind::Queen => 'q',
                PieceKind::King => 'k',
            },
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(Self { color: Color::White, kind: PieceKind::Pawn, moves_count: 0 }),
            'R' => Some(Self { color: Color::White, kind: PieceKind::Rook, moves_count: 0 }),
            'N' => Some(Self { color: Color::White, kind: PieceKind::Knight, moves_count: 0 }),
            'B' => Some(Self { color: Color::White, kind: PieceKind::Bishop, moves_count: 0 }),
            'Q' => Some(Self { color: Color::White, kind: PieceKind::Queen, moves_count: 0 }),
            'K' => Some(Self { color: Color::White, kind: PieceKind::King, moves_count: 0 }),
            'p' => Some(Self { color: Color::Black, kind: PieceKind::Pawn, moves_count: 0 }),
            'r' => Some(Self { color: Color::Black, kind: PieceKind::Rook, moves_count: 0 }),
            'n' => Some(Self { color: Color::Black, kind: PieceKind::Knight, moves_count: 0 }),
            'b' => Some(Self { color: Color::Black, kind: PieceKind::Bishop, moves_count: 0 }),
            'q' => Some(Self { color: Color::Black, kind: PieceKind::Queen, moves_count: 0 }),
            'k' => Some(Self { color: Color::Black, kind: PieceKind::King, moves_count: 0 }),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board(Vec<Option<Piece>>);

impl Board {

    pub fn new() -> Self {
        Board(vec![None; 8 * 8])
    }

    pub fn new_standard() -> Self {
        let mut board = Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        board
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
                    let n = c as u8 - '0' as u8;
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

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty_count = 0;

        for y in 0..8 {
            for x in 0..8 {
                match self.get(x, y) {
                    Some(piece) => {
                        if empty_count > 0 {
                            fen.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        fen.push(piece.to_char());
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

    pub fn is_tile_empty(&self, xy: (u8, u8)) -> bool {
        self.get(xy.0, xy.1).is_none()
    }

    pub fn is_tile_occupied(&self, xy: (u8, u8)) -> bool {
        self.get(xy.0, xy.1).is_some()
    }

    pub fn find_king(&self, color: Color) -> (u8, u8) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.get(x, y) {
                    if piece.color == color && piece.kind == PieceKind::King {
                        return (x, y);
                    }
                }
            }
        }
        panic!("King not found");
    }

    pub fn find_pieces(&self, color: Color, kind: PieceKind) -> Vec<(u8, u8)> {
        let mut pieces = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.get(x, y) {
                    if piece.color == color && piece.kind == kind {
                        pieces.push((x, y));
                    }
                }
            }
        }
        pieces
    }

}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                match self.get(x, y) {
                    Some(piece) => write!(f, "{}", piece.to_char())?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn in_bounds(pos: (u8, u8)) -> bool {
    pos.0 < 8 && pos.1 < 8
}