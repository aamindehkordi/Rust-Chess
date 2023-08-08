pub mod r#move;

use crate::board::board_info::BoardInfo;
use crate::board::in_bounds;
use crate::board::piece::{Piece, PieceKind};
use crate::game::player::Color;
use crate::rules::r#move::{CastleType, Move, MoveType};
use std::cmp::{max, min};

pub fn generate_pawn_moves(board_info: BoardInfo, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new(); // Vector to store moves
    let pos = piece.position; // Get the position of the pawn
    let (x, y) = pos; // Get the x and y coordinates of the pawn
    let color = piece.color; // Get the color of the pawn
    let direction = match color {
        // Get the direction of the pawn
        Color::White => 1,
        Color::Black => -1,
    };

    // Normal move forward
    let forward_pos = (x, (y as i8 + direction) as u8); // Get the square in front of the pawn
    let _forward_square = board_info.get_square(forward_pos); // Get the piece at the square in front of the pawn
    if in_bounds(forward_pos) && board_info.get_square(forward_pos).is_none() {
        // Check if the square is in bounds and empty
        let mvs = if forward_pos.1 == 0 || forward_pos.1 == 7 {
            promotion_move(board_info.clone(), color, pos, forward_pos)
        } else {
            vec![Move::new(piece, forward_pos, MoveType::Normal, piece.color)]
        };
        moves.extend(mvs);
    }

    // Double move forward
    let double_forward_pos = (x, (y as i8 + 2 * direction) as u8);
    if (y == 1 || y == 6)
        && in_bounds(double_forward_pos)
        && board_info.get_square(forward_pos).is_none()
        && board_info.get_square(double_forward_pos).is_none()
    {
        moves.push(Move::new(
            piece,
            double_forward_pos,
            MoveType::DoublePawnPush,
            piece.color,
        ));
    }

    // Captures
    for &dx in [-1, 1].iter() {
        let capture_pos = ((x as i8 + dx) as u8, (y as i8 + direction) as u8);
        if in_bounds(capture_pos) {
            match board_info.get_square(capture_pos) {
                Some(piece) if piece.color != color => {
                    let mvs = if capture_pos.1 == 0 || capture_pos.1 == 7 {
                        promotion_attack_move(board_info.clone(), color, pos, capture_pos)
                    } else {
                        vec![Move::new(
                            piece,
                            capture_pos,
                            MoveType::Capture,
                            piece.color,
                        )]
                    };
                    moves.extend(mvs);
                }
                _ => (),
            }
        }
    }

    // En Passant
    if let Some(last_move) = board_info.move_history.last() {
        if let MoveType::DoublePawnPush = last_move.move_type {
            if last_move.to.1 == y && (last_move.to.0 as i8 - x as i8).abs() == 1 {
                let en_passant_move = (last_move.to.0, (last_move.to.1 as i8 + direction) as u8);
                moves.push(Move::new(
                    piece,
                    en_passant_move,
                    MoveType::EnPassant,
                    piece.color,
                ));
            }
        }
    }

    moves
}

pub fn generate_king_moves(board_info: BoardInfo, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let pos = piece.position;
    let color = piece.color;
    let (x, y) = pos;
    let piece = board_info.get_square(pos).unwrap();

    // The king can move in 8 directions: up, down, left, right, and the 4 diagonals.
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(dx, dy) in directions.iter() {
        let to_pos = ((x as i8 + dx) as u8, (y as i8 + dy) as u8);
        capture_or_normal(board_info.clone(), color, piece, to_pos, &mut moves);
    }

    // Castle move generation
    // 1. The king hasn't moved before
    if !piece.has_moved {
        // 2. The king is not currently in check
        if !board_info.is_in_check(color) {
            // 3. The rook with which the king is castling hasn't moved before
            let rook_positions = if color == Color::White {
                [(0, 7), (7, 7)]
            } else {
                [(0, 0), (7, 0)]
            };
            for &rook_pos in &rook_positions {
                let rook = board_info.get_square(rook_pos);
                if rook.is_some() && !rook.unwrap().has_moved {
                    // 4. There are no pieces between the king and the rook
                    let min_x = min(pos.0, rook_pos.0) as usize;
                    let max_x = max(pos.0, rook_pos.0) as usize;
                    if (min_x..=max_x).all(|x| {
                        board_info.get_square((x as u8, pos.1)).is_none()
                            || (x as u8, pos.1) == pos
                            || (x as u8, pos.1) == rook_pos
                    }) {
                        // 5. The king does not pass through a square that is attacked by an enemy piece
                        let castle_through = if rook_pos.0 == 0 {
                            [(2, pos.1), (3, pos.1)]
                        } else {
                            [(5, pos.1), (6, pos.1)]
                        };
                        if castle_through
                            .iter()
                            .all(|&pos| !board_info.is_attacked(pos, color))
                        {
                            // 6. The king is not in check after the castle move
                            let to_pos = if rook_pos.0 == 0 {
                                (2, pos.1)
                            } else {
                                (6, pos.1)
                            };
                            let mv = Move::new(
                                piece,
                                to_pos,
                                MoveType::Castle(if rook_pos.0 == 0 {
                                    CastleType::QueenSide
                                } else {
                                    CastleType::KingSide
                                }),
                                piece.color,
                            );
                            if board_info.is_valid(&mv) {
                                moves.push(mv);
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}

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
    let _piece = board_info.get_square(from_pos).unwrap();

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
