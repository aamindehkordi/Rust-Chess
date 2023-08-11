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
}

#[derive(Debug, Clone)]
pub struct Bitboards {
    boards: [Bitboard; 15],
}

impl Bitboards {
    pub fn new() -> Self {
        Self { boards: [0; 15] }
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

    pub fn initialize_from_fen(&mut self, fen: &str) {
        // You can implement this function to initialize the bitboards from a FEN string
    }

    pub fn get_board(&self, board_type: BitboardType) -> Bitboard {
        self.boards[board_type as usize]
    }
}
