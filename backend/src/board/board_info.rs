use crate::board::piece::{PieceKind};
use crate::board::{Board, Square};
use crate::game::player::Color;
use crate::rules::r#move::{Move, MoveType};

pub type Bitboard = u64;
pub type Position = (u8, u8);

#[derive(Clone)]
pub struct BoardInfo {
    pub squares: [Square; 64], // Array of 64 Option<Piece> values
    pub piece_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece type
    pub player_bitboards: [Bitboard; 2], // Array of 2 Bitboards, one for each player
    pub all_pieces_bitboard: Bitboard, // Bitboard of all pieces

    pub piece_capture_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece that can be captured
    pub color_capture_bitboards: [Bitboard; 2], // Array of 2 Bitboards, one for each player whose pieces can be captured

    pub piece_move_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece's moves
    pub color_move_bitboards: [Bitboard; 2], // Array of 2 Bitboards, one for each player's moves

    pub white_king_pos: Position,
    pub black_king_pos: Position,

    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
}

impl BoardInfo {
    pub fn new(squares: [Square; 64]) -> Self {
        Self {
            squares,
            piece_bitboards: [0; 12],
            player_bitboards: [0; 2],
            all_pieces_bitboard: 0,

            piece_capture_bitboards: [0; 12],
            color_capture_bitboards: [0; 2],

            piece_move_bitboards: [0; 12],
            color_move_bitboards: [0; 2],

            white_king_pos: (0, 0),
            black_king_pos: (0, 0),

            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
        }
    }

    pub fn reset_bitboards(&mut self) {
        self.piece_bitboards = [0; 12];
        self.player_bitboards = [0; 2];
        self.all_pieces_bitboard = 0;

        self.piece_capture_bitboards = [0; 12];
        self.color_capture_bitboards = [0; 2];

        self.piece_move_bitboards = [0; 12];
        self.color_move_bitboards = [0; 2];
    }

    pub fn update_bitboards(&mut self, squares: [Square; 64]) {
        // Reset bitboards
        self.reset_bitboards();

        // Update bitboards
        for pos in 0..64 {
            if let Some(piece) = squares[pos] {
                let bitboard = 1 << pos;
                self.piece_bitboards[piece_idx(piece.kind, piece.color)] |= bitboard;
                self.color_bitboards[color_idx(piece.color)] |= bitboard;
                self.all_pieces_bitboard |= bitboard;
            }
        }
    }
}

pub fn bb_color_idx(color: Color) -> usize {
    if color == Color::White {
        0
    } else {
        1
    }
}

pub fn bb_piece_idx(kind: PieceKind, color: Color) -> usize {
    let mut idx: usize;
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