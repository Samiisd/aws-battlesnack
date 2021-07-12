use crate::engine::Movement;
use std::collections::HashMap;

use itertools::Itertools;
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
    pub fn new(n_threads: usize, color: [f32; 4]) -> Self {
        BotA {
            color,
            n_threads,
            async_search: None,
            // FIXME: id shouldn't be hardcoded
            id: 0,
        }
    }
}

impl Player for BotA {
    fn think(&mut self, game: &SnakeGame) {
        let mcts = MCTSManager::new(
            game.clone(),
            MyMCTS,
            MyEvaluator,
            UCTPolicy::new(1.5),
            (),
            // ApproxTable::new(1024),
        );

        self.async_search = Some(mcts.into_playout_parallel_async(self.n_threads));
    }

    fn next_move(&mut self) -> crate::engine::Movement {
        let search = std::mem::replace(&mut self.async_search, None);
        let best_move = match search {
            Some(search) => {
                let mcts = search.halt();

                let best_moves = mcts
                    .tree()
                    .root_node()
                    .moves()
                    .filter(|m| m.visits() > 0)
                    .map(|m| {
                        (
                            m.get_move()[self.id],
                            m.sum_rewards() as f64 / m.visits() as f64,
                        )
                    })
                    .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
                    .take(50);

                let moves_eval = best_moves.fold(HashMap::new(), |mut acc, (m, score)| {
                    let acc_score = acc.entry(m).or_insert_with(Vec::new);
                    acc_score.push(score);
                    acc
                });

                dbg!(mcts.tree().root_state().board().matrice().array());
                dbg!(&moves_eval);

                let b: HashMap<Movement, f64> = moves_eval
                    .into_iter()
                    .map(|(m, v)| (m, v.iter().sum::<f64>() / v.len() as f64))
                    // .map(|(m, v)|
                    // (m, v.into_iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap()))
                    .collect();

                dbg!(&b);

                b.into_iter()
                    // .map(|(m, v)| (m, v.iter().sum::<f64>() / v.len() as f64))
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(m, _)| m)
            }
            None => None,
        };

        let best_move = best_move.unwrap_or_else(|| {
            dbg!("WTFFF, answering random movement");
            rand::random()
        });

        dbg!(&best_move);

        best_move
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn register_key_event(&mut self, _: piston_window::Button) {}
}
