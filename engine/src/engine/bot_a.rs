use mcts::AsyncSearchOwned;

use mcts::{tree_policy::UCTPolicy, MCTSManager};

use super::Player;
use crate::engine::{MyEvaluator, MyMCTS, SnakeGame};

pub struct BotA {
    async_search: Option<AsyncSearchOwned<MyMCTS>>,
    color: [f32; 4],
    n_threads: usize,
    id: usize,
}

impl BotA {
    pub fn new(id: usize, n_threads: usize, color: [f32; 4]) -> Self {
        BotA {
            id,
            color,
            n_threads,
            async_search: None,
        }
    }
}

impl Player for BotA {
    fn think(&mut self, game: &SnakeGame) {
        let mut game = game.clone();
        game.set_player(self.id);

        let mcts = MCTSManager::new(game, MyMCTS, MyEvaluator, UCTPolicy::new(5.5), ());

        self.async_search = Some(mcts.into_playout_parallel_async(self.n_threads));
    }

    fn next_move(&mut self) -> crate::engine::Movement {
        let search = std::mem::replace(&mut self.async_search, None);
        let best_move = match search {
            Some(search) => {
                let mcts = search.halt();

                mcts.best_move()
            }
            None => None,
        };

        best_move.unwrap_or_else(|| {
            dbg!("WTFFF, answering random movement");
            rand::random()
        })
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
