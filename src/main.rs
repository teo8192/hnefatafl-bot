use hnefatafl_core::{Board, Turn};

mod random_bot;
mod bot;

use bot::Bot;

fn main() {
    let mut bot1 = random_bot::RandomBot::new();
    let mut bot2 = random_bot::RandomBot::new();

    let mut board = Board::new();

    loop {
        let mv = if board.get_turn() == Turn::Black {
            bot1.get_move()
        } else {
            bot2.get_move()
        };

        // println!("Move: {:?}", mv);

        let cm = board.do_move(&mv).unwrap();
        bot1.do_move(&mv);
        bot2.do_move(&mv);

        println!("{:?}", cm);
        println!("{}", board);
    }
}
