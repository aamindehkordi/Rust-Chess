use crate::board::piece::Piece;
use crate::game::player;
use crate::rules::r#move::Move;

pub mod piece;

pub type Position = (u8, u8);
pub type Square = Option<Piece>;
pub type Bitboard = u64;

#[derive(Clone)]
pub struct Board {
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

    pub move_history: Vec<Move>,
}

impl Board {
    pub fn new_standard() -> Self {
        let fen = "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr";
        let mut board = Self {
            squares: [None; 64],

            piece_bitboards: [0; 12],
            color_bitboards: [0; 2],
            all_pieces_bitboard: 0,

            piece_capture_bitboards: [0; 12],
            color_capture_bitboards: [0; 2],
            all_pieces_capture_bitboard: 0,

            piece_move_bitboards: [0; 12],
            color_move_bitboards: [0; 2],
            all_pieces_move_bitboard: 0,

            white_king_pos: (4, 0),
            black_king_pos: (4, 7),

            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,

            move_history: Vec::new(),
        };

        board.squares = squares_from_fen(fen);
        board.update_bitboards();

        board
    }

    pub fn get(&self, pos: Position) -> Square {
        self.squares[idx(pos)]
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

    pub fn make_move(&mut self, m: Move) {
        self.move_history.push(m.clone());
        let mut piece = m.from_piece.clone();
        piece.has_moved = true;
        let pos = piece.position;
        piece.position = m.to;
        self.squares[idx(pos)] = None;
        self.squares[idx(m.to)] = Some(piece);

        self.update_bitboards();
    }

    pub fn update_bitboards(&mut self){

        // Reset bitboards
        self.reset_bitboards();

        // Update bitboards
        for pos in 0..64 {
            if let Some(piece) = self.squares[pos] {
                let bitboard = 1 << pos;
                self.piece_bitboards[piece::idx(piece.kind, piece.color)] |= bitboard;
                self.color_bitboards[piece::color_idx(piece.color)] |= bitboard;
                self.all_pieces_bitboard |= bitboard;

            }
        }


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
            },
            '1'..='8' => {
                let n = c as u8 - b'0';
                for _ in 0..n {
                    squares[idx(pos)] = None;
                    pos.0 += 1;
                }
            },
            'p' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Pawn, pos, player::Color::Black));
                pos.0 += 1;
            },
            'r' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Rook, pos, player::Color::Black));
                pos.0 += 1;
            },
            'n' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Knight, pos, player::Color::Black));
                pos.0 += 1;
            },
            'b' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Bishop, pos, player::Color::Black));
                pos.0 += 1;
            },
            'q' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Queen, pos, player::Color::Black));
                pos.0 += 1;
            },
            'k' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::King, pos, player::Color::Black));
                pos.0 += 1;
            },
            'P' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Pawn, pos, player::Color::White));
                pos.0 += 1;
            },
            'R' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Rook, pos, player::Color::White));
                pos.0 += 1;
            },
            'N' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Knight, pos, player::Color::White));
                pos.0 += 1;
            },
            'B' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Bishop, pos, player::Color::White));
                pos.0 += 1;
            },
            'Q' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::Queen, pos, player::Color::White));
                pos.0 += 1;
            },
            'K' => {
                squares[idx(pos)] = Some(Piece::new(piece::PieceKind::King, pos, player::Color::White));
                pos.0 += 1;
            },
            _ => (),
        }
    }
    squares
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
    use crate::board::{Board, display_board, piece};

    #[test]
    pub fn test_standard_board_creation() {
        let board = Board::new_standard();
        display_board(&board);

        assert_eq!(board.get((0, 0)).unwrap().kind, piece::PieceKind::Rook);
    }
}



