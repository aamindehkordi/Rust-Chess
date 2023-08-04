pub mod r#move;

use crate::board::piece::PieceKind;
use crate::board::Position;
use crate::game::player::Color;
use crate::game::{is_attacked_not_bb, Game};
use crate::rules::r#move::{CastleType, Move, MoveType};
use std::cmp::{max, min};

pub fn generate_pawn_moves(game: Game, pos: Position, color: Color) -> Vec<Move> {
    let mut moves = Vec::new(); // Vector to store moves
    let (x, y) = pos; // Get the position of the pawn
    let piece = game.board.get(pos).unwrap(); // Get the piece at the given position
    let direction = match color {
        // Get the direction of the pawn
        Color::White => 1,
        Color::Black => -1,
    };

    // Normal move forward
    let forward_pos = (x, (y as i8 + direction) as u8); // Get the square in front of the pawn
    let _forward_square = game.board.get(forward_pos); // Get the piece at the square in front of the pawn
    if in_bounds(forward_pos) && game.board.get(forward_pos).is_none() {
        // Check if the square is in bounds and empty
        let mvs = if forward_pos.1 == 0 || forward_pos.1 == 7 {
            promotion_move(game.clone(), color, pos, forward_pos)
        } else {
            vec![Move::new(piece, forward_pos, MoveType::Normal, piece.color)]
        };
        moves.extend(mvs);
    }

    // Double move forward
    let double_forward_pos = (x, (y as i8 + 2 * direction) as u8);
    let _double_forward_square = game.board.get(double_forward_pos);
    if (y == 1 || y == 6)
        && in_bounds(double_forward_pos)
        && game.board.get(forward_pos).is_none()
        && game.board.get(double_forward_pos).is_none()
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
            match game.board.get(capture_pos) {
                Some(piece) if piece.color != color => {
                    let mvs = if capture_pos.1 == 0 || capture_pos.1 == 7 {
                        promotion_attack_move(game.clone(), color, pos, capture_pos)
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
    if let Some(last_move) = game.game_state.move_history.last() {
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

pub fn generate_king_moves(game: Game, pos: Position, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let gs = &game.game_state;
    let (x, y) = pos;
    let piece = game.board.get(pos).unwrap();

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
        capture_or_normal(game.clone(), color, pos, to_pos, &mut moves);
    }

    // Castle move generation
    // 1. The king hasn't moved before
    if !piece.has_moved {
        // 2. The king is not currently in check
        if !gs.is_in_check(color) {
            // 3. The rook with which the king is castling hasn't moved before
            let rook_positions = if color == Color::White {
                [(0, 7), (7, 7)]
            } else {
                [(0, 0), (7, 0)]
            };
            for &rook_pos in &rook_positions {
                let rook = game.board.get(rook_pos);
                if rook.is_some() && !rook.unwrap().has_moved {
                    // 4. There are no pieces between the king and the rook
                    let min_x = min(pos.0, rook_pos.0) as usize;
                    let max_x = max(pos.0, rook_pos.0) as usize;
                    if (min_x..=max_x).all(|x| {
                        game.board.get((x as u8, pos.1)).is_none()
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
                            .all(|&pos| !is_attacked_not_bb(game.clone(), pos, color))
                        {
                            // 6. The king is not in check after the castle move
                            let to_pos = if rook_pos.0 == 0 {
                                (2, pos.1)
                            } else {
                                (6, pos.1)
                            };
                            let mut temp = game.clone();
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
                            temp.board.make_move(mv.clone());
                            if !gs.is_in_check(color) {
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
pub fn generate_knight_moves(game: Game, from_pos: Position, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let (x, y) = from_pos;
    let _piece = game.board.get(from_pos).unwrap();

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
        capture_or_normal(game.clone(), color, from_pos, to_pos, &mut moves);
    }
    moves
}

// Pushes all promotion piece types moves to the list of moves
pub fn promotion_move(game: Game, _color: Color, fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
    let mut moves = vec![];
    let piece = game.board.get(fmv).unwrap();
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
pub fn promotion_attack_move(game: Game, _color: Color, fmv: (u8, u8), pmv: (u8, u8)) -> Vec<Move> {
    let mut moves = vec![];
    let piece = game.board.get(fmv).unwrap();
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
    game: Game,
    color: Color,
    from_pos: (u8, u8),
    to_pos: (u8, u8),
    moves: &mut Vec<Move>,
) -> bool {
    let piece = game.board.get(from_pos).unwrap();
    if in_bounds(to_pos) {
        let to_square = game.board.get(to_pos);
        match to_square {
            Some(to_piece) if to_piece.color != color => {
                moves.push(Move::new(piece, to_pos, MoveType::Capture, piece.color));
                return true;
            }
            None => {
                moves.push(Move::new(piece, to_pos, MoveType::Normal, piece.color));
                return false;
            }
            _ => (),
        }
    }
    true
}

// Function to generate all legal moves for a sliding piece at a given position
pub fn generate_sliding_move(game: Game, from_pos: Position, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let piece = game.board.get(from_pos).unwrap();
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

            if capture_or_normal(game.clone(), color, from_pos, to_pos, &mut moves) {
                break;
            }
            distance += 1;
        }
    }
    moves
}

pub fn in_bounds(pos: Position) -> bool {
    pos.0 < 8 && pos.1 < 8
}



