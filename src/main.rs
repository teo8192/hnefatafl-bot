use hnefatafl_core::{Board, Turn};

mod bot;

fn main() {
    let mut bot1 = bot::Bot::new(hnefatafl_core::Board::new());
    let mut bot2 = bot::Bot::new(hnefatafl_core::Board::new());

    let mut board = Board::new();

    loop {
        let mv = if board.get_turn() == Turn::Black {
            bot1.get_move()
        } else {
            bot2.get_move()
        };

        println!("Move: {:?}", mv);

        board.do_move(&mv);
        bot1.do_move(&mv);
        bot2.do_move(&mv);

        println!("{}", board);
    }
}
