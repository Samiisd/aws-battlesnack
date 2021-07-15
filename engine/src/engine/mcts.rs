use crate::engine::{Collision, DEFAULT_SNAKE_HEALTH};

use super::matrice::CellValue;
use super::{Movement, Snake, SnakeGame};
use mcts::{tree_policy::UCTPolicy, Evaluator, MCTS};
use ndarray::{Array1, Array2};
use std::collections::VecDeque;
use std::{usize, vec};

pub struct MyEvaluator;

impl MyEvaluator {
    fn expand_conquer_array(mut array: Array2<CellValue>, snakes: &[Snake]) -> Array1<i64> {
        let (height, width) = (array.shape()[0] as i32, array.shape()[1] as i32);
        let mut lengths: Array1<i64> = snakes.iter().map(|s| s.body().len() as i64).collect();

        let mut q : VecDeque<_> = snakes
            .iter()
            .enumerate()
            .map(|(id, s)| (id, *s.head()))
            .collect();

        while !q.is_empty() {
            let (id, pos) = q.pop_front().unwrap();
            lengths[id] += 1;

            let new_points = [
                Movement::Down,
                Movement::Up,
                Movement::Right,
                Movement::Left,
            ]
            .iter()
            .filter_map(|&m| {
                let p = pos.apply_mov(m);
                if (p.x < 0 || p.x >= width || p.y < 0 || p.y >= height)
                    || array[[p.y as usize, p.x as usize]] != 0
                {
                    None
                } else {
                    array[[p.y as usize, p.x as usize]] = (id + 1) as CellValue;
                    Some((id, p))
                }
            });

            q.extend(new_points.into_iter());
        }

        lengths
    }
}

impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = Array1<i64>;

    fn evaluate_new_state(
        &self,
        state: &SnakeGame,
        moves: &Vec<Movement>,
        _: Option<mcts::SearchHandle<MyMCTS>>,
    ) -> (Vec<()>, Self::StateEvaluation) {
        let snakes = state.board().snakes();

        let array = state.board().matrice().array().clone();

        let avg_len: i64 = (state
            .board()
            .alive_snakes()
            .map(|(_, s)| s.length() as f32)
            .sum::<f32>()
            / state.board().nb_snakes_alive() as f32)
            .round() as i64;

        let p_diff_len_with_mean: Array1<i64> = snakes
            .iter()
            .map(|s| -(s.length() as i64 - avg_len).abs())
            .collect();

        let p_health: Array1<i64> = snakes
            .iter()
            .map(|s| (s.health() as i64 - (DEFAULT_SNAKE_HEALTH as f32 / 2.).ceil() as i64))
            .collect();

        let p_area = Self::expand_conquer_array(array, snakes);
        let p_death: Array1<i64> = snakes
            .iter()
            .map(|s| if s.is_dead() { -100 } else { 0 })
            .collect();

        let mut p_collisions: Array1<i64> = Array1::zeros([snakes.len()]);
        state
            .board()
            .collisions()
            .iter()
            .flat_map(|c| match *c {
                Collision::Wall { id } => vec![(id, -100)],
                Collision::SelfBody { id } => vec![(id, -100)],
                Collision::OtherBody { id_1, id_2, .. } => vec![(id_1, -100), (id_2, 10)],
                Collision::HeadToHead {
                    src_length,
                    dst_length,
                    id_1,
                    id_2,
                    ..
                } => match src_length {
                    x if x == dst_length => vec![(id_1, -100), (id_2, -100)],
                    x if x > dst_length => vec![(id_1, 10), (id_2, -100)],
                    _ => vec![(id_1, -100), (id_2, 10)],
                },
            })
            .for_each(|(id, score)| {
                p_collisions[id] += score;
            });

        let p_total = p_area + p_collisions + p_death + p_health + p_diff_len_with_mean;

        (vec![(); moves.len()], p_total)
    }

    fn evaluate_existing_state(
        &self,
        _: &SnakeGame,
        existing_evaln: &Self::StateEvaluation,
        _: mcts::SearchHandle<MyMCTS>,
    ) -> Self::StateEvaluation {
        existing_evaln.clone()
    }

    fn interpret_evaluation_for_player(
        &self,
        evaluation: &Self::StateEvaluation,
        player: &mcts::Player<MyMCTS>,
    ) -> i64 {
        let score_player = evaluation[*player];
        let score_others = (evaluation.sum() - score_player) as f64;

        score_player - (0.5 * score_others) as i64
    }
}

#[derive(Default)]
pub struct MyMCTS;

impl MCTS for MyMCTS {
    type State = SnakeGame;
    type Eval = MyEvaluator;
    type NodeData = ();
    type ExtraThreadData = ();
    type TreePolicy = UCTPolicy;
    // type TranspositionTable = ApproxTable<Self>;
    type TranspositionTable = ();

    fn virtual_loss(&self) -> i64 {
        0
    }

    fn visits_before_expansion(&self) -> u64 {
        1
    }

    fn node_limit(&self) -> usize {
        std::usize::MAX
    }

    fn select_child_after_search<'a>(
        &self,
        children: &'a [mcts::MoveInfo<Self>],
    ) -> &'a mcts::MoveInfo<Self> {
        children.iter().max_by_key(|child| child.visits()).unwrap()
    }

    fn max_playout_length(&self) -> usize {
        1_000_000
    }

    fn on_backpropagation(
        &self,
        _evaln: &mcts::StateEvaluation<Self>,
        _handle: mcts::SearchHandle<Self>,
    ) {
    }

    fn cycle_behaviour(&self) -> mcts::CycleBehaviour<Self> {
        if std::mem::size_of::<Self::TranspositionTable>() == 0 {
            mcts::CycleBehaviour::Ignore
        } else {
            mcts::CycleBehaviour::UseCurrentEvalWhenCycleDetected
        }
    }
}
