use crate::board::square::{Position, Square};
use crate::moves::{CastleSide, Move, SimpleMoves};
use crate::piece::{Color, PieceAsByte, PieceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The type of bitboard.
/// The bitboards are represented by an array of 18 elements.
/// The first 12 elements represent the bitboards for each piece type.
/// The next 3 elements represent the bitboards for the occupied squares.
/// The last 3 elements represent the bitboards for the attacked squares.
/// The bitboards are represented by a 64 bit unsigned integer.
pub enum BitboardType {
    /// Index 0
    WhitePawns,
    /// Index 1
    WhiteKnights,
    /// Index 2
    WhiteBishops,
    /// Index 3
    WhiteRooks,
    /// Index 4
    WhiteQueens,
    /// Index 5
    WhiteKing,
    /// Index 6
    BlackPawns,
    /// Index 7
    BlackKnights,
    /// Index 8
    BlackBishops,
    /// Index 9
    BlackRooks,
    /// Index 10
    BlackQueens,
    /// Index 11
    BlackKing,
    /// Index 12
    WhiteOccupied,
    /// Index 13
    BlackOccupied,
    /// Index 14
    AllOccupied,
    /// Index 16
    /// Represents the squares attacked by White.
    WhiteAttacking,
    /// Index 17
    /// Represents the squares attacked by Black.
    BlackAttacking,
    /// Index 18
    /// Represents the squares attacked by both White and Black.
    AllAttacked,
}

impl BitboardType {
    pub fn from_piece(piece: PieceAsByte) -> BitboardType {
        // 8 is white, 16 is black
        // Piece kinds are 0 to 6 (None, King, Pawn, Knight, Bishop, Rook, Queen)
        match piece & 7 {
            1 => match piece & 24 {
                8 => BitboardType::WhiteKing,
                16 => BitboardType::BlackKing,
                _ => panic!("Invalid piece."),
            },
            2 => match piece & 24 {
                8 => BitboardType::WhitePawns,
                16 => BitboardType::BlackPawns,
                _ => panic!("Invalid piece."),
            },
            3 => match piece & 24 {
                8 => BitboardType::WhiteKnights,
                16 => BitboardType::BlackKnights,
                _ => panic!("Invalid piece."),
            },
            4 => match piece & 24 {
                8 => BitboardType::WhiteBishops,
                16 => BitboardType::BlackBishops,
                _ => panic!("Invalid piece."),
            },
            5 => match piece & 24 {
                8 => BitboardType::WhiteRooks,
                16 => BitboardType::BlackRooks,
                _ => panic!("Invalid piece."),
            },
            6 => match piece & 24 {
                8 => BitboardType::WhiteQueens,
                16 => BitboardType::BlackQueens,
                _ => panic!("Invalid piece."),
            },
            _ => panic!("Invalid piece."),
        }
    }

    pub fn from_color(color: Color, attacking: bool) -> BitboardType {
        match color {
            Color::White => {
                if attacking {
                    BitboardType::WhiteAttacking
                } else {
                    BitboardType::WhiteOccupied
                }
            }
            Color::Black => {
                if attacking {
                    BitboardType::BlackAttacking
                } else {
                    BitboardType::BlackOccupied
                }
            }
        }
    }
}

pub type Bitboard = u64;

#[derive(Debug, Clone)]
pub struct Bitboards {
    /// Represents the bitboards for each piece type.
    /// * 0: None
    /// * 1: King
    /// * 2: Pawn
    /// * 3: Knight
    /// * 4: Bishop
    /// * 5: Rook
    /// * 6: Queen
    pub piece_bitboards: [Bitboard; 7],
    /// Represents the bitboards for the occupied squares.
    /// * 0: All Occupied
    /// * 1: White Occupied
    /// * 2: Black Occupied
    pub occupied_bitboards: [Bitboard; 3],

    /// Represents the bitboards for the attacked squares.
    /// * 0: White Attacked
    /// * 1: Black Attacked
    pub attacked_bitboards: [Bitboard; 2],

    /// The castling rights are represented by a 4 element array.
    /// Each element represents the castling rights for a given side.
    /// * 0: White King Side
    /// * 1: White Queen Side
    /// * 2: Black King Side
    /// * 3: Black Queen Side
    pub castling_rights: [bool; 4],
}

impl Bitboards {
    /// Creates a new Bitboards struct.
    ///
    /// # Returns
    /// A new Bitboards struct.
    pub fn new() -> Bitboards {
        Bitboards {
            piece_bitboards: [0; 7],
            occupied_bitboards: [0; 3],
            attacked_bitboards: [0; 2],
            castling_rights: [false; 4],
        }
    }

    /// Returns the bitboard for a given bitboard type.
    ///
    /// # Arguments
    /// * `bitboard_type` The bitboard type to get the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given bitboard type.
    pub fn get_bitboard(&self, bitboard_type: BitboardType) -> Bitboard {
        match bitboard_type {
            BitboardType::WhitePawns => {
                self.piece_bitboards[PieceKind::Pawn as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::WhiteKnights => {
                self.piece_bitboards[PieceKind::Knight as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::WhiteBishops => {
                self.piece_bitboards[PieceKind::Bishop as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::WhiteRooks => {
                self.piece_bitboards[PieceKind::Rook as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::WhiteQueens => {
                self.piece_bitboards[PieceKind::Queen as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::WhiteKing => {
                self.piece_bitboards[PieceKind::King as usize]
                    | self.occupied_bitboards[Color::White as usize / 8]
            }
            BitboardType::BlackPawns => {
                self.piece_bitboards[PieceKind::Pawn as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::BlackKnights => {
                self.piece_bitboards[PieceKind::Knight as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::BlackBishops => {
                self.piece_bitboards[PieceKind::Bishop as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::BlackRooks => {
                self.piece_bitboards[PieceKind::Rook as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::BlackQueens => {
                self.piece_bitboards[PieceKind::Queen as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::BlackKing => {
                self.piece_bitboards[PieceKind::King as usize]
                    | self.occupied_bitboards[Color::Black as usize / 8]
            }
            BitboardType::WhiteOccupied => self.occupied_bitboards[1],
            BitboardType::BlackOccupied => self.occupied_bitboards[2],
            BitboardType::AllOccupied => self.occupied_bitboards[0],
            BitboardType::WhiteAttacking => self.attacked_bitboards[0],
            BitboardType::BlackAttacking => self.attacked_bitboards[1],
            BitboardType::AllAttacked => self.attacked_bitboards[0] | self.attacked_bitboards[1],
        }
    }

    /// Sets the given bitboard to the given bitboard type.
    pub fn set_bitboard(&mut self, bitboard_type: BitboardType, bitboard: Bitboard) {
        match bitboard_type {
            BitboardType::WhitePawns => {
                self.piece_bitboards[PieceKind::Pawn as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            },
            BitboardType::WhiteKnights => {
                self.piece_bitboards[PieceKind::Knight as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            }
            BitboardType::WhiteBishops => {
                self.piece_bitboards[PieceKind::Bishop as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            }
            BitboardType::WhiteRooks => {
                self.piece_bitboards[PieceKind::Rook as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            },
            BitboardType::WhiteQueens => {
                self.piece_bitboards[PieceKind::Queen as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            },
            BitboardType::WhiteKing => {
                self.piece_bitboards[PieceKind::King as usize] = bitboard;
                self.occupied_bitboards[Color::White as usize/ 8] = bitboard;
            },
            BitboardType::BlackPawns => {
                self.piece_bitboards[PieceKind::Pawn as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            },
            BitboardType::BlackKnights => {
                self.piece_bitboards[PieceKind::Knight as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            }
            BitboardType::BlackBishops => {
                self.piece_bitboards[PieceKind::Bishop as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            }
            BitboardType::BlackRooks => {
                self.piece_bitboards[PieceKind::Rook as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            },
            BitboardType::BlackQueens => {
                self.piece_bitboards[PieceKind::Queen as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            },
            BitboardType::BlackKing => {
                self.piece_bitboards[PieceKind::King as usize] = bitboard;
                self.occupied_bitboards[Color::Black as usize/ 8] = bitboard;
            },
            BitboardType::WhiteOccupied => {
                self.occupied_bitboards[Color::White as usize / 8] = bitboard;
            }
            BitboardType::BlackOccupied => {
                self.occupied_bitboards[Color::Black as usize / 8] = bitboard;
            }
            BitboardType::AllOccupied => {
                self.occupied_bitboards[0] = bitboard;
            }
            BitboardType::WhiteAttacking => self.attacked_bitboards[0] = bitboard,
            BitboardType::BlackAttacking => self.attacked_bitboards[1] = bitboard,
            BitboardType::AllAttacked => {
                self.attacked_bitboards[0] = bitboard;
                self.attacked_bitboards[1] = bitboard;
            }
        }
    }

    /// Sets a bit in the given bitboard type.
    ///
    /// # Arguments
    /// * `bitboard_type` The bitboard type to set the bit in.
    /// * `position` The position of the bit to set.
    pub fn set_bit(&mut self, bitboard_type: BitboardType, position: Position) {
        let bitboard = self.get_bitboard(bitboard_type);
        self.set_bitboard(bitboard_type, bitboard | (1 << position));
    }

    /// Clears a bit in the given bitboard type.
    ///
    /// # Arguments
    /// * `bitboard_type` The bitboard type to clear the bit in.
    /// * `position` The position of the bit to clear.
    pub fn clear_bit(&mut self, bitboard_type: BitboardType, position: Position) {
        let bitboard = self.get_bitboard(bitboard_type);
        self.set_bitboard(bitboard_type, bitboard & !(1 << position));
    }

    /// Gets the bitboard for a given piece kind.
    ///
    /// # Arguments
    /// * `piece_kind` The piece kind to get the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given piece kind.
    pub fn get_piece_bitboard(&self, piece_kind: PieceKind) -> Bitboard {
        self.piece_bitboards[piece_kind as usize]
    }

    /// Gets the bitboard for a given color.
    ///
    ///
    /// # Arguments
    /// * `color` The color to get the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given color.
    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        self.occupied_bitboards[color as usize / 8]
    }

    /// Gets the unoccupied squares bitboard.
    ///
    /// # Returns
    /// The unoccupied squares bitboard.
    pub fn get_unoccupied_squares_bitboard(&self) -> Bitboard {
        self.piece_bitboards[0]
    }

    /// Gets the attacked by squares bitboard for a given color.
    ///
    /// # Arguments
    /// * `color` The color to get the attacked by squares bitboard for.
    ///
    /// # Returns
    /// The attacked by squares bitboard for the given color.
    pub fn get_attacked_by_bitboard(&self, color: Color) -> Bitboard {
        self.attacked_bitboards[color as usize / 8]
    }

    /// Gets the pieces that could be captured by a given color.
    ///
    /// # Arguments
    /// * `color` The color to get the attacked pieces for.
    ///
    /// # Returns
    /// The attacked pieces for the given color.
    pub fn get_attacked_pieces(&self, color: Color) -> Bitboard {
        self.get_attacked_by_bitboard(color) & self.get_color_bitboard(color)
    }

    /// Gets the empty attacked squares for a given color.
    ///
    /// # Arguments
    /// * `color` The color that is attacking.
    ///
    /// # Returns
    /// The empty attacked squares for the given color.
    pub fn get_empty_attacked_squares(&self, color: Color) -> Bitboard {
        !self.occupied_bitboards[0] & self.get_attacked_by_bitboard(color)
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

    /// Sets the castling rights for a given side.
    ///
    /// # Arguments
    /// * `side` The side to set the castling rights for.
    /// * `color` The color to set the castling rights for.
    /// * `value` The value to set the castling rights to.
    pub fn set_castling_rights(&mut self, side: CastleSide, color: Color, value: bool) {
        self.castling_rights[(side as usize) + (color as usize / 8)] = value;
    }

    /// Returns the bitboard for a given position.
    ///
    /// # Arguments
    /// * `pos` The position to get the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given position.
    pub fn from_pos(pos: Position) -> Bitboard {
        1 << pos
    }

    /// Generates a bitboard for every move in a given move list.
    ///
    /// # Arguments
    /// * `moves` The move list to generate the bitboard for.
    ///
    /// # Returns
    /// The bitboard for the given move list.
    pub fn from_simple_move_list(moves: SimpleMoves) -> Bitboard {
        let mut bitboard = 0;
        for (_, to) in moves {
            bitboard |= Bitboards::from_pos(to);
        }
        bitboard
    }

    /// Returns the positions for a given bitboard.
    ///
    /// # Arguments
    /// * `bitboard` The bitboard to get the positions for.
    ///
    /// # Returns
    /// The positions for the given bitboard.
    pub fn to_pos(bitboard: Bitboard) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut bitboard = bitboard;
        while bitboard != 0 {
            let pos = bitboard.trailing_zeros() as Position;
            positions.push(pos);
            bitboard &= bitboard - 1;
        }
        positions
    }

    pub fn update(&mut self, squares: &[Square]) {
        self.reset();
        for square in squares {
            let pos = square.position;
            let pos_bitboard = Bitboards::from_pos(pos);
            if square.is_occupied() {
                let piece = square.piece.to_byte();
                let color = square.piece.color.unwrap();

                let piece_bitboard_type = BitboardType::from_piece(piece);
                let color_bitboard_type = BitboardType::from_color(color, false);
                let piece_bitboard = self.get_bitboard(piece_bitboard_type);
                let color_bitboard = self.get_bitboard(color_bitboard_type);
                self.set_bitboard(piece_bitboard_type, piece_bitboard | pos_bitboard);
                self.set_bitboard(color_bitboard_type, color_bitboard | pos_bitboard);

                let attacked_bitboard_type = BitboardType::from_color(color, true);

                if square.is_attacked {
                    self.set_bit(attacked_bitboard_type, pos);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.piece_bitboards = [0; 7];
        self.occupied_bitboards = [0; 3];
        self.attacked_bitboards = [0; 2];
        self.castling_rights = [false; 4];
    }
}
