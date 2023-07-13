use crate::model::board::Board;
use crate::model::moves::r#move::{Move, MoveType, CastleType};
use crate::model::pieces::piece::{Color, Piece, PieceType};

pub struct MoveGenerator { }

impl MoveGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_moves(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        match piece.get_type() {
            PieceType::Pawn => self.generate_moves_for_pawn(piece, board),
            PieceType::Rook => self.generate_moves_for_rook(piece, board),
            PieceType::Knight => self.generate_moves_for_knight(piece, board),
            PieceType::Bishop => self.generate_moves_for_bishop(piece, board),
            PieceType::Queen => self.generate_moves_for_queen(piece, board),
            PieceType::King => self.generate_moves_for_king(piece, board),
        }
    }

    pub fn get_move(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> Move {
        let piece_type = piece.get_type();
        let mut move_type = MoveType::Invalid;

        match piece_type {
            PieceType::Pawn => move_type = self.get_move_type_for_pawn(from, to, piece, board),
            PieceType::Rook => move_type = self.get_move_type_for_rook(from, to, piece, board),
            PieceType::Knight => move_type = self.get_move_type_for_knight(from, to, piece, board),
            PieceType::Bishop => move_type = self.get_move_type_for_bishop(from, to, piece, board),
            PieceType::Queen => move_type = self.get_move_type_for_queen(from, to, piece, board),
            PieceType::King => move_type = self.get_move_type_for_king(from, to, piece, board),
        }

        let mut mv = Move::new(move_type.clone(), from.clone(), to.clone());
        if move_type != MoveType::Invalid {
            mv.set_valid(true);
        }
        mv
    }

    fn get_move_type_for_pawn(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let mut move_type = MoveType::Invalid;
        let color = piece.get_color();
        let (fx, fy) = *from;
        let (tx, ty) = *to;

        // Normal move: moving one square forward
        let normal_move = match color {
            Color::Black => fx == tx + 1 && fy == ty,
            Color::White => fx + 1 == tx && fy == ty,
        };
        if normal_move {
            return MoveType::Normal;
        }

        // Double push: moving two squares forward on the pawn's first move
        let double_push = match color {
            Color::Black => fx == tx + 2 && fy == ty && fx == 6,
            Color::White => fx + 2 == tx && fy == ty && fx == 1,
        };
        if double_push {
            return MoveType::DoublePush;
        }

        // Capture: taking an opponent's piece diagonally
        let capture = match color {
            Color::Black => fx == tx + 1 && (fy as i32 - ty as i32).abs() == 1,
            Color::White => fx + 1 == tx && (fy as i32 - ty as i32).abs() == 1,
        };
        if capture {
            if let Some(dest_piece) = board.get_piece(*to) {
                if dest_piece.get_color() != color {
                    return MoveType::Capture;
                }
            }
        }

        // Promotion: reaching the end of the board
        let promotion = match color {
            Color::Black => fx == 1,
            Color::White => fx == 6,
        };
        if promotion {
            // Assume that the pawn will be promoted to a queen.
            // In a full-featured chess game, the player should be asked what piece to promote the pawn to.
            return MoveType::Promotion(PieceType::Queen);
            todo!("Implement promotion to other pieces")
        }
        return move_type;

        // En passant: capturing an opponent's pawn in passing
        // This is quite complex and would require access to the game's history to check if the last move was a pawn double push.
        // It is omitted here for brevity.
        todo!("Implement en passant")
    }


    fn generate_moves_for_pawn(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = piece.get_position().clone();
        let color = piece.get_color();
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        // Moving one square forward
        let move_one_forward = ((pos.0 as i32 + direction) as usize, pos.1);

        // Moving two squares forward on the pawn's first move
        let move_two_forward = ((pos.0 as i32 + 2 * direction) as usize, pos.1);

        // Capturing diagonally
        let capture_moves = [((pos.0 as i32 + direction) as usize, (pos.1 as i32 + direction) as usize), ((pos.0 as i32 + direction) as usize, (pos.1 as i32 - direction) as usize)];
        // wrong [(pos.0 + 1, (pos.1 as i32 + direction) as usize), (pos.0 - 1, (pos.1 as i32 + direction) as usize)];

        if is_valid_pos(move_one_forward) && board.get_piece(move_one_forward).is_none() {
            moves.push(normal_move(piece, move_one_forward));
        }

        // Moving two squares forward on the pawn's first move
        if piece.get_moves().is_empty() && is_valid_pos(move_two_forward) && board.get_piece(move_two_forward).is_none() {
            moves.push(double_push_move(piece, move_two_forward));
        }

        // Capturing diagonally
        for &cmv in &capture_moves {
            // check if the position is valid
            if !is_valid_pos(cmv) {
                continue;
            }
            if let Some(dest_piece) = board.get_piece(cmv) {
                if dest_piece.get_color() != piece.get_color() {
                    moves.push(capture_move(piece, cmv));
                }
            }
        }

        // En passant and Promotion will be handled in get_move_type_for_pawn()

        moves
    }

    fn get_move_type_for_rook(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let (fx, fy) = *from;
        let (tx, ty) = *to;
        let mut move_type = MoveType::Invalid;
        let color = piece.get_color();

        // Moving along the same rank (row) or file (column)
        if fx == tx || fy == ty {
            // Check if there are any pieces between the source and destination squares
            let min_x = fx.min(tx);
            let max_x = fx.max(tx);
            let min_y = fy.min(ty);
            let max_y = fy.max(ty);

            for x in (min_x+1)..max_x {
                if board.get_piece((x, fy)).is_some() {
                    return move_type;
                }
            }

            for y in (min_y+1)..max_y {
                if board.get_piece((fx, y)).is_some() {
                    return move_type;
                }
            }

            // If the destination square is occupied by an enemy piece, it's a capture move
            if let Some(dest_piece) = board.get_piece(*to) {
                if dest_piece.get_color() != color {
                    return MoveType::Capture;
                }
            }

            return MoveType::Normal;
        }

        move_type
    }

    fn generate_moves_for_rook(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = piece.get_position().clone();

        // Generate moves along each direction: up, down, left, right
        let directions = piece.get_directions();

        for &(dx, dy) in directions {
            let mut new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);

            while is_valid_pos(new_pos) {
                if let Some(dest_piece) = board.get_piece(new_pos) {
                    if dest_piece.get_color() == piece.get_color() {
                        // Can't capture our own piece and can't move any further in this direction
                        break;
                    } else {
                        // Capture move
                        moves.push(capture_move(piece, new_pos));
                        // Can't move any further in this direction after capturing a piece
                        break;
                    }
                } else {
                    // Normal move
                    moves.push(normal_move(piece, new_pos));
                }

                new_pos = ((new_pos.0 as i32 + dx) as usize, (new_pos.1 as i32 + dy) as usize);
            }
        }

        moves
    }

    fn get_move_type_for_knight(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let mut move_type = MoveType::Invalid; // Create a variable to store the move type
        let directions = piece.get_directions(); // Get the directions in which the piece can move
        let new_pos = (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32); // Calculate the new position

        // Loop through the directions
        for &direction in directions {
            if direction == new_pos { // Check if the new position is in the directions
                move_type = MoveType::Normal; // Set the move type to normal
                break; // Break out of the loop
            }
        }

        if move_type == MoveType::Invalid { // Check if the move type hasn't changed
            if let Some(dest_piece) = board.get_piece(to.clone()) { // Get the piece at the new position
                if dest_piece.get_color() != piece.get_color() { // Check if the piece at the new position is of a different color
                    move_type = MoveType::Capture; // Set the move type to capture
                }
            }
        }

        move_type // Return the move type
    }

    fn generate_moves_for_knight(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new();

        let directions = piece.get_directions();

        let cur_pos = piece.get_position();
        for &direction in directions {
            let new_pos = calculate_new_pos(cur_pos, direction);

            if is_valid_pos(new_pos) {
                let dest_piece = board.get_piece(new_pos);

                if let Some(dest_piece) = dest_piece {
                    if dest_piece.get_color() != piece.get_color() {
                        moves.push(capture_move(piece, new_pos));
                    }
                } else {
                    moves.push(normal_move(piece, new_pos));
                }
            }
        }

        moves
    }

    fn get_move_type_for_bishop(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let mut move_type = MoveType::Invalid;
        let (fx, fy) = *from;
        let (tx, ty) = *to;

        // The Bishop moves diagonally, so if the move is not a diagonal, it's invalid
        if (fx as i32 - tx as i32).abs() != (fy as i32 - ty as i32).abs() {
            return move_type;
        }

        // Now we need to check if the path between the source and destination is clear.
        let dx = if tx > fx { 1 } else { -1 };
        let dy = if ty > fy { 1 } else { -1 };
        let mut x = fx as i32 + dx;
        let mut y = fy as i32 + dy;

        while (x as usize, y as usize) != *to {
            if board.get_piece((x as usize, y as usize)).is_some() {
                // There's a piece in the path
                return move_type;
            }
            x += dx;
            y += dy;
        }

        // If the destination square is occupied by an enemy piece, this is a capture
        if let Some(dest_piece) = board.get_piece(*to) {
            if dest_piece.get_color() != piece.get_color() {
                return MoveType::Capture;
            }
        }

        MoveType::Normal
    }

    fn generate_moves_for_bishop(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = piece.get_position().clone();

        let directions = piece.get_directions();

        for direction in directions.iter() {
            let mut next_position = ((pos.0 as i32 + direction.0) as usize, (pos.1 as i32 + direction.1) as usize);

            // While the position is valid and not occupied, add it as a potential move
            while is_valid_pos(next_position) && board.get_piece(next_position).is_none() {
                moves.push(normal_move(piece, next_position));
                next_position = ((next_position.0 as i32 + direction.0) as usize, (next_position.1 as i32 + direction.1) as usize);
            }

            // If the position is occupied by an enemy piece, add it as a potential capture move
            if is_valid_pos(next_position) {
                if let Some(dest_piece) = board.get_piece(next_position) {
                    if dest_piece.get_color() != piece.get_color() {
                        moves.push(capture_move(piece, next_position));
                    }
                }
            }
        }

        moves
    }



    fn generate_moves_for_queen(&self, piece: &Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        todo!("Implement generate_moves_for_queen")
    }

    fn get_move_type_for_queen(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        todo!("Implement get_move_type_for_queen")
    }


    fn get_move_type_for_king(&self, from: &(usize, usize), to: &(usize, usize), piece: &Box<dyn Piece>, board: &Board) -> MoveType {
        let mut move_type = MoveType::Invalid;
        let (fx, fy) = *from;
        let (tx, ty) = *to;

        // Normal move: moving one square in any direction
        let dx = (fx as i32 - tx as i32).abs();
        let dy = (fy as i32 - ty as i32).abs();
        if dx <= 1 && dy <= 1 {
            return MoveType::Normal;
        }

        // Castling: moving two squares towards a rook on its initial square,
        // and then the rook moves to the square the king skipped over.
        if dy == 2 && dx == 0 && piece.get_moves().is_empty() {
            // Assuming here that the Rook's initial position is the corner of the board.
            // You'll need to check if the squares between the King and the Rook are empty and no square the king crosses is threatened.
            if fy < ty && board.get_piece((fx, 7)).map_or(false, |p| p.get_type() == PieceType::Rook && p.get_moves().is_empty()) {
                return MoveType::Castle(CastleType::Kingside);
            } else if fy > ty && board.get_piece((fx, 0)).map_or(false, |p| p.get_type() == PieceType::Rook && p.get_moves().is_empty()) {
                return MoveType::Castle(CastleType::Queenside);
            }
        }

        move_type
    }

    fn generate_moves_for_king(&self, piece: &mut Box<dyn Piece>, board: &mut Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = piece.get_position().clone();

        // King can move in all 8 directions, but only one square
        let all_directions = piece.get_directions();

        for &dir in all_directions.iter() {
            let new_pos = ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize);
            if is_valid_pos(new_pos) {
                if let Some(dest_piece) = board.get_piece(new_pos) {
                    if dest_piece.get_color() != piece.get_color() {
                        moves.push(capture_move(piece, new_pos));
                    }
                } else {
                    moves.push(normal_move(piece, new_pos));
                }
            }
        }

        // Castling
        if piece.get_moves().is_empty() {
            // Assuming here that the Rook's initial position is the corner of the board.
            // You'll need to check if the squares between the King and the Rook are empty and no square the king crosses is threatened.
            let kingside_pos = (pos.0, 7);
            let queenside_pos = (pos.0, 0);

            if board.get_piece(kingside_pos).map_or(false, |p| p.get_type() == PieceType::Rook && p.get_moves().is_empty()) {
                moves.push(castling_move(piece, (pos.0, pos.1 + 2)));
            }

            if board.get_piece(queenside_pos).map_or(false, |p| p.get_type() == PieceType::Rook && p.get_moves().is_empty()) {
                moves.push(castling_move(piece, (pos.0, pos.1 - 2)));
            }
        }

        moves
    }
}

fn calculate_new_pos(p0: &(usize, usize), p1: (i32, i32)) -> (usize, usize) {
    let x = p0.0 as i32 + p1.0;
    let y = p0.1 as i32 + p1.1;

    (x as usize, y as usize)
}

fn is_valid_pos(pos: (usize, usize)) -> bool {
    pos.0 < 8 && pos.1 < 8
}

fn double_push_move(piece: &Box<dyn Piece>, new_pos: (usize, usize)) -> Move {
    let mut mv = Move::new(MoveType::DoublePush, piece.get_position().clone(), new_pos);
    mv.set_valid(true);
    mv
}

fn normal_move(piece: &Box<dyn Piece>, new_pos: (usize, usize)) -> Move {
    let mut mv = Move::new(MoveType::Normal, piece.get_position().clone(), new_pos);
    mv.set_valid(true);
    mv
}

fn capture_move(piece: &Box<dyn Piece>, new_pos: (usize, usize)) -> Move {
    let mut mv = Move::new(MoveType::Capture, piece.get_position().clone(), new_pos);
    mv.set_valid(true);
    mv
}

fn castling_move(piece: &Box<dyn Piece>, new_pos: (usize, usize)) -> Move {
    if new_pos.1 > piece.get_position().1 { // if new_pos is on this piece
        Move::new(MoveType::Castle(CastleType::Kingside), piece.get_position().clone(), new_pos)
    } else {
        Move::new(MoveType::Castle(CastleType::Queenside), piece.get_position().clone(), new_pos)
    }
}