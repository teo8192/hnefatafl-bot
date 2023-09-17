use hnefatafl_core::{Board, Move};

pub struct Bot {
    board: Board,
}

impl Bot {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    pub fn do_move(&mut self, mv: &Move) {
        self.board.do_move(mv);
    }

    pub fn get_move(&self) -> Move {
        let moves = self.board.available_moves();

        // select one of the moves:
        let selected_move = moves[0].clone();

        // return the selected move:
        selected_move
    }
}
