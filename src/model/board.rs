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
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = Vec::new();
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
        Self { tiles, current_turn: Color::White }
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
        self.tiles[idx.0 * 8 + idx.1].piece.as_ref().map(|piece| piece.clone_box())
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

    pub fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = &self.tiles[i * 8 + j].piece {
                    if p.get_color() == color && p.get_type() == PieceType::King {
                        return Some((i, j));
                    }
                }
            }
        }
        None
    }


    /// Change the current player.
    /// Returns the new current player.
    pub fn change_current_player(&mut self){
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
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
    pub fn pick_up_piece(&mut self, idx: (usize, usize)) -> Option<Box<dyn Piece>> {
        self.tiles[idx.0 * 8 + idx.1].piece.take()
    }


    /// Puts down the picked up piece on a tile.
    pub fn put_down_piece(&mut self, idx: (usize, usize), piece: Box<dyn Piece>) {
        self.tiles[idx.0 * 8 + idx.1].piece = Some(piece);
    }

    /// Moves a piece from one tile to another.
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let piece = self.pick_up_piece(from).unwrap();
        self.put_down_piece(to, piece);
    }
}