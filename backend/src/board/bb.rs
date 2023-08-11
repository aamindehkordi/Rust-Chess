use crate::moves::CastleSide;

pub type Bitboard = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitboardType {
    WhitePawns,
    WhiteKnights,
    WhiteBishops,
    WhiteRooks,
    WhiteQueens,
    WhiteKing,
    BlackPawns,
    BlackKnights,
    BlackBishops,
    BlackRooks,
    BlackQueens,
    BlackKing,
    WhiteOccupied,
    BlackOccupied,
    AllOccupied,
    EnPassant,     // Bitboard to represent en passant target squares
    WhiteAttacked, // Bitboard to represent squares attacked by White
    BlackAttacked, // Bitboard to represent squares attacked by Black
}

#[derive(Debug, Clone)]
pub struct Bitboards {
    boards: [Bitboard; 18],     // Increased size to accommodate new bitboards
    castling_rights: [bool; 4], // Flags to represent castling rights
}

impl Bitboards {
    pub fn new() -> Self {
        Self {
            boards: [0; 18],
            castling_rights: [true; 4], // Initially, all castling rights are available
        }
    }

    pub fn set_bit(&mut self, board_type: BitboardType, position: usize) {
        self.boards[board_type as usize] |= 1 << position;
    }

    pub fn clear_bit(&mut self, board_type: BitboardType, position: usize) {
        self.boards[board_type as usize] &= !(1 << position);
    }

    pub fn get_bit(&self, board_type: BitboardType, position: usize) -> bool {
        (self.boards[board_type as usize] & (1 << position)) != 0
    }

    pub fn initialize_from_fen(&mut self, _fen: &str) {
        // You can implement this function to initialize the bitboards from a FEN string
    }

    pub fn get_board(&self, board_type: BitboardType) -> Bitboard {
        self.boards[board_type as usize]
    }

    // Functions to manage castling rights
    pub fn can_castle(&self, side: CastleSide) -> bool {
        self.castling_rights[side as usize]
    }

    pub fn set_castle(&mut self, side: CastleSide, value: bool) {
        self.castling_rights[side as usize] = value;
    }
}
