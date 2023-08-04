use crate::board::board_info::BoardInfo;
use crate::board::piece::{Piece,  to_char};
use crate::game::player;
use crate::rules::r#move::{Move, MoveType};

pub mod piece;
mod board_info;

pub type Position = (u8, u8);
pub type Square = Option<Piece>;

#[derive(Clone)]
pub struct Board {
    pub squares: [Square; 64],
    pub board_info: BoardInfo,
    pub move_history: Vec<Move>,
    pub captured_pieces: Vec<Piece>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let squares = [None; 64];
        Self {
            squares,

            board_info: BoardInfo::new(squares),

            move_history: Vec::new(),
            captured_pieces: Vec::new(),
        }
    }

    pub fn new_from_fen(fen: &str) -> Self {
        let mut board = Self::new();

        board.squares = squares_from_fen(fen);
        board.board_info.update_bitboards(board.squares);

        board
    }

    pub fn new_standard() -> Self {
        let fen = "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr";
        let mut board = Self::new();

        board.squares = squares_from_fen(fen);
        board.board_info.update_bitboards(board.squares);

        board
    }

    pub fn get(&self, pos: Position) -> Square {
        self.squares[idx(pos)]
    }


    pub fn make_move(&mut self, m: Move) {
        self.move_history.push(m.clone());
        let mut piece = m.from_piece;
        if piece.first_move { piece.first_move = false; }
        piece.has_moved = true;
        let pos = piece.position;
        piece.position = m.to;
        if let Some(captured_piece) = self.squares[idx(m.to)] {
            self.captured_pieces.push(captured_piece);
        }
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);

        self.board_info.update_bitboards(self.squares);
    }

    pub fn undo_move(&mut self) {
        let m = self.move_history.pop().unwrap();
        let mut piece = m.from_piece;
        //piece.first_move = true;
        let pos = piece.position;
        piece.position = m.from;
        if m.is_capture() {
            let captured_piece = self.captured_pieces.pop().unwrap();
            self.squares[idx(m.to)] = Some(captured_piece);
        } else {
            self.squares[idx(m.to)] = None;
        }
        self.squares[idx(pos)] = Some(piece);

        self.board_info.update_bitboards(self.squares);
    }
}

#[inline]
pub fn idx(pos: Position) -> usize {
    (pos.1 * 8 + pos.0) as usize
}

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
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Pawn,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'r' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Rook,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'n' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Knight,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'b' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Bishop,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'q' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Queen,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'k' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::King,
                    pos,
                    player::Color::Black,
                ));
                pos.0 += 1;
            }
            'P' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Pawn,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            'R' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Rook,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            'N' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Knight,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            'B' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Bishop,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            'Q' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::Queen,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            'K' => {
                squares[idx(pos)] = Some(Piece::new(
                    piece::PieceKind::King,
                    pos,
                    player::Color::White,
                ));
                pos.0 += 1;
            }
            _ => (),
        }
    }
    squares
}

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

pub fn display_board(board: &Board) {
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

#[cfg(test)]
mod tests {
    use crate::board::{display_board, piece, Board};
    use crate::board::piece::PieceKind::{Bishop, Pawn, Queen};
    use crate::game::player::Color;
    use crate::game::player::Color::{Black, White};
    use crate::rules::r#move::CastleType::KingSide;
    use crate::rules::r#move::Move;
    use crate::rules::r#move::MoveType::{Capture, Castle, EnPassant, Normal, Promotion, PromotionCapture};

    #[test]
    pub fn test_standard_board_creation() {
        let board = Board::new_standard();
        display_board(&board);

        assert_eq!(board.get((0, 0)).unwrap().kind, piece::PieceKind::Rook);
    }

    #[test]
    pub fn test_fen_board_creation() {
        let board = Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        display_board(&board);

        assert_eq!(board.get((0, 0)).unwrap().kind, piece::PieceKind::Rook);
    }

    fn test_move(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Normal, color);
        board.make_move(m);
    }

    fn test_capture(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Capture, color);
        board.make_move(m);
    }

    fn test_en_passant(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, EnPassant, color);
        board.make_move(m);
    }

    fn test_promotion(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Promotion(Queen), color);
        board.make_move(m);
    }

    fn test_promotion_capture(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, PromotionCapture(Queen), color);
        board.make_move(m);
    }

    fn test_queenside_castle(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Castle(KingSide), color);
        board.make_move(m);
    }

    fn test_kingside_castle(board: &mut Board, from: (u8, u8), to: (u8, u8), color: Color) {
        let m = Move::new(board.get(from).unwrap(), to, Castle(KingSide), color);
        board.make_move(m);
    }

    fn test_undo(board: &mut Board) {
        board.undo_move();
    }

    #[test]
    pub fn test_moves() {
        let mut board = Board::new_standard();
        display_board(&board);

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

        assert_eq!(board.get(to).unwrap().kind, piece::PieceKind::Knight);

        let from = (6, 7); // g8
        let to = (5, 5); // f6
        test_move(&mut board, from, to, Black);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, piece::PieceKind::Knight);

        // continue to castle once implemented.

    }

    #[test]
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
    pub fn test_capture_moves() {
        let mut board = Board::new_standard();
        display_board(&board);

        let from = (0, 1); // Pawn
        let to = (0, 3);
        test_move(&mut board, from, to, White);
        display_board(&board);

        let from = (1, 6); // Pawn
        let to = (1, 4);
        test_move(&mut board, from, to, Black);
        display_board(&board);

        let from = (0, 3); // Pawn
        let to = (1, 4);
        test_capture(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.captured_pieces.len(), 1);
        assert_eq!(board.get(to).unwrap().kind, Pawn);
    }

    // Color Index [White, Black]
    // Piece Index [King, Pawn, Knight, Bishop, Rook, Queen]
    // All Pieces Index White King, White Pawn, White Knight, White Bishop, White Rook, White Queen
    //                  Black King, Black Pawn, Black Knight, Black Bishop, Black Rook, Black Queen
    #[test]
    pub fn test_bitboard() {
        let mut board = Board::new_standard();
        display_board(&board);
        assert_eq!(board.board_info.all_pieces_bitboard.count_ones(), 32);
        assert_eq!(board.board_info.all_pieces_bitboard.count_zeros(), 32);

        // pieces
        assert_eq!(board.board_info.color_bitboards[0].count_ones(), 16);
        assert_eq!(board.board_info.color_bitboards[0].count_zeros(), 48);
        assert_eq!(board.board_info.color_bitboards[1].count_ones(), 16);
        assert_eq!(board.board_info.color_bitboards[1].count_zeros(), 48);

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
    pub fn test_bitboard_move() {
        let mut board = Board::new_standard();
        display_board(&board);

        let from = (0, 1); // Pawn
        let to = (0, 3);
        test_move(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.get(to).unwrap().kind, Pawn);
        assert!(board.get(from).is_none());

        assert_eq!(board.board_info.piece_bitboards[1].count_ones(), 8);
        assert_eq!(board.board_info.piece_bitboards[1].count_zeros(), 56);
    }

    #[test]
    pub fn test_bitboard_capture() {
        let mut board = Board::new_standard();
        display_board(&board);

        let from = (0, 1); // Pawn
        let to = (0, 3);
        test_move(&mut board, from, to, White);
        display_board(&board);

        let from = (1, 6); // Pawn
        let to = (1, 4);
        test_move(&mut board, from, to, Black);
        display_board(&board);

        let from = (0, 3); // Pawn
        let to = (1, 4);
        test_capture(&mut board, from, to, White);
        display_board(&board);

        assert_eq!(board.captured_pieces.len(), 1);
        assert_eq!(board.get(to).unwrap().kind, Pawn);
        assert!(board.get(from).is_none());

        assert_eq!(board.board_info.piece_bitboards[1].count_ones(), 8);
        assert_eq!(board.board_info.piece_bitboards[1].count_zeros(), 56);
    }

    #[test]
    pub fn test_bitboard_undo() {
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

}
