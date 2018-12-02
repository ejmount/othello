#[macro_use]
extern crate lazy_static;

mod board;
mod engine;
mod monte;
mod player;

use self::board::*;
//use self::engine::Engine;
use self::player::*;
use crate::monte::MonteCarloPlayer;

fn main() {
    let mut p = MonteCarloPlayer::new();
    let b = Board::default();

    p.build_tree(&b, 4, Colour::Dark);
    println!("{:?} leaves, {:?} nodes", p.leaves.len(), p.cache.len());
    for l in p.leaves.clone() {
        p.play_node(l as usize, Colour::Dark);
    }
    println!("{:?} wins of {:?} plays", p.cache[0].wins, p.cache[0].plays);
    for n in p.cache.iter() {
        if n.parent != -1 && p.cache[n.parent as usize].parent == -1 {
            println!("{:?} {:?} {:?}", n.mov, n.wins, n.plays);
        }
    }
}
