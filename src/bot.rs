use hnefatafl_core::{CompactMove, HnefataflError, Move};

pub trait Bot {
    fn do_move(&mut self, mv: &Move) -> Result<CompactMove, HnefataflError>;
    fn get_move(&self) -> Move;

    fn evaluate_board(&self) -> f32 {
        0.0
    }
}
