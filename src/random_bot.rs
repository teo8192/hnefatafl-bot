use std::sync::Mutex;

use hnefatafl_core::{Board, CompactMove, HnefataflError, Move};

use rand::prelude::*;

use crate::bot::Bot;

pub struct RandomBot<R: Rng> {
    board: Board,
    rng: Mutex<R>,
}

impl RandomBot<ThreadRng> {
    pub fn new() -> RandomBot<ThreadRng> {
        RandomBot {
            board: Board::new(),
            rng: Mutex::new(rand::thread_rng()),
        }
    }

    pub fn with_board(board: Board) -> RandomBot<ThreadRng> {
        RandomBot {
            board,
            rng: Mutex::new(rand::thread_rng()),
        }
    }
}

impl<R: Rng> RandomBot<R> {
    pub fn with_rng(rng: R) -> RandomBot<R> {
        RandomBot {
            board: Board::new(),
            rng: Mutex::new(rng),
        }
    }

    pub fn set_rng<T: Rng>(&mut self, rng: T) -> RandomBot<T> {
        RandomBot {
            board: self.board.clone(),
            rng: Mutex::new(rng),
        }
    }
}

impl<R: Rng> Bot for RandomBot<R> {
    fn do_move(&mut self, mv: &Move) -> Result<CompactMove, HnefataflError> {
        self.board.do_move(mv)
    }

    fn get_move(&self) -> Move {
        let moves = self.board.available_moves();

        // select one of the moves:
        let n = self.rng.lock().unwrap().gen_range(0..moves.len());

        let selected_move = moves[n].clone();

        // return the selected move:
        selected_move
    }
}
