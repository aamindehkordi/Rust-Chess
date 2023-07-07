use crate::model::board::Board;

pub trait Move {
    // Returns a vector of valid moves for the piece.
    fn get_valid_moves(&mut self, board: &Board) -> Vec<(usize, usize)>;

    // Attempts to execute a move. If the move is valid, it modifies the board
    // and returns `Ok(())`. If the move is not valid, it returns `Err` with an
    // error message.
    fn execute_move(&mut self, board: &mut Board, from: (usize, usize), to: (usize, usize)) -> Result<(), String>;

}
