use hnefatafl_core::{Board, CompactMove, HnefataflError, Move};

use crate::bot::Bot;

pub struct ChooseFirstMoveBot {
    board: Board,
}

impl ChooseFirstMoveBot {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn with_board(board: Board) -> Self {
        Self { board }
    }
}

impl Bot for ChooseFirstMoveBot {
    fn do_move(&mut self, mv: &Move) -> Result<CompactMove, HnefataflError> {
        self.board.do_move(mv)
    }

    fn get_move(&self) -> Move {
        let moves = self.board.available_moves();

        if moves.is_empty() {
            panic!("ChooseFirstMoveBot: No moves available!");
        }

        // select one of the moves:
        let selected_move = moves[0].clone();

        // return the selected move:
        selected_move
    }

    fn reset(&mut self) {
        self.board = Board::new();
    }

    fn evaluate_board(&self) -> f32 {
        0.0
    }
}
