use crate::board::Board;
use crate::board::Colour;
use crate::board::Move;
use crate::board::Position;
use crate::board::BOARD_SIZE;
use rand::seq::IteratorRandom;
use rand::Rng;

pub trait Player: 'static {
    fn get_move(&mut self, b: &Board, availiable: &mut Iterator<Item = Move>) -> Option<Move>;
}

pub struct BasicPlayer {}

impl Player for BasicPlayer {
    fn get_move(&mut self, _b: &Board, availiable: &mut Iterator<Item = Move>) -> Option<Move> {
        availiable.next()
    }
}

pub struct RandomPlayer<R: Rng + 'static> {
    pub rng: R,
}
impl<R: Rng + 'static> Player for RandomPlayer<R> {
    fn get_move(&mut self, _b: &Board, availiable: &mut Iterator<Item = Move>) -> Option<Move> {
        availiable.choose(&mut self.rng)
    }
}

pub struct Human {}
impl Human {
    const MAP: [char; 33] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', /*'D',*/ 'E',
        'F', 'G', 'H', 'I', 'J', 'K', /*'L',*/ 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ]; // This should be enough so long as the board is only 8x8. Ambigious entries removed.
}

impl Player for Human {
    fn get_move(&mut self, b: &Board, mvs: &mut Iterator<Item = Move>) -> Option<Move> {
        use std::io::{BufRead, Write};

        let numbered_moves: Vec<_> = mvs.enumerate().collect();
        let moves_positions: Vec<_> = numbered_moves
            .into_iter()
            .flat_map(|(num, mv)| {
                let end_maybe = b.get_empty_at_end_of_line(mv.origin, mv.dir);
                match end_maybe {
                    Some(end) => Some((num, mv, end)),
                    None => None,
                }
            }).collect();

        let mut board_display = [['_'; BOARD_SIZE]; BOARD_SIZE];

        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let p = Position::new(r, c).unwrap();
                let val = b.get(p);

                match val {
                    Some(Colour::Light) => board_display[r][c] = 'L',
                    Some(Colour::Dark) => board_display[r][c] = 'D',
                    None => {}
                }
            }
        }

        for &(num, _, position) in moves_positions.iter() {
            let r = position.get_x();
            let c = position.get_y();
            board_display[r][c] = Self::MAP[num];
        }

        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                print!("{}", board_display[r][c]);
            }
            println!("");
        }

        loop {
            print!("Move number #: ");
            std::io::stdout().flush().ok();
            let mut buffer = String::new();
            std::io::stdin().lock().read_line(&mut buffer).unwrap();
            let input_char = buffer.trim().chars().next();
            if let None = input_char {
                continue;
            }
            let input_char = input_char.unwrap();
            println!("");
            if input_char == 'Q' || input_char == 'q' {
                panic!("");
            }

            match Self::MAP.iter().position(|&c| c == input_char) {
                Some(index) => return Some(moves_positions[index as usize].1),
                None => println!("Didn't understand \"{}\"", buffer.trim()),
            }
        }
    }
}
