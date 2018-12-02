use crate::board::Board;
use crate::board::Colour;
use crate::board::Move;
use crate::engine::Engine;
use crate::player::RandomPlayer;
use crate::Player;

#[derive(Clone)]
pub struct TreeNode {
    pub parent: i32,
    content: Board,
    pub mov: Option<Move>,
    pub wins: u32,
    pub plays: u32,
}

pub struct MonteCarloPlayer {
    pub cache: Vec<TreeNode>,
    pub leaves: Vec<u32>,
}
impl MonteCarloPlayer {
    pub fn new() -> MonteCarloPlayer {
        let mut cache = vec![];
        cache.reserve(5_000_000);
        let mut leaves = vec![];
        leaves.reserve(1_000_000);
        MonteCarloPlayer { cache, leaves }
    }

    pub fn build_tree(&mut self, b: &Board, depth: usize, team: Colour) {
        self.cache.clear();
        self.leaves.clear();
        let root = TreeNode {
            parent: -1,
            content: b.clone(),
            plays: 0,
            wins: 0,
            mov: None,
        };
        self.cache.push(root);
        self.attach_to_tree(0, depth, team);
    }

    pub fn attach_to_tree(&mut self, parent_index: usize, depth: usize, team: Colour) {
        let parent = self.cache[parent_index].content.clone();
        let children = parent.get_moves(team).map(|m| {
            let mut new_b = parent.clone();
            new_b.apply_move(m);
            TreeNode {
                parent: parent_index as i32,
                content: new_b,
                wins: 0,
                plays: 0,
                mov: Some(m),
            }
        });
        if depth == 0 {
            for c in children {
                self.cache.push(c);
                self.leaves.push((self.cache.len() - 1) as u32);
            }
        } else {
            for n in children {
                self.cache.push(n);
                let ind = self.cache.len() - 1;
                self.attach_to_tree(ind, depth - 1, team.opposite());
            }
        }
    }

    pub fn play_node(&mut self, node: usize, team: Colour) {
        let position = self.cache[node].content.clone();
        let light = RandomPlayer {
            rng: rand::thread_rng(),
        };
        let dark = RandomPlayer {
            rng: rand::thread_rng(),
        };
        let mut engine = Engine::new(dark, light, position);
        engine.run_to_end();
        let end = engine.get_board();
        let score = end.get_score();
        let has_win = match team {
            Colour::Dark => score.0 > score.1,
            Colour::Light => score.0 < score.0,
            // Ties are counted as losses
        };
        let win = if has_win { 1 } else { 0 };

        let mut cursor = node as i32;
        loop {
            self.cache[cursor as usize].plays += 1;
            self.cache[cursor as usize].wins += win;
            cursor = self.cache[cursor as usize].parent;
            if cursor == -1 {
                break;
            }
        }
    }
}

impl Player for MonteCarloPlayer {
    fn get_move(&mut self, _b: &Board, _availiable: &mut Iterator<Item = Move>) -> Option<Move> {
        return None;
    }
}
