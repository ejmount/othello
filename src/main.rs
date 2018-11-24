mod board;
mod engine;
mod player;

use self::board::*;
use self::engine::Engine;
use self::player::*;

fn main() {
    let b = Board::default();

    let light = BasicPlayer {};
    let dark = Human {};

    let mut e = Engine::new(light, dark, b);
    e.run_to_end();
    print!("End: {}", e.get_board());
}
