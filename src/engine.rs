use crate::board::{Board, Colour};
use crate::player::Player;

pub struct Engine {
    light_player: Box<Player>,
    dark_player: Box<Player>,
    turn: Colour,
    board: Board,
    dark_move: bool,
    light_move: bool,
}

impl Engine {
    pub fn new(light_player: impl Player, dark_player: impl Player, board: Board) -> Engine {
        Engine {
            board,
            light_player: Box::new(light_player),
            dark_player: Box::new(dark_player),
            turn: Colour::Dark,
            dark_move: true,
            light_move: true,
        }
    }

    pub fn get_board(&self) -> &Board {
        return &self.board;
    }

    fn take_turn(&mut self) -> bool {
        let player = match self.turn {
            Colour::Light => &mut self.light_player,
            Colour::Dark => &mut self.dark_player,
        };
        let mut moves = self.board.get_moves(self.turn).peekable();
        let valid_move = moves.peek().is_some();
        if valid_move {
            let mov = player.get_move(&self.board, &mut moves);
            std::mem::drop(moves);
            if let Some(m) = mov {
                self.board.apply_move(m);
            }
        }
        match self.turn {
            Colour::Light => {
                self.light_move = valid_move;
                self.turn = Colour::Dark;
            }
            Colour::Dark => {
                self.dark_move = valid_move;
                self.turn = Colour::Light;
            }
        }
        return valid_move;
    }

    pub fn run_to_end(&mut self) {
        loop {
            self.take_turn();
            if !self.dark_move && !self.light_move {
                break;
            }
        }
    }
}
