use mcts::AsyncSearchOwned;

use mcts::{MCTSManager, transposition_table::ApproxTable, tree_policy::UCTPolicy};

use crate::engine::{MyEvaluator, MyMCTS, SnakeGame};
use super::Player;

pub struct BotA {
    async_search: Option<AsyncSearchOwned<MyMCTS>>,
    color: [f32; 4],
    n_threads: usize,
}

impl BotA {
    pub fn new(n_threads: usize, color: [f32; 4]) -> Self {
        BotA {
            color,
            n_threads,
            async_search: None,
        }
    }
}

impl Player for BotA {
    fn think(&mut self, game: &SnakeGame) {
        let mcts = MCTSManager::new(
            game.clone(),
            MyMCTS,
            MyEvaluator,
            UCTPolicy::new(5.0),
            ApproxTable::new(1024),
        );

        self.async_search = Some(mcts.into_playout_parallel_async(self.n_threads));
    }

    fn next_move(&mut self) -> crate::engine::Movement {
        let search = std::mem::replace(&mut self.async_search, None);
        let best_move = if let Some(search) = search {
            let mcts = search.halt();
            dbg!(mcts.principal_variation_info(10));
            mcts.best_move().map(|movs| movs[0])
        } else {
            None
        };

        best_move.unwrap_or_else(|| {
            dbg!("WTFFF, answering random movement");
            rand::random()
        })
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn register_key_event(&mut self, _: piston_window::Button) { () }    
}
