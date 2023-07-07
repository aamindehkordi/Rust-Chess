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
                    1 => Some(Piece::new(PieceType::Pawn(Pawn::new(Color::Black, (i, j))), Color::Black)),
                    6 => Some(Piece::new(PieceType::Pawn(Pawn::new(Color::White, (i, j))), Color::White)),
                    0 => match j {
                        0 | 7 => Some(Piece::new(PieceType::Rook(Rook::new(Color::Black, (i, j))), Color::Black)),
                        1 | 6 => Some(Piece::new(PieceType::Knight(Knight::new(Color::Black, (i, j))), Color::Black)),
                        2 | 5 => Some(Piece::new(PieceType::Bishop(Bishop::new(Color::Black, (i, j))), Color::Black)),
                        3 => Some(Piece::new(PieceType::Queen(Queen::new(Color::Black, (i, j))), Color::Black)),
                        4 => Some(Piece::new(PieceType::King(King::new(Color::Black, (i, j))), Color::Black)),
                        _ => None,
                    },
                    7 => match j {
                        0 | 7 => Some(Piece::new(PieceType::Rook(Rook::new(Color::White, (i, j))), Color::White)),
                        1 | 6 => Some(Piece::new(PieceType::Knight(Knight::new(Color::White, (i, j))), Color::White)),
                        2 | 5 => Some(Piece::new(PieceType::Bishop(Bishop::new(Color::White, (i, j))), Color::White)),
                        3 => Some(Piece::new(PieceType::Queen(Queen::new(Color::White, (i, j))), Color::White)),
                        4 => Some(Piece::new(PieceType::King(King::new(Color::White, (i, j))), Color::White)),
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
    pub fn get_pieces(&self, piece: &Piece) -> Vec<(usize, usize)> {
        let mut pieces = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = &self.tiles[i * 8 + j].piece {
                    if eq(p, piece) {
                        pieces.push((i, j));
                    }
                }
            }
        }
        pieces
    }


    pub fn get_piece(&mut self, idx: (usize, usize)) -> Option<&mut Piece> {
        self.tiles[idx.0 * 8 + idx.1].piece.as_mut()
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
    pub fn pick_up_piece(&mut self, idx: (usize, usize)) -> Option<Piece> {
        if let Some(piece) = self.tiles[idx.0 * 8 + idx.1].piece.take() {
            Some(piece)
        } else {
            None
        }

    }


    /// Puts down the picked up piece on a tile.
    pub fn put_down_piece(&mut self, idx: (usize, usize), piece: Piece) {
        self.tiles[idx.0 * 8 + idx.1].piece = Some(piece);
    }
}