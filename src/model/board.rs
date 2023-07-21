use crate::model::moves::move_generator::MoveGenerator;
use crate::model::moves::r#move::{Move, MoveHistory};
use crate::model::pieces::pawn::Pawn;
use crate::model::pieces::rook::Rook;
use crate::model::pieces::knight::Knight;
use crate::model::pieces::bishop::Bishop;
use crate::model::pieces::queen::Queen;
use crate::model::pieces::king::King;
use crate::model::pieces::piece::Piece;
use crate::model::pieces::piece::Color;
use crate::model::pieces::piece::PieceType;

use crate::model::tile::Tile;

pub struct Board {
    pub tiles: Vec<Tile>,
    pub current_turn: Color,
    pub taken_pieces: Vec<Box<dyn Piece>>,
    pub all_possible_moves: Vec<Move>,
    pub move_generator: MoveGenerator,
    pub move_history: Vec<MoveHistory>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut tiles = Vec::new();
        for tile in &self.tiles {
            tiles.push(tile.clone());
        }
        let mut taken_pieces = Vec::new();
        for piece in &self.taken_pieces {
            taken_pieces.push(piece.clone_box());
        }
        Self {
            tiles,
            current_turn: self.current_turn.clone(),
            taken_pieces,
            all_possible_moves: self.all_possible_moves.clone(),
            move_generator: self.move_generator.clone(),
            move_history: self.move_history.clone(),
        }
    }
}

impl Board {
    pub fn new_standard() -> Self {
        let mut tiles = Vec::new();
        let taken_pieces = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                // Create the piece for the tile
                let piece = match i {
                    1 => Some(Box::new(Pawn::new(Color::White, (i, j))) as Box<dyn Piece>),
                    6 => Some(Box::new(Pawn::new(Color::Black, (i, j))) as Box<dyn Piece>),
                    0 => match j {
                        0 | 7 => Some(Box::new(Rook::new(Color::White, (i, j))) as Box<dyn Piece>),
                        1 | 6 => Some(Box::new(Knight::new(Color::White, (i, j))) as Box<dyn Piece>),
                        2 | 5 => Some(Box::new(Bishop::new(Color::White, (i, j))) as Box<dyn Piece>),
                        3 => Some(Box::new(Queen::new(Color::White, (i, j))) as Box<dyn Piece>),
                        4 => Some(Box::new(King::new(Color::White, (i, j))) as Box<dyn Piece>),
                        _ => None,
                    },
                    7 => match j {
                        0 | 7 => Some(Box::new(Rook::new(Color::Black, (i, j))) as Box<dyn Piece>),
                        1 | 6 => Some(Box::new(Knight::new(Color::Black, (i, j))) as Box<dyn Piece>),
                        2 | 5 => Some(Box::new(Bishop::new(Color::Black, (i, j))) as Box<dyn Piece>),
                        3 => Some(Box::new(Queen::new(Color::Black, (i, j))) as Box<dyn Piece>),
                        4 => Some(Box::new(King::new(Color::Black, (i, j))) as Box<dyn Piece>),
                        _ => None,
                    },
                    _ => None,
                };
                tiles.push(Tile::new((i, j), piece));
            }
        }
        Self { tiles, current_turn: Color::White, taken_pieces, all_possible_moves: Vec::new(), move_generator: MoveGenerator::new(), move_history: Vec::new() }
    }

    pub fn new() -> Self {
        let mut tiles = Vec::new();
        let taken_pieces = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                tiles.push(Tile::new((i, j), None));
            }
        }
        Self { tiles, current_turn: Color::White, taken_pieces, all_possible_moves: Vec::new(), move_generator: MoveGenerator::new(), move_history: Vec::new() }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            if c == ' ' {
                break;
            }
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let color = if c.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let piece = match c.to_ascii_lowercase() {
                    'p' => Some(Box::new(Pawn::new(color, (rank, file))) as Box<dyn Piece>),
                    'n' => Some(Box::new(Knight::new(color, (rank, file))) as Box<dyn Piece>),
                    'b' => Some(Box::new(Bishop::new(color, (rank, file))) as Box<dyn Piece>),
                    'r' => Some(Box::new(Rook::new(color, (rank, file))) as Box<dyn Piece>),
                    'q' => Some(Box::new(Queen::new(color, (rank, file))) as Box<dyn Piece>),
                    'k' => Some(Box::new(King::new(color, (rank, file))) as Box<dyn Piece>),
                    _ => None,
                };
                if let Some(piece) = piece {
                    board.get_tile_mut((rank, file)).set_piece(Some(piece));
                }
                file += 1;
            }
        }
        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.len() > 1 {
            let color = if parts[1] == "w" {
                Color::White
            } else {
                Color::Black
            };
            board.current_turn = color;
        }

        board.all_possible_moves = board.update_all_possible_moves();
        board
    }

    // Getters
    pub fn get_current_player(&self) -> &Color {
        &self.current_turn
    }
    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
    pub fn get_tile(&self, idx: (usize, usize)) -> &Tile {
        &self.tiles[idx.0 * 8 + idx.1]
    }
    pub fn get_tile_mut(&mut self, idx: (usize, usize)) -> &mut Tile {
        &mut self.tiles[idx.0 * 8 + idx.1]
    }
    pub fn get_piece(&self, idx: (usize, usize)) -> Option<Box<dyn Piece>> {
        let tile = &self.tiles[idx.0 * 8 + idx.1];
        tile.get_piece().as_ref().map(|piece| piece.clone_box())
    }

    pub fn get_all_pieces(&self) -> Vec<Box<dyn Piece>> {
        let mut pieces = Vec::new();
        for tile in &self.tiles {
            if let Some(piece) = tile.get_piece().as_ref() {
                pieces.push(piece.clone_box());
            }
        }
        pieces
    }


    pub fn get_taken_pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.taken_pieces
    }
    pub fn get_all_possible_moves(&mut self) -> &Vec<Move> {
        self.all_possible_moves = self.update_all_possible_moves();
        &self.all_possible_moves
    }

    /// Returns a vector of all possible moves for the current player.
    fn update_all_possible_moves(&self) -> Vec<Move> {
        
        // sort by color
        self.move_generator.generate_all_moves(self)
    }

    /// Returns true if the given index of the given color is attacked by an enemy piece.
    pub fn is_square_attacked(&self, idx: (usize, usize), color: Color) -> (bool, Option<Move>) {
        let mut mvs = self.update_all_possible_moves();
        // println!("All possible moves: {:?}", mvs.clone());
        // filter moves from opposite color
        mvs.retain(|mv| mv.clone().get_color() != color);

        for mv in &mvs {
            if mv.get_to() == &idx {
                // if the move is from a pawn, consider it an attack even if it's not a valid move
                if let Some(piece) = self.get_piece(*mv.get_from()) {
                    if piece.get_type() == PieceType::Pawn {
                        println!("Attack move: {:?}", mv);
                        return (true, Some(mv.clone()));
                    }
                }
                if mv.valid() {
                    println!("Threat move: {:?}", mv);
                    return (true, Some(mv.clone()));
                }
            }
        }
        (false, None)
    }

    pub fn find_king(&self, color: Color) -> (usize, usize) {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.get_piece((i, j)) {
                    if piece.get_color() == color && piece.get_type() == PieceType::King {
                        return (i, j);
                    }
                }
            }
        }
        panic!("King not found");
    }


    /// Returns true if the given player is in check.
    pub fn is_king_in_check(&self, color: &Color) -> bool {
        let king_idx = self.find_king(color.clone());
        let (is_square_attacked, mv ) = self.is_square_attacked(king_idx, color.clone());
        if is_square_attacked {
            if let Some(mv) = mv {
                if mv.valid() {
                    return true;
                }
                // println!("King is in check by: {:?}", mv);
            }
        }
        false

    }

    /// Returns true if the given player's king is trapped.
    /// A king is trapped if it has no moves.
    pub fn is_king_trapped(&self, color: &Color) -> bool {
        let king_idx = self.find_king(color.clone());
        let king = self.get_piece(king_idx).expect("King not found");
        let king_moves = king.get_moves();
        if king_moves.is_empty() {
            return true;
        }
        false
    }

    /// Pick up a piece from a tile.
    /// Returns the piece that was picked up.
    /// Returns None if there was no piece on the tile.
    pub fn pick_up_piece(&mut self, idx: &(usize, usize)) -> Option<Box<dyn Piece>> {
        self.tiles[idx.0 * 8 + idx.1].piece.take()
    }

    /// Puts down the picked up piece on a tile.
    pub fn put_down_piece(&mut self, idx: &(usize, usize), piece: Option<Box<dyn Piece>>) {
        self.tiles[idx.0 * 8 + idx.1].set_piece(piece);
    }

    pub fn temp_move_piece(&self, from: &(usize, usize), to: &(usize, usize)) -> bool {
        // copy the board
        let mut temp_board = self.clone();
        // pick up the piece
        let piece = temp_board.pick_up_piece(from);
        // put down the piece

        temp_board.put_down_piece(to, piece);
        temp_board.all_possible_moves = temp_board.update_all_possible_moves();
        // check if the king is in check
        temp_board.is_king_in_check(&self.current_turn)
    }

    /// Moves a piece from one tile to another.
    pub fn move_piece(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        if let Some(mut piece) = self.pick_up_piece(from) {
            // check if the piece is the same color as the current player
            if piece.get_color() != self.current_turn {
                self.put_down_piece(from, Some(piece));
                return;
            }

            piece.set_position(*to);
            self.put_down_piece(to, Some(piece));
            self.all_possible_moves = self.update_all_possible_moves();
        }
    }

    /// Moves a taken piece to the taken pieces vector.
    pub fn take_piece(&mut self, idx: &(usize, usize)) {
        let piece = self.tiles[idx.0 * 8 + idx.1].piece.take().unwrap();
        self.taken_pieces.push(piece);
    }
}
