use crate::board::piece::PieceKind;
use crate::board::Square;
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

            white_can_castle_kingside: false,
            white_can_castle_queenside: false,
            black_can_castle_kingside: false,
            black_can_castle_queenside: false,

            move_history: Vec::new(),
            captured_pieces: Vec::new(),
            white_psuedo_moves: Vec::new(),
            black_psuedo_moves: Vec::new(),
            valid_moves: Vec::new(),

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

    pub fn king(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::King, color)]
    }

    pub fn queen(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Queen, color)]
    }

    pub fn rook(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Rook, color)]
    }

    pub fn bishop(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Bishop, color)]
    }

    pub fn knight(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Knight, color)]
    }

    pub fn pawn(&self, color: Color) -> Bitboard {
        self.piece_bitboards[bb_piece_idx(PieceKind::Pawn, color)]
    }

    pub fn king_pos(&self, color: Color) -> Position {
        if color == Color::White {
            self.white_king_pos
        } else {
            self.black_king_pos
        }
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let enemy_color = color.other();
        let king = self.king(color);
        let enemy_moves = self.color_move_bitboards[bb_color_idx(enemy_color)];
        (king & enemy_moves) != 0
    }

    pub fn is_attacked(&self, pos: Position, color: Color) -> bool {
        let enemy_color = color.other();
        let enemy_moves = self.color_move_bitboards[bb_color_idx(enemy_color)];
        (pos_to_bb(pos) & enemy_moves) != 0
    }

    pub fn is_valid(&self, mv: &Move) -> bool {
        let from = mv.from;
        let to = mv.to;
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        from_x < 8 && from_y < 8 && to_x < 8 && to_y < 8
        // todo check for check / pins
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
pub fn bb(idx: usize) -> Bitboard {
    1 << idx
}

pub fn pos_to_bb(pos: Position) -> Bitboard {
    let (x, y) = pos;
    1 << (x + y * 8)
}

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
    board_info
}

pub fn update_bitboards(board_info: &mut BoardInfo, piece: &Piece, position: usize) {
    let kind = piece.kind;
    let color = piece.color;

    let bitboard = pos_to_bb(((position % 8) as u8, (position / 8) as u8));
    board_info.piece_bitboards[bb_piece_idx(kind, color)] |= bitboard;
    board_info.player_bitboards[bb_color_idx(color)] |= bitboard;
    board_info.all_pieces_bitboard |= bitboard;
}