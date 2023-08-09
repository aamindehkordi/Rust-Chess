use crate::board::piece::{get_moves, Piece, PieceKind};
use crate::board::{idx, Square};
use crate::game::player::Color;
use crate::rules::r#move::Move;

pub type Bitboard = u64;
pub type Position = (u8, u8);

#[derive(Clone)]
pub struct BoardInfo {
    pub squares: [Square; 64],           // Array of 64 Option<Piece> values
    pub piece_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece type
    pub player_bitboards: [Bitboard; 2], // Array of 2 Bitboards, one for each player
    pub all_pieces_bitboard: Bitboard,   // Bitboard of all pieces

    pub piece_capture_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece that can be captured
    pub color_capture_bitboards: [Bitboard; 2], // Array of 2 Bitboards, one for each player whose pieces can be captured

    pub piece_move_bitboards: [Bitboard; 12], // Array of 12 Bitboards, one for each piece's moves
    pub color_move_bitboards: [Bitboard; 2],  // Array of 2 Bitboards, one for each player's moves

    pub white_king_pos: Position,
    pub black_king_pos: Position,

    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,

    pub move_history: Vec<Move>,
    pub captured_pieces: Vec<Piece>,
    pub white_psuedo_moves: Vec<Move>,
    pub black_psuedo_moves: Vec<Move>,
    pub valid_moves: Vec<Move>,

    pub turn: u8,
}

impl BoardInfo {
    /**

    * Creates a new instance of the Chessboard struct.

    *

    * This function takes an array of `Square` values and initializes all the fields of the Chessboard struct.

    * The piece_bitboards, player_bitboards, and color_capture_bitboards are initialized with zeros.

    * The all_pieces_bitboard, piece_capture_bitboards and color_move_bitboards are also initialized with zeros.

    * The king positions are initialized to (0, 0).

    * The can_castle_* fields are initialized to false.

    * The move_history, captured_pieces, white_psuedo_moves, black_psuedo_moves, and valid_moves vectors are initialized as empty.

    *

    * @param squares - An array containing `Square` values representing the initial configuration of the chessboard.

    * @return A new instance of the Chessboard struct.

    */
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

            white_can_castle_kingside: false,
            white_can_castle_queenside: false,
            black_can_castle_kingside: false,
            black_can_castle_queenside: false,

            move_history: Vec::new(),
            captured_pieces: Vec::new(),
            white_psuedo_moves: Vec::new(),
            black_psuedo_moves: Vec::new(),
            valid_moves: Vec::new(),

            turn: 0,
        }
    }

    // Getters and setters
    // -------------------
    /**
     * Sets the square on the chessboard at the specified position.
     *
     * This function updates the square at the specified position with the given square value.
     *
     * @param position - The position of the square to be set.
     * @param square - The square value to set at the specified position.
     */
    pub fn set_square(&mut self, position: Position, square: Square) {
        self.squares[idx(position)] = square;
    }

    /**
     * Returns the square at the specified position.
     *
     * This function retrieves the square at the given position from the Chessboard struct.
     *
     * @param position - The position of the square to fetch.
     * @return The square at the specified position.
     */
    pub fn get_square(&self, position: Position) -> Square {
        self.squares[idx(position)]
    }

    /**
     * Sets the specified piece at the given position on the chessboard.
     *
     * This function updates the square at the specified position with the piece provided.
     *
     * @param position - The position on the chessboard where the piece is to be set.
     * @param piece - The piece to set at the specified position.
     */
    pub fn set_piece(&mut self, position: Position, piece: Piece) {
        self.set_square(position, Some(piece));
    }

    /**
     * Retrieves the piece at the given position on the chessboard.
     *
     * This function returns an Option containing the piece at the specified position, if there is a piece on that square.
     * If there is no piece at the specified position, it returns None.
     *
     * @param position - The position to retrieve the piece from.
     * @return An Option containing the piece at the specified position, or None if there is no piece.
     */
    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.get_square(position)
    }

    /**
     * Removes a chess piece from the specified position on the chessboard.
     *
     * This function updates the square at the specified position to contain no piece.
     *
     * @param position - The position from which to remove the chess piece.
     */
    pub fn remove_piece(&mut self, position: Position) {
        self.set_square(position, None);
    }

    /**
     * Gets the bitboard for the specified piece and player color.
     *
     * This function returns the bitboard corresponding to the specified piece and player color
     * from the Chessboard struct.
     *
     * @param piece - The piece kind for which the bitboard is to be retrieved.
     * @param color - The player color for which the bitboard is to be retrieved.
     *
     * @return The bitboard corresponding to the specified piece and player color.
     */
    pub fn get_piece_bitboard(&self, piece: PieceKind, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(piece, color)]
    }

    /**
     * Sets the bitboard for the given piece and player color.
     *
     * This function updates the bitboard for the specified piece and player color in the Chessboard struct.
     *
     * @param piece - The kind of piece whose bitboard is to be set.
     * @param color - The player color whose bitboard is to be set.
     * @param bitboard - The bitboard to set for the specified piece and player color.
     */
    pub fn set_piece_bitboard(&mut self, piece: PieceKind, color: Color, bitboard: Bitboard) {
        self.piece_bitboards[bb_piece_idx(piece, color)] = bitboard;
    }

    /**
     * Returns the bitboard for the given player color.
     *
     * This function retrieves and returns the bitboard for the specified player color from the Chessboard struct.
     *
     * @param color - The player color whose bitboard is to be retrieved.
     * @return The bitboard for the specified player color.
     */
    pub fn get_player_bitboard(&self, color: Color) -> Bitboard {
        self.player_bitboards[bb_color_idx(color)]
    }

    /**
     * Sets the bitboard for the given player color.
     *
     * This function updates the bitboard for the specified player color in the Chessboard struct.
     *
     * @param color - The player color whose bitboard is to be set.
     * @param bitboard - The bitboard to set for the specified player color.
     */
    pub fn set_player_bitboard(&mut self, color: Color, bitboard: Bitboard) {
        self.player_bitboards[bb_color_idx(color)] = bitboard;
    }

    /**
     * Retrieves the bitboard representing all the pieces on the chessboard.
     *
     * This function retrieves the bitboard that represents the positions of all the pieces on the chessboard.
     *
     * @return The bitboard representing all the pieces on the chessboard.
     */
    pub fn get_all_pieces_bitboard(&self) -> Bitboard {
        self.all_pieces_bitboard
    }

    /**
     * Sets the bitboard for all pieces.
     *
     * This function updates the bitboard that represents all the pieces on the chessboard.
     *
     * @param bitboard - The bitboard to set for all pieces.
     */
    pub fn set_all_pieces_bitboard(&mut self, bitboard: Bitboard) {
        self.all_pieces_bitboard = bitboard;
    }

    /**
     * Gets the capture bitboard for the specified piece and color.
     *
     * This function retrieves the capture bitboard for the specified piece kind and player color
     * from the Chessboard struct.
     *
     * @param piece - The piece kind whose capture bitboard is to be retrieved.
     * @param color - The player color whose capture bitboard is to be retrieved.
     * @return The capture bitboard for the specified piece and color.
     */
    pub fn get_piece_capture_bitboard(&self, piece: PieceKind, color: Color) -> Bitboard {
        self.piece_capture_bitboards[bb_piece_idx(piece, color)]
    }

    /**
     * Sets the capture bitboard for a specific piece and player color.
     *
     * This function updates the capture bitboard for the specified piece and player color in the Chessboard struct.
     *
     * @param piece - The type of piece whose capture bitboard is to be set.
     * @param color - The player color for which the capture bitboard is to be set.
     * @param bitboard - The capture bitboard to set for the specified piece and player color.
     */
    pub fn set_piece_capture_bitboard(
        &mut self,
        piece: PieceKind,
        color: Color,
        bitboard: Bitboard,
    ) {
        self.piece_capture_bitboards[bb_piece_idx(piece, color)] = bitboard;
    }

    /**
     * Retrieves the capture bitboard for the specified player color.
     *
     * This function returns the capture bitboard associated with the specified player color
     * from the Chessboard struct.
     *
     * @param color - The player color whose capture bitboard is to be retrieved.
     * @return The capture bitboard for the specified player color.
     */
    pub fn get_color_capture_bitboard(&self, color: Color) -> Bitboard {
        self.color_capture_bitboards[bb_color_idx(color)]
    }

    /**
     * Sets the capture bitboard for the given color.
     *
     * This function updates the capture bitboard for the specified color in the Chessboard struct.
     *
     * @param color - The color whose capture bitboard is to be set.
     * @param bitboard - The bitboard to set for the specified color.
     */
    pub fn set_color_capture_bitboard(&mut self, color: Color, bitboard: Bitboard) {
        self.color_capture_bitboards[bb_color_idx(color)] = bitboard;
    }

    /**
     * Retrieves the bitboard containing possible moves for the specified piece of the given color.
     *
     * This function returns the bitboard that represents the possible moves for a specific piece
     * of the specified player color in the Chessboard struct.
     *
     * @param piece - The type of the piece for which the move bitboard is to be retrieved.
     * @param color - The color of the player whose piece move bitboard is to be retrieved.
     * @return The bitboard containing possible moves for the specified piece and color.
     */
    pub fn get_piece_move_bitboard(&self, piece: PieceKind, color: Color) -> Bitboard {
        self.piece_move_bitboards[bb_piece_idx(piece, color)]
    }

    /**
     * Sets the move bitboard for the specified piece and player color.
     *
     * This function updates the move bitboard for the specified piece and player color in the Chessboard struct.
     *
     * @param piece - The piece kind whose move bitboard is to be set.
     * @param color - The player color whose move bitboard is to be set.
     * @param bitboard - The move bitboard to set for the specified piece and player color.
     */
    pub fn set_piece_move_bitboard(&mut self, piece: PieceKind, color: Color, bitboard: Bitboard) {
        self.piece_move_bitboards[bb_piece_idx(piece, color)] = bitboard;
    }

    /**
     * Retrieves the move bitboard for the specified player color.
     *
     * This function returns the move bitboard associated with the specified player color from the Chessboard struct.
     *
     * @param color - The player color for which to retrieve the move bitboard.
     * @return The move bitboard for the specified player color.
     */
    pub fn get_color_move_bitboard(&self, color: Color) -> Bitboard {
        self.color_move_bitboards[bb_color_idx(color)]
    }

    /**
     * Sets the move bitboard for the given color.
     *
     * This function updates the move bitboard for the specified player color in the Chessboard struct.
     *
     * @param color - The color whose move bitboard is to be set.
     * @param bitboard - The move bitboard to set for the specified color.
     */
    pub fn set_color_move_bitboard(&mut self, color: Color, bitboard: Bitboard) {
        self.color_move_bitboards[bb_color_idx(color)] = bitboard;
    }

    /**
     * Retrieves the bitboard representing the kings of the specified color.
     *
     * This function returns the bitboard that represents the kings of the specified player color in the Chessboard struct.
     *
     * @param color - The player color whose kings bitboard is to be retrieved.
     * @return The bitboard representing the kings of the specified color.
     */
    pub fn king(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::King, color)]
    }

    /**
     * Returns the bitboard representing the queens of the specified color.
     *
     * This function retrieves the bitboard representing the queens of the specified player color from the Chessboard struct.
     *
     * @param color - The player color whose queens are to be retrieved.
     * @return The bitboard representing the queens of the specified color.
     */
    pub fn queen(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Queen, color)]
    }

    /**
     * Retrieves the bitboard for rooks of the specified color.
     *
     * This function returns the bitboard representing the rooks of the specified color in the Chessboard struct.
     *
     * @param color - The player color for which to retrieve the rook bitboard.
     * @return The bitboard representing the rooks of the specified color.
     */
    pub fn rook(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Rook, color)]
    }

    /**
     * Retrieves the bitboard representing all the bishops of the specified color.
     *
     * This function returns the bitboard representing the bishops of the specified color.
     *
     * @param color - The color of the bishops to retrieve.
     * @return The bitboard representing the bishops of the specified color.
     */
    pub fn bishop(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Bishop, color)]
    }

    /**
     * Retrieves the bitboard representing all the knights for the given player color.
     *
     * This function returns the bitboard representing the current positions of all the knights for the specified player color.
     *
     * @param color - The player color whose knight positions are to be retrieved.
     * @return The bitboard representing the positions of the knights for the specified player color.
     */
    pub fn knight(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Knight, color)]
    }

    /**
     * Retrieves the bitboard representing the pawns of the specified color.
     *
     * This function returns the bitboard representing all pawns of the given color in the Chessboard struct.
     *
     * @param color - The color of the pawns to retrieve.
     * @return The bitboard representing the pawns of the specified color.
     */
    pub fn pawn(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Pawn, color)]
    }

    // -------------------

    /**
     * Resets all bitboards in the Chessboard struct to their initial state.
     *
     * This function resets all bitboards in the Chessboard struct, including piece bitboards,
     * player bitboards, all pieces bitboard, piece capture bitboards, color capture bitboards,
     * piece move bitboards, and color move bitboards.
     *
     * Note: After calling this function, all bitboards will be set to 0.
     */
    pub fn reset_bitboards(&mut self) {
        self.piece_bitboards = [0; 12];
        self.player_bitboards = [0; 2];
        self.all_pieces_bitboard = 0;

        self.piece_capture_bitboards = [0; 12];
        self.color_capture_bitboards = [0; 2];

        self.piece_move_bitboards = [0; 12];
        self.color_move_bitboards = [0; 2];
    }

    /**
     * Retrieves the position of the king for the specified player color.
     *
     * This function returns the position of the king for the specified player color in the Chessboard struct.
     *
     * @param color - The player color whose king position is to be retrieved.
     * @return The position of the king for the specified player color.
     */
    pub fn king_pos(&self, color: Color) -> Position {
        if color == Color::White {
            self.white_king_pos
        } else {
            self.black_king_pos
        }
    }

    /**
     * Checks if the current player of the specified color is in check.
     *
     * This function determines if the specified color's player is in check by checking if the
     * king's position intersects with any of the enemy player's possible moves.
     *
     * @param color - The player color to check for check.
     * @return true if the player is in check, false otherwise.
     */
    pub fn is_in_check(&self, color: Color) -> bool {
        let enemy_color = color.other();
        let king = self.king(color);
        let enemy_moves = self.color_move_bitboards[bb_color_idx(enemy_color)];
        (king & enemy_moves) != 0
    }

    /**
     * Checks if the specified position on the chessboard is attacked by the given player color.
     *
     * This function determines if the position is targeted by any of the moves of the opposing player's pieces.
     *
     * @param pos - The position to check for attack.
     * @param color - The player color whose moves are to be considered.
     * @return true if the position is attacked by the given player color, false otherwise.
     */
    pub fn is_attacked(&self, pos: Position, color: Color) -> bool {
        let enemy_color = color.other();
        let enemy_moves = self.color_move_bitboards[bb_color_idx(enemy_color)];
        (pos_to_bb(pos) & enemy_moves) != 0
    }

}

/**
 * Returns the index corresponding to the given player color.
 *
 * This function maps the player color to its corresponding index in the Chessboard struct's player bitboards array.
 *
 * @param color - The player color.
 * @return The index corresponding to the given player color.
 */
pub fn bb_color_idx(color: Color) -> usize {
    if color == Color::White {
        0
    } else {
        1
    }
}

/**
 * Returns the index of a piece kind for the specified color.
 *
 * This function returns the index of a piece kind (e.g., Pawn, Knight, etc.) for the specified player color.
 * The index is used for indexing into arrays or vectors to access corresponding information related to the piece kind.
 *
 * @param kind - The piece kind to get the index for.
 * @param color - The player color to get the index for.
 * @returns The index of the piece kind for the specified player color.
 */
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

/**
 * Creates a bitboard with only a single bit set at the specified index.
 *
 * This function returns a new bitboard with only one bit set, which is at the provided index.
 *
 * @param idx - The index at which the bit is set in the returned bitboard.
 * @returns The new bitboard with only one bit set at the specified index.
 */
pub fn bb(idx: usize) -> Bitboard {
    1 << idx
}

/**
 * Converts a position (x, y) to a corresponding bitboard representation.
 *
 * This function takes a position on the chessboard and converts it into the corresponding
 * bitboard representation. The resulting bitboard will have only one bit set, representing
 * the position on the chessboard.
 *
 * @param pos - The position (x, y) to be converted.
 * @return The bitboard representation of the given position.
 */
pub fn pos_to_bb(pos: Position) -> Bitboard {
    let (x, y) = pos;
    1 << (x + y * 8)
}

/**
 * Updates the board information for the current board state.
 *
 * This function updates the board information for the current board state. It updates the bitboards
 * for the current player's pieces, the enemy player's pieces, and the current player's king position.
 * It also updates the current player's psuedo moves.
 *
 * @param board_info - The board information to be updated.
 * @param squares - The squares of the chessboard.
 * @return The updated board information.
 */
pub fn update_board_info(board_info: BoardInfo, squares: [Option<Piece>; 64]) -> BoardInfo {
    let mut board_info = board_info;
    let mut white_psuedo_moves: Vec<Move> = Vec::new();
    let mut black_psuedo_moves: Vec<Move> = Vec::new();

    for (i, square) in squares.iter().enumerate() {
        if let Some(piece) = square {
            update_bitboards(&mut board_info, piece, i);

            let moves = get_moves(&board_info, piece);
            if piece.color == Color::White {
                white_psuedo_moves.extend(moves);
            } else {
                black_psuedo_moves.extend(moves);
            }
        }
    }

    board_info.white_psuedo_moves = white_psuedo_moves;
    board_info.black_psuedo_moves = black_psuedo_moves;

    board_info.turn ^= 1;
    board_info
}

/**
 * Update the bitboards in the board information based on the given piece and position.
 *
 * This function updates the relevant bitboards in the BoardInfo struct based on the given piece
 * and its position on the chessboard.
 *
 * @param board_info - A mutable reference to the BoardInfo struct to be updated.
 * @param piece - The piece to be considered for updating the bitboards.
 * @param position - The position of the piece on the chessboard.
 */
pub fn update_bitboards(board_info: &mut BoardInfo, piece: &Piece, position: usize) {
    let kind = piece.kind;
    let color = piece.color;

    let bitboard = pos_to_bb(((position % 8) as u8, (position / 8) as u8));
    board_info.piece_bitboards[bb_piece_idx(kind, color)] |= bitboard;
    board_info.player_bitboards[bb_color_idx(color)] |= bitboard;
    board_info.all_pieces_bitboard |= bitboard;
}
