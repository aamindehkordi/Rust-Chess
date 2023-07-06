use std::ptr::eq;
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

// An abstract Board class that represents a chess board.
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {

    //Constructors

    /// Creates a new Board with the default starting position.
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                // Create the piece for the tile
                let piece = match i {
                    1 => Some(Piece::new(PieceType::Pawn(Pawn::new(Color::White)), Color::White)),
                    6 => Some(Piece::new(PieceType::Pawn(Pawn::new(Color::Black)), Color::Black)),
                    0 => match j {
                        0 | 7 => Some(Piece::new(PieceType::Rook(Rook::new(Color::White)), Color::White)),
                        1 | 6 => Some(Piece::new(PieceType::Knight(Knight::new(Color::White)), Color::White)),
                        2 | 5 => Some(Piece::new(PieceType::Bishop(Bishop::new(Color::White)), Color::White)),
                        3 => Some(Piece::new(PieceType::Queen(Queen::new(Color::White)), Color::White)),
                        4 => Some(Piece::new(PieceType::King(King::new(Color::White)), Color::White)),
                        _ => None,
                    },
                    _ => None,
                };
                row.push(Tile::new((i, j), piece));
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    /// Creates a new empty Board.
    pub fn new_empty() -> Self {
        let mut tiles = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                row.push(Tile::new((i, j), None));
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    // Methods
    pub fn pick_up(&mut self, idx: (usize, usize)) -> Option<Piece> {
        let piece = self.tiles[idx.0][idx.1].piece.clone();
        self.tiles[idx.0][idx.1].piece = None;
        piece
    }

    pub fn put_down(&mut self, idx: (usize, usize), piece: Option<Piece>) {
        self.tiles[idx.0][idx.1].piece = piece;
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let piece = self.pick_up(from);
        self.put_down(to, piece);
    }

    // Getters
    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }
    pub fn get_tile(&self, idx: (usize, usize)) -> &Tile {
        &self.tiles[idx.0][idx.1]
    }
    pub fn get_tile_mut(&mut self, idx: (usize, usize)) -> &mut Tile {
        &mut self.tiles[idx.0][idx.1]
    }
    pub fn get_pieces(&self, piece: &Piece) -> Vec<(usize, usize)> {
        let mut pieces = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = &self.tiles[i][j].piece {
                    if eq(p, piece) {
                        pieces.push((i, j));
                    }
                }
            }
        }
        pieces
    }
    pub fn get_piece(&self, idx: (usize, usize)) -> Option<&Piece> {
        self.tiles[idx.0][idx.1].piece.as_ref()
    }
    /// Given an index, determines the proper notation for the tilee.
    /// For example, (0,0) would return "A1".
    pub fn get_notation(&self, idx: (usize, usize)) -> String {
        let mut notation = String::new();
        notation.push((idx.1 + 65) as u8 as char);
        notation.push((idx.0 + 49) as u8 as char);
        notation
    }
    // Setters
    pub fn set_tile(&mut self, idx: (usize, usize), tile: Tile) {
        self.tiles[idx.0][idx.1] = tile;
    }

    // Utility
    pub fn iter(&self) -> std::slice::Iter<Vec<Tile>> {
        self.tiles.iter()
    }
}