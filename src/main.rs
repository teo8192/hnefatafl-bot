use hnefatafl_core::{Board, Turn};

mod random_bot;
mod bot;
mod choose_first_move;

use bot::Bot;

fn main() {
    let mut bot1 = random_bot::RandomBot::new();
    let mut bot2 = choose_first_move::ChooseFirstMoveBot::new();

    let mut board = Board::new();

    loop {
        let mv = if board.get_turn() == Turn::Black {
            bot1.get_move()
        } else {
            bot2.get_move()
        };

        let cm = board.do_move(&mv).unwrap();
        bot1.do_move(&mv).unwrap();
        bot2.do_move(&mv).unwrap();

        if board.is_game_over() {
            println!("Game over, {:?} won!", board.get_turn());
            break;
        }

        println!("{:?}", cm);
        println!("{}", board);
    }
}
