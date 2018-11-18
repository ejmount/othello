use crate::board::Board;
use crate::board::Move;

pub trait Player {
    fn get_move(&mut self, b: &Board, availiable: &mut Iterator<Item = Move>) -> Option<Move>;
}

pub struct BasicPlayer {}

impl Player for BasicPlayer {
    fn get_move(&mut self, _b: &Board, availiable: &mut Iterator<Item = Move>) -> Option<Move> {
        let m = availiable.next();
        /*println!("Applying {:?} to...", m);
        println!("{}", b);*/
        return m;
    }
}
