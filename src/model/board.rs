use std::ptr::eq;
use std::slice::Iter;
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
        }
    }
}

impl Board {
    pub fn new() -> Self {
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
        Self { tiles, current_turn: Color::White, taken_pieces }
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


    pub fn find_piece(&self, piece: &Box<dyn Piece>) -> Option<(usize, usize)> {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = &self.tiles[i * 8 + j].piece {
                    if eq(p, piece) {
                        return Some((i, j));
                    }
                }
            }
        }
        None
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


    /// Change the current player.
    /// Returns the new current player.
    pub fn change_current_player(&mut self){
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    /// Returns true if the given player is in check.
    pub fn is_checkmate(&self, color: &Color) -> bool {
        return if self.is_king_in_check(&color) && self.is_king_trapped(&color){ true } else { false };
    }

    /// Returns true if the given player is in check.
    pub fn is_king_in_check(&self, color: &Color) -> bool {
        let king_idx = self.find_king(color.clone());
        if let idx = king_idx {
            let king = self.get_piece(idx).expect("King not found");
            let king_moves = king.get_moves();
            for i in 0..8 {
                for j in 0..8 {
                    if let Some(piece) = self.get_piece((i, j)) {
                        if piece.get_color() != *color {
                            let piece_moves = piece.get_moves();
                            for piece_move in piece_moves {
                                if king_moves.contains(piece_move) {
                                    println!("{color:?} is in check!");
                                    return true;
                                }
                            }
                        }
                    }
                }
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
        if king_moves.len() == 0 {
            return true;
        }
        false
    }

    /// Given an index, determines the proper notation for the tile.
    /// For example, (0,0) would return "A1".
    pub fn get_notation(&self, idx: (usize, usize)) -> String {
        let mut notation = String::new();
        notation.push((idx.1 + 65) as u8 as char);
        notation.push((idx.0 + 49) as u8 as char);
        notation
    }

    pub fn iter(&self) -> Iter<'_, Tile> {
        self.tiles.iter()
    }

    /// Pick up a piece from a tile.
    /// Returns the piece that was picked up.
    /// Returns None if there was no piece on the tile.
    pub fn pick_up_piece(&mut self, idx: &(usize, usize)) -> Option<Box<dyn Piece>> {
        self.tiles[idx.0 * 8 + idx.1].piece.take()
    }


    /// Puts down the picked up piece on a tile.
    pub fn put_down_piece(&mut self, idx: &(usize, usize), piece: Option<Box<dyn Piece>>) {
        self.tiles[idx.0 * 8 + idx.1].piece = piece;
    }

    pub fn temp_move_piece(&mut self, from: &(usize, usize), to: &(usize, usize)) -> bool {
        // copy the board
        let mut temp_board = self.clone();
        // pick up the piece
        let piece = temp_board.pick_up_piece(from);
        // put down the piece
        temp_board.put_down_piece(to, piece);
        // check if the king is in check
        !temp_board.is_king_in_check(&self.current_turn)
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
            self.change_current_player();
        }
    }

    /// Moves a taken piece to the taken pieces vector.
    pub fn take_piece(&mut self, idx: &(usize, usize)) {
        let piece = self.tiles[idx.0 * 8 + idx.1].piece.take().unwrap();
        self.taken_pieces.push(piece);
    }
}