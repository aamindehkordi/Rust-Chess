use crate::board::board_info::{update_board_info, BoardInfo};
use crate::board::piece::{to_char, Piece, PieceKind};
use crate::game::player::Color;

use crate::rules::r#move::{CastleType, Move, MoveType};

pub mod board_info;
pub mod piece;

pub type Position = (u8, u8);

pub type Square = Option<Piece>;

#[derive(Clone)]
pub struct Board {
    pub squares: [Square; 64],
    pub board_info: BoardInfo,
}

impl Default for Board {
    /**
     * Creates a new instance of the struct using the default values.
     *
     * This function creates and returns a new instance of the struct using the default values specified in the `new` function.
     *
     * @returns A new instance of the struct with default values.
     */
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /**
     * Creates a new instance of Chessboard.
     *
     * This function initializes a new Chessboard struct with empty squares and board information.
     *
     * @return A newly created Chessboard instance.
     */
    pub fn new() -> Self {
        let squares = [None; 64];
        Self {
            squares,

            board_info: BoardInfo::new(squares),
        }
    }

    /**
     * Creates a new Chessboard instance based on the given FEN (Forsyth-Edwards Notation) string.
     *
     * This function creates a new Chessboard instance and initializes it with the pieces and their positions based on the provided FEN string.
     *
     * @param fen - The FEN string representing the initial state of the chessboard.
     * @return A new Chessboard instance initialized with the pieces and positions from the FEN string.
     */
    pub fn new_from_fen(fen: &str) -> Self {
        let mut board = Self::new();

        // Generate squares from fen
        board.squares = squares_from_fen(fen);

        board
    }

    /**
     * Creates a new standard chessboard.
     *
     * This function creates a new chessboard with the standard starting position.
     * It uses the Forsyth–Edwards Notation (FEN) to represent the position.
     *
     * @return - The newly created chessboard.
     */
    pub fn new_standard() -> Self {
        let fen = "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr";
        let mut board = Self::new_from_fen(fen);
        board.update();
        board
    }

    /**
     * Update the chessboard state.
     *
     * This function updates the chessboard state by calling `update_board_info` and updating the `board_info` field of
     * the `Chessboard` struct based on the current state of `squares`.
     */
    pub fn update(&mut self) {
        self.board_info = update_board_info(self.board_info.clone(), self.squares);
    }

    /**
     * Retrieves the value of the square at the given position.
     *
     * This function returns the square value located at the specified position on the chessboard.
     *
     * @param pos - The position of the square to retrieve.
     * @return The value of the square at the specified position.
     */
    pub fn get(&self, pos: Position) -> Square {
        self.squares[idx(pos)]
    }

    /**
     * Converts the chessboard representation to FEN notation.
     *
     * This function converts the current chessboard state to the Forsyth–Edwards Notation (FEN). It calls the `fen_from_squares`
     * function which generates the FEN string based on the internal representation of the chessboard's squares.
     *
     * @return The chessboard state represented in FEN notation.
     */
    pub fn to_fen(&self) -> String {
        fen_from_squares(&self.squares)
    }

    /**
     * Undoes the last move made on the chessboard.
     *
     * This function reverses the effects of the last move by restoring the previous state of the chessboard.
     * It retrieves the last move from the move_history stack and updates the position of the moved piece, restores captured pieces (if any),
     * and updates the squares on the chessboard accordingly.
     */
    pub fn undo_move(&mut self) {
        let m = self.board_info.move_history.pop().unwrap();
        let piece = m.from_piece;
        //piece.first_move = true;
        let pos = piece.position;
        if m.is_capture() {
            let captured_piece = self.board_info.captured_pieces.pop().unwrap();
            self.squares[idx(m.to)] = Some(captured_piece);
        } else {
            self.squares[idx(m.to)] = None;
        }
        self.squares[idx(pos)] = Some(piece);
    }

    /**
     * Makes a move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     */
    pub fn make_move(&mut self, m: Move) {
        self.board_info.move_history.push(m.clone());
        match m.move_type {
            MoveType::Castle(castle_type) => self.make_castle_move(m, castle_type),
            MoveType::EnPassant => self.make_en_passant_move(m),
            MoveType::Promotion(piece_kind) => self.make_promotion_move(m, piece_kind),
            MoveType::PromotionCapture(piece_kind) => {
                self.make_promotion_capture_move(m, piece_kind)
            }
            _ => self.make_normal_move(m),
        }
    }

    /**
     * Makes a promotion capture move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     * @param piece_kind - The kind of piece to promote to.
     */
    fn make_promotion_capture_move(&mut self, m: Move, piece_kind: PieceKind) {
        let mut piece = m.from_piece;
        piece.kind = piece_kind;
        piece.has_moved = true;
        piece.en_passant = None;
        let pos = piece.position;
        piece.position = m.to;
        let captured_piece = self.squares[idx(m.to)];
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);
        self.board_info
            .captured_pieces
            .push(captured_piece.unwrap());
    }

    /**
     * Makes a Promotion move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     * @param piece_kind - The kind of piece to promote to.
     */
    fn make_promotion_move(&mut self, m: Move, piece_kind: PieceKind) {
        let mut piece = m.from_piece;
        piece.kind = piece_kind;
        if piece.first_move {
            piece.first_move = false;
        }
        piece.has_moved = true;
        piece.en_passant = None;
        let pos = piece.position;
        piece.position = m.to;
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);
    }

    /**
     * Makes an en passant move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     */
    fn make_en_passant_move(&mut self, m: Move) {
        let mut piece = m.from_piece;
        if piece.first_move {
            piece.first_move = false;
        }
        piece.has_moved = true;
        piece.en_passant = None;
        let pos = piece.position;
        piece.position = m.to;
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);
        let captured_piece = self.squares[idx((m.to.0, m.from.1))].unwrap();
        self.squares[idx((m.to.0, m.from.1))] = None;
        self.board_info.captured_pieces.push(captured_piece);
    }

    /**
     * Makes a castle move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     * @param castle_type - The type of castle move to be made.
     */
    fn make_castle_move(&mut self, m: Move, castle_type: CastleType) {
        let mut piece = m.from_piece;
        if piece.first_move {
            piece.first_move = false;
        }
        piece.has_moved = true;
        let pos = piece.position;
        piece.position = m.to;
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);
        let rook_pos = match castle_type {
            CastleType::KingSide => (7u8, pos.1) as Position,
            CastleType::QueenSide => (0u8, pos.1) as Position,
        };
        let mut rook = self.squares[idx(rook_pos)].unwrap();
        rook.has_moved = true;
        rook.position = match castle_type {
            CastleType::KingSide => (5u8, pos.1) as Position,
            CastleType::QueenSide => (3u8, pos.1) as Position,
        };
        self.squares[idx(rook_pos)] = None;
        self.squares[idx(rook.position)] = Some(rook);
    }

    /**
     * Makes a normal move on the chessboard.
     *
     * This function updates the chessboard state based on the given move. It updates the move history,
     * modifies the relevant pieces, captures pieces if necessary, and updates the position of the moved piece.
     *
     * @param m - The move to be made on the chessboard.
     */
    fn make_normal_move(&mut self, m: Move) {
        let mut piece = m.from_piece;
        if piece.first_move {
            piece.first_move = false;
        }
        piece.has_moved = true;
        let pos = piece.position;
        piece.position = m.to;
        if let Some(captured_piece) = self.squares[idx(m.to)] {
            self.board_info.captured_pieces.push(captured_piece);
        }
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);
    }
}

#[inline]
/**
 * Calculates the index in a one-dimensional array corresponding to a given position on a chessboard.
 *
 * This function takes a position (x, y) on a chessboard and calculates the corresponding index in a one-dimensional array representation of the chessboard.
 *
 * @param pos - The position (x, y) on the chessboard.
 * @return The calculated index in the one-dimensional array.
 */
pub fn idx(pos: Position) -> usize {
    (pos.1 * 8 + pos.0) as usize
}

/**
 * Generates the FEN string from the given squares.
 *
 * This function generates the Forsyth–Edwards Notation (FEN) string from the given squares.
 *
 * @param squares - The squares to generate the FEN string from.
 * @return The FEN string representing the given squares.
 */
pub fn squares_from_fen(fen: &str) -> [Square; 64] {
    let mut squares = [None; 64];
    let mut pos: Position = (0, 0);
    for c in fen.chars() {
        match c {
            '/' => {
                pos.0 = 0;
                pos.1 += 1;
            }
            '1'..='8' => {
                let n = c as u8 - b'0';
                for _ in 0..n {
                    squares[idx(pos)] = None;
                    pos.0 += 1;
                }
            }
            'p' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Pawn, pos, Color::Black));
                pos.0 += 1;
            }
            'r' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Rook, pos, Color::Black));
                pos.0 += 1;
            }
            'n' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Knight, pos, Color::Black));
                pos.0 += 1;
            }
            'b' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Bishop, pos, Color::Black));
                pos.0 += 1;
            }
            'q' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Queen, pos, Color::Black));
                pos.0 += 1;
            }
            'k' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::King, pos, Color::Black));
                pos.0 += 1;
            }
            'P' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Pawn, pos, Color::White));
                pos.0 += 1;
            }
            'R' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Rook, pos, Color::White));
                pos.0 += 1;
            }
            'N' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Knight, pos, Color::White));
                pos.0 += 1;
            }
            'B' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Bishop, pos, Color::White));
                pos.0 += 1;
            }
            'Q' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::Queen, pos, Color::White));
                pos.0 += 1;
            }
            'K' => {
                squares[idx(pos)] = Some(Piece::new(PieceKind::King, pos, Color::White));
                pos.0 += 1;
            }
            _ => (),
        }
    }
    squares
}

/**
 * Generates the FEN string from the given squares.
 *
 * This function generates the Forsyth–Edwards Notation (FEN) string from the given squares.
 *
 * @param squares - The squares to generate the FEN string from.
 * @return The FEN string representing the given squares.
 */
pub fn fen_from_squares(squares: &[Square; 64]) -> String {
    let mut fen = String::new();
    let mut empty_squares = 0;
    for y in 0..8 {
        for x in 0..8 {
            let pos = (x, y);
            let idx = idx(pos);
            if let Some(piece) = squares[idx] {
                if empty_squares > 0 {
                    fen.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                fen.push(to_char(piece));
            } else {
                empty_squares += 1;
            }
        }
        if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
            empty_squares = 0;
        }
        if y < 7 {
            fen.push('/');
        }
    }
    fen
}

/**
 * Checks if the board state represented by the FEN string is in check for the given player color.
 *
 * This function creates a new Board instance, initializes it with the squares parsed from the FEN string,
 * and then checks if the specified player color is in check on the board.
 *
 * @param fen - The FEN string representing the board state.
 * @param color - The player color to check for check.
 * @return true if the player is in check, false otherwise.
 */
pub fn is_fen_in_check(fen: &str, color: Color) -> bool {
    let mut board = Board::new();
    board.squares = squares_from_fen(fen);
    board.board_info.is_in_check(color)
}

pub fn display_board(board: &Board) {
    println!();
    for y in 0..8 {
        for x in 0..8 {
            let pos = (x, y);
            let idx = idx(pos);
            if let Some(piece) = board.squares[idx] {
                print!("{}", piece);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/**
 * Checks if a given position is within the bounds of the chessboard.
 *
 * This function takes a position represented by a tuple of two integers and checks if it falls within
 * the bounds of the chessboard (i.e., if the coordinates are both less than 8). Returns true if the
 * position is within bounds, false otherwise.
 *
 * @param pos - The position to check.
 * @return true if the position is within bounds, false otherwise.
 */
pub fn in_bounds(pos: Position) -> bool {
    pos.0 < 8 && pos.1 < 8
}

#[cfg(test)]
mod tests {
    use crate::board::piece::PieceKind;
    use crate::board::piece::PieceKind::King;
    use crate::board::PieceKind::{Bishop, Pawn, Queen};
    use crate::board::{display_board, Board};
    use crate::game::player::Color;
    use crate::game::player::Color::{Black, White};
    use crate::rules::r#move::CastleType::{KingSide, QueenSide};
    use crate::rules::r#move::Move;
    use crate::rules::r#move::MoveType::{
        Capture, Castle, EnPassant, Normal, Promotion, PromotionCapture,
    };

    #[test]
    /**
     * Test function for creating a standard chess board.
     *
     * This function creates a standard chess board using the `new_standard` method of the Board struct.
     * It then displays the board using the `display_board` function and asserts that the piece at position (0, 0)
     * is a Rook.
     */
    pub fn test_standard_board_creation() {
        let board = Board::new_standard();
        display_board(&board);

        assert_eq!(board.get((0, 0)).unwrap().kind, PieceKind::Rook);
    }

    #[test]
    /**
     * Test function for creating a Chessboard from a FEN string.
     *
     * This function creates a Chessboard object by parsing the FEN string provided as argument.
     * It then displays the board using the display_board function.
     * Finally, it asserts that the piece at position (0, 0) on the board is a Rook.
     */
    pub fn test_fen_board_creation() {
        let board = Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        display_board(&board);

        assert_eq!(board.get((0, 0)).unwrap().kind, PieceKind::Rook);
    }

    /**
     * Tests a move on the chessboard.
     *
     * This function tests a move on the chessboard by creating a Move object with the given parameters and
     * then making the move on the board.
     *
     * @param board - A mutable reference to the chessboard on which the move is to be tested.
     * @param from - The starting position of the move.
     * @param to - The target position of the move.
     * @param color - The player color making the move.
     */
    fn test_move(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Normal, color);
        board.make_move(m);
    }

    /**
     * Tests a capture move on the chessboard.
     *
     * This function performs a capture move on the chessboard, from a starting position to a target position,
     * with the specified player color. It creates a new move object using the specified parameters and calls
     * the make_move function of the Board struct to make the move on the board.
     *
     * @param board - The mutable reference to the chessboard.
     * @param from - The starting position of the move.
     * @param to - The target position of the move.
     * @param color - The color of the player making the move.
     */
    fn test_capture(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Capture, color);
        board.make_move(m);
    }

    /**
     * Tests the en passant move on the chessboard.
     *
     * This function tests the en passant move by creating a Move object with the specified parameters,
     * and then makes the move on the chessboard.
     *
     * Note: The en passant move is a special chess move where a pawn captures another pawn as if it had moved only one square forward.
     *
     * @param board - A mutable reference to the chessboard.
     * @param from - The starting position of the piece making the en passant move.
     * @param to - The destination position of the piece making the en passant move.
     * @param color - The color of the piece making the en passant move.
     */
    fn test_en_passant(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, EnPassant, color);
        board.make_move(m);
    }

    /**
     * Tests the promotion of a pawn on the chessboard.
     *
     * This function creates a move that represents a pawn promotion and applies it to the chessboard.
     *
     * @param board - A mutable reference to the chessboard.
     * @param from - The source square of the pawn.
     * @param to - The destination square where the pawn will promote.
     * @param color - The color of the promoting pawn.
     */
    fn test_promotion(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Promotion(Queen), color);
        board.make_move(m);
    }

    /**
     * Test promotion capture move.
     *
     * This function tests a promotion capture move in the chess board. It creates a Move object
     * with the specified piece, target position, promotion type, and player color. Then, it makes
     * the move on the board.
     *
     * @param board - The chess board to perform the move on.
     * @param from - The initial position of the piece.
     * @param to - The target position to move the piece to.
     * @param color - The color of the player making the move.
     */
    fn test_promotion_capture(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, PromotionCapture(Queen), color);
        board.make_move(m);
    }

    /**
     * Tests the queenside castle move.
     *
     * This function tests the queenside castle move by creating a move object and making the move on the board.
     *
     * @param board - The mutable reference to the chessboard.
     * @param from - The coordinates of the piece's starting position.
     * @param to - The coordinates of the piece's ending position.
     * @param color - The color of the player making the move.
     */
    fn test_queenside_castle(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Castle(QueenSide), color);
        board.make_move(m);
    }

    /**
     * Tests the possibility of performing a kingside castle move.
     *
     * This function simulates a kingside castle move on the given chessboard. It checks if the move
     * is legal and updates the board accordingly.
     *
     * @param board - A mutable reference to the chessboard.
     * @param from - The starting position of the piece to move.
     * @param to - The target position of the piece to move.
     * @param color - The color of the player executing the move.
     */
    fn test_kingside_castle(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Castle(KingSide), color);
        board.make_move(m);
    }

    /**
     * Reverts the last move made on the chessboard.
     *
     * This function undoes the previous move made on the chessboard by calling the `undo_move` method on the
     * `Board` struct, reverting the chessboard to its previous state.
     *
     * @param board - The board to perform the undo operation on.
     */
    fn test_undo(board: &mut Board) {
        board.undo_move();
    }

    #[test]
    /**
     * Executes a series of test moves on the chessboard.
     *
     * This function demonstrates the usage of the `test_move` function to perform a sequence of moves on a chessboard.
     * It also verifies the correctness of the moved pieces on the board after each move.
     *
     * Note: This function assumes the `display_board` function is defined.
     * It also assumes the `test_move` function is defined to make moves on the board.
     */
    pub fn test_moves() {
        let mut board = Board::new_standard();
        display_board(&board);

        // Test pawn moves
        let from = (4, 1); // e2
        let to = (4, 3); // e4
        test_move(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);

        let from = (4, 6); // e7
        let to = (4, 4); // e5
        test_move(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);

        let from = (3, 0); // d1
        let to = (7, 4); // h5
        test_move(&mut board, from, to, White);
        display_board(&board);

        // Test Other Pieces
        assert_eq!(board.get(to).unwrap().kind, Queen);

        let from = (5, 7); // f8
        let to = (1, 3); // b4
        test_move(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Bishop);

        let from = (1, 0); // b1
        let to = (2, 2); // c3
        test_move(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, PieceKind::Knight);

        let from = (6, 7); // g8
        let to = (5, 5); // f6
        test_move(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, PieceKind::Knight);

        // Test King Castling
        let from = (4, 0); // e1
        let to = (6, 0); // g1
        test_kingside_castle(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, King);

        let from = (4, 7); // e8
        let to = (2, 7); // c8
        test_queenside_castle(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, King);

        // Test Pawn En Passant
        let from = (7, 1); // h2
        let to = (7, 4); // h5 illegal move for testing
        test_move(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);

        let from = (6, 6); // g7
        let to = (6, 4); // g5
        test_move(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);

        let from = (7, 4); // h5
        let to = (6, 5); // g6
        test_en_passant(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);

        // Test Pawn Promotion
        let from = (2, 6); // c7
        let to = (2, 0); // e1 illegal move for testing
        test_promotion_capture(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Queen);

        let from = (6, 5); // g6
        let to = (6, 7); // g8 illegal move for testing
        test_promotion(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Queen);

        // Test Undo
        test_undo(&mut board);
        display_board(&board);

        assert!(board.get(to).is_none());

        let _captured_pieces = board.board_info.captured_pieces.clone();
        test_undo(&mut board);
        display_board(&board);

        let to = (2, 0);
        assert_eq!(board.get(to).unwrap().kind, Bishop);
    }

    #[test]
    /**
     * Undoes moves on the chessboard.
     *
     * This function demonstrates the process of undoing moves on a chessboard.
     * It creates a new standard chessboard, displays it, performs a test move, displays the updated board,
     * then undoes the move and displays the board again. Finally, it asserts that the piece at the initial position is a pawn.
     *
     * Note: The `display_board` and `test_undo` functions are assumed to be implemented elsewhere in the code.
     * This function serves as a test/demo for undoing moves.
     *
     */
    pub fn undo_moves() {
        let mut board = Board::new_standard();
        display_board(&board);

        let from = (0, 1); // Pawn
        let to = (0, 3);
        test_move(&mut board, from, to, White);
        display_board(&board);

        test_undo(&mut board);
        display_board(&board);

        assert_eq!(board.get(from).unwrap().kind, Pawn);
    }

    #[test]
    /**
     * Test function for capturing moves in a chess game.
     *
     * This function performs a series of test moves and captures on the chess board to validate
     * the correctness of the capture behavior.
     */
    pub fn test_capture_moves() {
        // Create a new standard chess board
        let mut board = Board::new_standard();
        display_board(&board);

        // Perform a pawn move for the white player
        let from = (0, 1); // Pawn initial position
        let to = (0, 3); // Pawn new position
        test_move(&mut board, from, to, White);
        display_board(&board);

        // Perform a pawn move for the black player
        let from = (1, 6); // Pawn initial position
        let to = (1, 4); // Pawn new position
        test_move(&mut board, from, to, Black);
        display_board(&board);

        // Perform a capture move for the white player
        let from = (0, 3); // Pawn initial position
        let to = (1, 4); // Opponent's pawn position
        test_capture(&mut board, from, to, White);
        display_board(&board);

        // Assert the number of captured pieces and the kind of the piece in the new position
        assert_eq!(board.board_info.captured_pieces.len(), 1);
        assert_eq!(board.get(to).unwrap().kind, Pawn);
    }

    // Color Index [White, Black]
    // Piece Index [King, Pawn, Knight, Bishop, Rook, Queen]
    // All Pieces Index White King, White Pawn, White Knight, White Bishop, White Rook, White Queen
    //                  Black King, Black Pawn, Black Knight, Black Bishop, Black Rook, Black Queen
    #[test]
    /**
     * Tests the bitboard functionalities of the Chessboard struct.
     *
     * This function asserts various properties of the bitboards in the Chessboard struct to ensure their correctness.
     * It checks the total number of pieces, the number of pieces and empty squares for each player, and the number
     * of pieces of each type (pawns, knights, bishops, rooks, and queens).
     * The assertions are based on a standard chessboard configuration.
     */
    pub fn test_bitboard() {
        let board = Board::new_standard();
        display_board(&board);
        assert_eq!(board.board_info.all_pieces_bitboard.count_ones(), 32);
        assert_eq!(board.board_info.all_pieces_bitboard.count_zeros(), 32);

        // pieces
        assert_eq!(board.board_info.player_bitboards[0].count_ones(), 16);
        assert_eq!(board.board_info.player_bitboards[0].count_zeros(), 48);
        assert_eq!(board.board_info.player_bitboards[1].count_ones(), 16);
        assert_eq!(board.board_info.player_bitboards[1].count_zeros(), 48);

        // kings
        assert_eq!(board.board_info.piece_bitboards[0].count_ones(), 1);
        assert_eq!(board.board_info.piece_bitboards[6].count_ones(), 1);
        assert_eq!(board.board_info.piece_bitboards[0].count_zeros(), 63);
        assert_eq!(board.board_info.piece_bitboards[6].count_zeros(), 63);

        // pawns
        assert_eq!(board.board_info.piece_bitboards[1].count_ones(), 8);
        assert_eq!(board.board_info.piece_bitboards[7].count_ones(), 8);
        assert_eq!(board.board_info.piece_bitboards[1].count_zeros(), 56);
        assert_eq!(board.board_info.piece_bitboards[7].count_zeros(), 56);

        // knights
        assert_eq!(board.board_info.piece_bitboards[2].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[8].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[2].count_zeros(), 62);
        assert_eq!(board.board_info.piece_bitboards[8].count_zeros(), 62);

        // bishops
        assert_eq!(board.board_info.piece_bitboards[3].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[9].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[3].count_zeros(), 62);
        assert_eq!(board.board_info.piece_bitboards[9].count_zeros(), 62);

        // rooks
        assert_eq!(board.board_info.piece_bitboards[4].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[10].count_ones(), 2);
        assert_eq!(board.board_info.piece_bitboards[4].count_zeros(), 62);
        assert_eq!(board.board_info.piece_bitboards[10].count_zeros(), 62);

        // queens
        assert_eq!(board.board_info.piece_bitboards[5].count_ones(), 1);
        assert_eq!(board.board_info.piece_bitboards[11].count_ones(), 1);
        assert_eq!(board.board_info.piece_bitboards[5].count_zeros(), 63);
        assert_eq!(board.board_info.piece_bitboards[11].count_zeros(), 63);
    }

    #[test]
    /**
     * Performs a test move on the chessboard.
     *
     * This function simulates a move on the chessboard for testing purposes. It moves a piece from the 'from' square
     * to the 'to' square for the specified player color. After the move, the chessboard is displayed and assertions are made
     * to verify the correctness of the move.
     */
    pub fn test_bitboard_move() {
        // Creating a new standard chessboard
        let mut board = Board::new_standard();

        // Displaying the current state of the chessboard
        display_board(&board);

        let from = (1, 1); // Coordinates of the 'from' square
        let to = (1, 3); // Coordinates of the 'to' square

        test_move(&mut board, from, to, White);

        // Displaying the state of the chessboard after the move
        display_board(&board);

        // Assertion to check if the piece at the 'to' square is a Pawn
        assert_eq!(board.get(to).unwrap().kind, Pawn);

        // Assertion to check if the 'from' square is now empty
        assert!(board.get(from).is_none());

        // Assertion to check the count of ones in the pawn piece bitboard
        assert_eq!(board.board_info.piece_bitboards[1].count_ones(), 8);

        // Assertion to check the count of zeros in the pawn piece bitboard
        assert_eq!(board.board_info.piece_bitboards[1].count_zeros(), 56);
    }

    #[test]
    /**
     * Performs a test on bitboard capture.
     *
     * This function tests the functionality of capturing a piece on the chessboard using the bitboard representation.
     * It performs a series of moves and captures, and then asserts the expected outcomes to validate the functionality.
     * The intermediate states of the chessboard are displayed during the test.
     */
    pub fn test_bitboard_capture() {
        // Create a new standard chessboard
        let mut board = Board::new_standard();

        // Display the initial chessboard state
        display_board(&board);

        // Perform a move from (0, 1) to (0, 3) with a white pawn
        let from = (0, 1);
        let to = (0, 3);
        test_move(&mut board, from, to, White);
        display_board(&board);

        // Perform a move from (1, 6) to (1, 4) with a black pawn
        let from = (1, 6);
        let to = (1, 4);
        test_move(&mut board, from, to, Black);
        display_board(&board);

        // Perform a capture from (0, 3) to (1, 4) with a white pawn
        let from = (0, 3);
        let to = (1, 4);
        test_capture(&mut board, from, to, White);
        display_board(&board);

        // Assert the expected outcomes of the test
        assert_eq!(board.board_info.captured_pieces.len(), 1);
        assert_eq!(board.get(to).unwrap().kind, Pawn);
        assert!(board.get(from).is_none());

        assert_eq!(board.board_info.piece_bitboards[1].count_ones(), 8);
        assert_eq!(board.board_info.piece_bitboards[1].count_zeros(), 56);
    }

    #[test]
    /**
     * Tests the functionality of undoing a move on the chessboard.
     *
     * This function performs a series of moves on the chessboard and then tests the undo feature.
     * It asserts that the piece at the 'from' position is a pawn after the undo operation is performed.
     */
    pub fn test_bitboard_undo() {
        // Create a new standard chessboard
        let mut board = Board::new_standard();

        // Display the initial board state
        display_board(&board);

        // Select the 'from' and 'to' coordinates for a move
        let from = (0, 1); // Pawn
        let to = (0, 3);

        // Test the move function
        test_move(&mut board, from, to, White);

        // Display the board after the move
        display_board(&board);

        // Test the undo function
        test_undo(&mut board);

        // Display the board after undoing the move
        display_board(&board);

        // Assert that the piece at the 'from' position is a pawn
        assert_eq!(board.get(from).unwrap().kind, Pawn);
    }
}
