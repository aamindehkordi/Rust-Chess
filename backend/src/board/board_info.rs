use crate::board::piece::{color_idx, Piece, piece_idx};
use crate::board::Square;

pub type Bitboard = u64;
pub type Position = (u8, u8);

#[derive(Clone)]
pub struct BoardInfo {
    pub squares: [Square; 64],
    pub piece_bitboards: [Bitboard; 12],
    pub color_bitboards: [Bitboard; 2],
    pub all_pieces_bitboard: Bitboard,

    pub piece_capture_bitboards: [Bitboard; 12],
    pub color_capture_bitboards: [Bitboard; 2],
    pub all_pieces_capture_bitboard: Bitboard,

    pub piece_move_bitboards: [Bitboard; 12],
    pub color_move_bitboards: [Bitboard; 2],
    pub all_pieces_move_bitboard: Bitboard,

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
            color_bitboards: [0; 2],
            all_pieces_bitboard: 0,

            piece_capture_bitboards: [0; 12],
            color_capture_bitboards: [0; 2],
            all_pieces_capture_bitboard: 0,

            piece_move_bitboards: [0; 12],
            color_move_bitboards: [0; 2],
            all_pieces_move_bitboard: 0,

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
        self.color_bitboards = [0; 2];
        self.all_pieces_bitboard = 0;

        self.piece_capture_bitboards = [0; 12];
        self.color_capture_bitboards = [0; 2];
        self.all_pieces_capture_bitboard = 0;

        self.piece_move_bitboards = [0; 12];
        self.color_move_bitboards = [0; 2];
        self.all_pieces_move_bitboard = 0;
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