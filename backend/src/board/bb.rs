use crate::board::square::Position;
use crate::moves::CastleSide;
use crate::piece::{Color, PieceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The type of bitboard.
/// The bitboards are represented by an array of 18 elements.
/// The first 12 elements represent the bitboards for each piece type.
/// The next 3 elements represent the bitboards for the occupied squares.
/// The last 3 elements represent the bitboards for the attacked squares.
/// The bitboards are represented by a 64 bit unsigned integer.
pub enum BitboardType {
    /// Index 0
    WhitePawns = 0,
    /// Index 1
    WhiteKnights = 1,
    /// Index 2
    WhiteBishops = 2,
    /// Index 3
    WhiteRooks = 3,
    /// Index 4
    WhiteQueens = 4,
    /// Index 5
    WhiteKing = 5,
    /// Index 6
    BlackPawns = 6,
    /// Index 7
    BlackKnights = 7,
    /// Index 8
    BlackBishops = 8,
    /// Index 9
    BlackRooks = 9,
    /// Index 10
    BlackQueens = 10,
    /// Index 11
    BlackKing = 11,
    /// Index 12
    WhiteOccupied = 12,
    /// Index 13
    BlackOccupied = 13,
    /// Index 14
    AllOccupied = 14,
    /// Index 15
    /// Represent en passant target squares.
    EnPassant = 15,
    /// Index 16
    /// Represents the squares attacked by White.
    WhiteAttacked = 16,
    /// Index 17 
    /// Represents the squares attacked by Black.
    BlackAttacked = 17,
}

pub type Bitboard = u64;

#[derive(Debug, Clone)]
pub struct Bitboards {
    /// The bitboards are represented by an array of 18 elements.
    /// The first 12 elements represent the bitboards for each piece type.
    /// The next 3 elements represent the bitboards for the occupied squares.
    /// The last 3 elements represent the bitboards for the attacked squares.
    boards: [Bitboard; 18],
    /// The castling rights are represented by a 4 element array.
    /// The first element represents the castling rights for White King Side.
    /// The second element represents the castling rights for White Queen Side.
    /// The third element represents the castling rights for Black King Side.
    /// The fourth element represents the castling rights for Black Queen Side.
    castling_rights: [bool; 4], // Flags to represent castling rights
}

impl Bitboards {
    /// Creates a new instance of Bitboards.
    ///
    /// # Returns
    /// A new instance of Bitboards.
    pub fn new() -> Self {
        Self {
            boards: [0; 18],
            castling_rights: [true; 4], // Initially, all castling rights are available
        }
    }

    /// Sets a bit at a given position in a the bitboard.
    ///
    /// # Arguments
    /// * `board_type` The type of bitboard to set the bit in.
    /// * `position` The position of the bit to set.
    pub fn set_bit(&mut self, board_type: BitboardType, position: usize) {
        self.boards[board_type as usize] |= 1 << position;
    }

    /// Clears a bit at a given position in a the bitboard.
    ///
    /// # Arguments
    /// * `board_type` The type of bitboard to clear the bit in.
    /// * `position` The position of the bit to clear.
    pub fn clear_bit(&mut self, board_type: BitboardType, position: usize) {
        self.boards[board_type as usize] &= !(1 << position);
    }

    /// Gets a bit at a given position in a the bitboard.
    ///
    /// # Arguments
    /// * `board_type` The type of bitboard to get the bit from.
    /// * `position` The position of the bit to get.
    pub fn get_bit(&self, board_type: BitboardType, position: usize) -> bool {
        (self.boards[board_type as usize] & (1 << position)) != 0
    }

    /// Initializes the bitboards from a FEN string.
    ///
    /// # Arguments
    /// * `fen` The FEN string to initialize the bitboards from.
    ///
    /// # Example
    /// ```rs
    ///    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    ///    let mut board = Board::new_standard();
    ///    board.bb.initialize_from_fen(fen);
    /// ```
    pub fn initialize_from_fen(&mut self, fen: &str) {
        let mut fen_split = fen.split_whitespace();
        let mut rank = 7;
        let mut file = 0;
        let mut char_iter = fen_split.next().unwrap().chars();
        let piece_kind_from_symbol = [
            ('P', PieceKind::Pawn),
            ('N', PieceKind::Knight),
            ('B', PieceKind::Bishop),
            ('R', PieceKind::Rook),
            ('Q', PieceKind::Queen),
            ('K', PieceKind::King),
        ];
        while let Some(c) = char_iter.next() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap();
            } else {
                let mut piece_color = Color::White;
                if c.is_lowercase() {
                    piece_color = Color::Black;
                }
                let mut piece_kind = PieceKind::None;
                for (piece_symbol, piece_kind_) in piece_kind_from_symbol.iter() {
                    if c.to_ascii_uppercase() == *piece_symbol {
                        piece_kind = *piece_kind_;
                    }
                }
                let pos: Position = rank * 8usize + file as usize;
                let board_type = match piece_color {
                    Color::White => match piece_kind {
                        PieceKind::Pawn => BitboardType::WhitePawns,
                        PieceKind::Knight => BitboardType::WhiteKnights,
                        PieceKind::Bishop => BitboardType::WhiteBishops,
                        PieceKind::Rook => BitboardType::WhiteRooks,
                        PieceKind::Queen => BitboardType::WhiteQueens,
                        PieceKind::King => BitboardType::WhiteKing,
                        _ => BitboardType::WhiteOccupied,
                    },
                    Color::Black => match piece_kind {
                        PieceKind::Pawn => BitboardType::BlackPawns,
                        PieceKind::Knight => BitboardType::BlackKnights,
                        PieceKind::Bishop => BitboardType::BlackBishops,
                        PieceKind::Rook => BitboardType::BlackRooks,
                        PieceKind::Queen => BitboardType::BlackQueens,
                        PieceKind::King => BitboardType::BlackKing,
                        _ => BitboardType::BlackOccupied,
                    },
                };
                self.set_bit(board_type, pos);
                file += 1;
            }
        }
    }

    /// Gets the bitboard for a given piece type.
    ///
    /// # Arguments
    /// * `piece_type` The type of piece to get the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given piece type.
    pub fn get_board(&self, board_type: BitboardType) -> Bitboard {
        self.boards[board_type as usize]
    }

    /// Checks if the given color can castle on the given side.
    ///
    /// # Arguments
    /// * `side` The side to check if the color can castle on.
    /// * `color` The color to check if it can castle on the given side.
    ///
    /// # Returns
    /// True if the color can castle on the given side, false otherwise.
    pub fn can_castle(&self, side: CastleSide, color: Color) -> bool {
        self.castling_rights[(side as usize) + (color as usize / 8)]
    }

    /// Sets the castling rights for a given side.
    ///
    /// # Arguments
    /// * `side` The side to set the castling rights for.
    /// * `color` The color to set the castling rights for.
    /// * `value` The value to set the castling rights to.
    pub fn set_castling_rights(&mut self, side: CastleSide, color: Color, value: bool) {
        self.castling_rights[(side as usize) + (color as usize / 8)] = value;
    }
}
