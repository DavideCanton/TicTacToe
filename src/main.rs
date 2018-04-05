#[macro_use]
extern crate itertools;

#[macro_use]
mod board;

use board::global_board::GlobalBoard;
use board::common::{Position, Player, Board};

fn main() {
    let mut board = GlobalBoard::new();

    for i in 0..9 {
        board.set_pos(pos!(0, i), Some(Player::X));
    }
}



