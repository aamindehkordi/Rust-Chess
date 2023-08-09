pub mod r#move;

use crate::board::board_info::BoardInfo;
use crate::board::in_bounds;
use crate::board::piece::{Piece, PieceKind};
use crate::game::player::Color;
use crate::rules::r#move::{CastleType, Move, MoveType};
use std::cmp::{max, min};



// Function to generate all legal moves for a knight at a given position
/**
 * Generates a list of possible moves for a knight on the given chessboard.
 *
 * This function calculates and returns a vector of Move objects representing the possible moves that a knight can make.
 *
 * @param board_info - The board information containing the current state of the chessboard.
 * @param piece - The knight piece for which the moves are to be generated.
 *
 * @return A vector of Move objects representing the possible moves for the knight.
 */
pub fn generate_knight_moves(board_info: BoardInfo, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = piece.color;
    let from_pos = piece.position;
    let (x, y) = from_pos;

    // The knight can move in 8 directions: up up left/right, down down left/right, left left up/down, right right up/down
    let directions = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    for direction in &directions {
        let to_pos = ((x as i8 + direction.0) as u8, (y as i8 + direction.1) as u8);
        capture_or_normal(board_info.clone(), color, piece, to_pos, &mut moves);
    }
    moves
}

// Function to generate all legal moves for a sliding piece at a given position
pub fn generate_sliding_move(board_info: BoardInfo, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let from_pos = piece.position;
    let color = piece.color;
    let directions: Vec<(i8, i8)>;
    // horizontal and vertical directions
    let hdirections = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    let ddirections = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    // diagonal directions
    if piece.kind == PieceKind::Queen {
        directions = hdirections
            .iter()
            .chain(ddirections.iter())
            .cloned()
            .collect();
    } else if piece.kind == PieceKind::Rook {
        directions = hdirections.to_vec();
    } else {
        // Bishop
        directions = ddirections.to_vec();
    }

    for direction in directions.iter() {
        let mut distance = 1;
        loop {
            let new_x = (from_pos.0 as i8 + distance * direction.0) as u8;
            let new_y = (from_pos.1 as i8 + distance * direction.1) as u8;
            let to_pos = (new_x, new_y);

            if capture_or_normal(board_info.clone(), color, piece, to_pos, &mut moves) {
                break;
            }
            distance += 1;
        }
    }
    moves
}

// Pushes all promotion piece types moves to the list of moves
/**
 * Generates a list of promotion moves.
 *
 * This function takes in the board information, the color of the player, the from and to positions of the move,
 * and generates a list of possible promotion moves for the given piece.
 *
 * @param board_info - The board information containing the current state of the game board.
 * @param _color - The color of the player.
 * @param fmv - The from position of the move.
 * @param pmv - The to position of the move.
 * @return A vector of Move objects representing the possible promotion moves.
 */
pub fn promotion_move(
    board_info: BoardInfo,
    _color: Color,
    fmv: (u8, u8),
    pmv: (u8, u8),
) -> Vec<Move> {
    let mut moves = vec![];
    let piece = board_info.get_square(fmv).unwrap();
    let to_pos = pmv;
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::Promotion(PieceKind::Queen),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::Promotion(PieceKind::Rook),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::Promotion(PieceKind::Bishop),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::Promotion(PieceKind::Knight),
        piece.color,
    ));
    moves
}

// Pushes all promotion piece types attack moves to the list of moves
/**
 * Generates promotion attack moves for a piece on the specified board.
 *
 * This function takes in the board information, piece color, from position, and to position.
 * It generates a vector of moves representing all possible promotion attack moves for the piece.
 *
 * @param board_info - The board information containing the piece positions.
 * @param color - The color of the piece.
 * @param fmv - The from position of the piece.
 * @param pmv - The to position for the piece.
 * @return A vector of Move objects representing the promotion attack moves.
 */
pub fn promotion_attack_move(
    board_info: BoardInfo,
    _color: Color,
    fmv: (u8, u8),
    pmv: (u8, u8),
) -> Vec<Move> {
    let mut moves = vec![];
    let piece = board_info.get_square(fmv).unwrap();
    let to_pos = pmv;
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::PromotionCapture(PieceKind::Queen),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::PromotionCapture(PieceKind::Rook),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::PromotionCapture(PieceKind::Bishop),
        piece.color,
    ));
    moves.push(Move::new(
        piece,
        to_pos,
        MoveType::PromotionCapture(PieceKind::Knight),
        piece.color,
    ));
    moves
}

pub fn capture_or_normal(
    board_info: BoardInfo,
    color: Color,
    piece: Piece,
    to_pos: (u8, u8),
    moves: &mut Vec<Move>,
) -> bool {
    if in_bounds(to_pos) {
        let nmv = Move::new(piece, to_pos, MoveType::Normal, piece.color);
        let cmv = Move::new(piece, to_pos, MoveType::Capture, piece.color);

        let to_square = board_info.get_square(to_pos);
        match to_square {
            Some(to_piece) if to_piece.color != color => {
                moves.push(cmv);
                return true;
            }
            None => {
                moves.push(nmv);
                return false;
            }
            _ => (),
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::board_info::BoardInfo;
    use crate::board::piece::{Piece, PieceKind};
    use crate::game::player::Color;
    use crate::rules::generate_pawn_moves;
    use crate::rules::r#move::{Move, MoveType};

}