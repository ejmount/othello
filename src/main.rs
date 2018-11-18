mod board;
mod engine;

use self::board::*;

fn main() {
    let mut b = Board::default();

    let m = b.get_moves(Colour::Light).next().unwrap();
    b.apply_move(m);
    println!("{}", b);
    for i in b.get_moves(Colour::Light) {
        println!("{:?}", i);
    }
}
