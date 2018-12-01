#[macro_use]
extern crate lazy_static;

mod board;
mod engine;
mod player;

use self::board::*;
use self::engine::Engine;
use self::player::*;

fn main() {
    for _ in 0..10000 {
        let b = Board::default();

        let light = Human {
            //rng: rand::thread_rng(),
        };
        let dark = RandomPlayer {
            rng: rand::thread_rng(),
        };

        let mut e = Engine::new(light, dark, b);
        e.run_to_end();
        //print!("End: {}", e.get_board());
    }
}
