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
        Self { tiles, current_turn: Color::White, taken_pieces }
    }

    pub fn new() -> Self {
        let mut tiles = Vec::new();
        let taken_pieces = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                tiles.push(Tile::new((i, j), None));
            }
        }
        Self { tiles, current_turn: Color::White, taken_pieces }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            if c == ' ' {
                break;
            }
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
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
    pub fn get_taken_pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.taken_pieces
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
        }
    }

    /// Moves a taken piece to the taken pieces vector.
    pub fn take_piece(&mut self, idx: &(usize, usize)) {
        let piece = self.tiles[idx.0 * 8 + idx.1].piece.take().unwrap();
        self.taken_pieces.push(piece);
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_fen() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(board.get_piece((0, 0)).unwrap().get_type(), PieceType::Rook);
        assert_eq!(board.get_piece((0, 1)).unwrap().get_type(), PieceType::Knight);
        assert_eq!(board.get_piece((0, 2)).unwrap().get_type(), PieceType::Bishop);
        assert_eq!(board.get_piece((0, 3)).unwrap().get_type(), PieceType::Queen);
        assert_eq!(board.get_piece((0, 4)).unwrap().get_type(), PieceType::King);
        assert_eq!(board.get_piece((0, 5)).unwrap().get_type(), PieceType::Bishop);
        assert_eq!(board.get_piece((0, 6)).unwrap().get_type(), PieceType::Knight);
        assert_eq!(board.get_piece((0, 7)).unwrap().get_type(), PieceType::Rook);
        for i in 0..8 {
            assert_eq!(board.get_piece((1, i)).unwrap().get_type(), PieceType::Pawn);
        }
        for i in 0..8 {
            assert_eq!(board.get_piece((6, i)).unwrap().get_type(), PieceType::Pawn);
        }
        assert_eq!(board.get_piece((7, 0)).unwrap().get_type(), PieceType::Rook);
        assert_eq!(board.get_piece((7, 1)).unwrap().get_type(), PieceType::Knight);
        assert_eq!(board.get_piece((7, 2)).unwrap().get_type(), PieceType::Bishop);
        assert_eq!(board.get_piece((7, 3)).unwrap().get_type(), PieceType::Queen);
        assert_eq!(board.get_piece((7, 4)).unwrap().get_type(), PieceType::King);
        assert_eq!(board.get_piece((7, 5)).unwrap().get_type(), PieceType::Bishop);
        assert_eq!(board.get_piece((7, 6)).unwrap().get_type(), PieceType::Knight);
        assert_eq!(board.get_piece((7, 7)).unwrap().get_type(), PieceType::Rook);
    }

    #[test]
    fn test_is_king_in_check() {
        let mut no_check_board = Board::new_standard();
        assert_eq!(no_check_board.is_king_in_check(&Color::Black), false);

        let mut check_board = Board::from_fen("RkrK4/8/8/8/8/8/8/8"); // white rook -> black king, black rook -> white king
        assert_eq!(check_board.is_king_in_check(&Color::Black), true);

    }

    #[test]
    fn test_is_king_trapped(){
        let mut not_trapped_board = Board::from_fen("k7/8/8/8/8/8/7K/8"); // free kings
        assert_eq!(not_trapped_board.is_king_trapped(&Color::White), false);

        let mut trapped_board = Board::new_standard();
        assert_eq!(trapped_board.is_king_trapped(&Color::White), true);
    }

    #[test]
    fn test_temp_move_piece(){
        let mut not_pinned_board = Board::from_fen("k7/b7/8/8/8/8/B7/K7"); // A file has black king then bishop and then white bishop and king
        assert_eq!(not_pinned_board.temp_move_piece(&(2,6), &(3,5)), true); // black bishop can move
        assert_eq!(not_pinned_board.temp_move_piece(&(5,1), &(4,2)), true); // white bishop can move

        let mut pinned_king_board = Board::from_fen("k6r/b7/8/8/8/8/7B/R6K"); // white rook pinning black bishop (1,7) to black king and vice versa (6,0)
        assert_eq!(pinned_king_board.temp_move_piece(&(1,7), &(2,6)), false); // black bishop can't move because it's pinned
        assert_eq!(pinned_king_board.temp_move_piece(&(6,0), &(5,1)), false); // white bishop can't move because it's pinned
    }

    #[test]
    fn test_move_piece(){
        let mut board = Board::from_fen("k7/b7/8/8/8/8/B7/K7"); // free bishops
        board.move_piece(&(2,6)as &(usize,usize), &(3,5)as &(usize,usize)); // black bishop moves
        board.move_piece(&(5,1)as &(usize,usize), &(4,2)as &(usize,usize)); // white bishop moves

        assert_eq!(board.get_piece(&(2,6) as (usize, usize)).unwrap().get_type(), PieceType::Bishop); // black bishop moved
        assert_eq!(board.get_piece(&(5,1) as (usize, usize)).unwrap().get_type(), PieceType::Bishop); // white bishop moved
    }

    #[test]
    fn test_take_piece(){
        let mut board = Board::from_fen("k7/r7/8/8/8/8/R7/K7"); // free rooks
        board.move_piece(&(0,1)as &(usize,usize), &(0,7)as &(usize,usize)); // black rook takes white rook
        board.move_piece(&(0,8)as &(usize,usize), &(0,7)as &(usize,usize)); // white king takes black rook

        assert_eq!(board.get_piece(&(0,1) as (usize, usize)), None); // black rook taken
        assert_eq!(board.get_piece(&(0,8) as (usize, usize)), None); // white rook taken
        assert_eq!(board.get_piece(&(0,7) as (usize, usize)).unwrap().get_type(), PieceType::King); // white king moved
    }

}
