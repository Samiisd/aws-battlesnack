#![feature(hash_drain_filter)]
#![feature(deque_range)]
mod engine;

extern crate ndarray;

use crate::engine::{Board, Movement, MyEvaluator, MyMCTS, Point, Snake, SnakeGame};
use mcts::transposition_table::ApproxTable;
use mcts::tree_policy::UCTPolicy;
use mcts::MCTSManager;

fn create_board() -> Board {
    let snakes = [
        Point { x: 0, y: 1 },
        Point { x: 2, y: 3 },
        Point { x: 3, y: 1 },
        Point { x: 3, y: 2 },
        Point { x: 3, y: 3 },
        // Point { x: 6, y: 2 },
        // Point { x: 7, y: 2 },
        // Point { x: 8, y: 1 },
        // Point { x: 8, y: 3 },
        // Point { x: 8, y: 4 },
    ]
    .iter()
    .map(|&p| Snake::new(p))
    .collect();

    Board::new(4, 4, snakes)
}

fn benchmark_n_snakes(snakes: Vec<Snake>) {
    let n_snakes = snakes.len();

    let mut board = Board::new(21, 21, snakes.clone());
    for _ in 0..100 {
        if board.alive_snakes().count() > 0 {
            board.step((0..n_snakes).into_iter().map(|_| rand::random()).collect());
        }
    }
}

pub fn benchmark_engine_10_snakes() {
    let snakes = [
        Point { x: 2, y: 4 },
        Point { x: 5, y: 1 },
        Point { x: 17, y: 12 },
        Point { x: 16, y: 12 },
        Point { x: 15, y: 12 },
        Point { x: 15, y: 11 },
        Point { x: 15, y: 10 },
        Point { x: 10, y: 19 },
        Point { x: 9, y: 19 },
        Point { x: 0, y: 19 },
    ]
    .iter()
    .map(|&p| Snake::new(p))
    .collect();

    benchmark_n_snakes(snakes);
}

pub fn benchmark_engine_4_snakes() {
    let snakes = [
        Point { x: 2, y: 4 },
        Point { x: 5, y: 1 },
        Point { x: 17, y: 12 },
        Point { x: 10, y: 19 },
    ]
    .iter()
    .map(|&p| Snake::new(p))
    .collect();

    benchmark_n_snakes(snakes);
}

pub fn main() {
    // benchmark_engine_10_snakes();
    let game = SnakeGame::new(create_board());
    dbg!(game.available_moves_snake(0));

    // for _ in 0..100 {
    //     if game.board().alive_snakes().count() > 0 {
    //         board.step((0..board.snakes).into_iter().map(|_| rand::random()).collect());
    //     }
    // }
    dbg!(game.board().matrice().array());

    let mut mcts = MCTSManager::new(
        game,
        MyMCTS,
        MyEvaluator,
        UCTPolicy::new(5.0),
        // (),
        ApproxTable::new(1024),
    );

    mcts.playout_n(1_000_000);
    // mcts.playout_n_parallel(1_000_000, 12);
    dbg!(mcts.best_move());
    // let pv: Vec<_> = mcts
    //     .principal_variation_states(10)
    //     .into_iter()
    //     .collect();
    // println!("Principal variation: {:?}", pv);
    println!("Evaluation of moves:");
    mcts.tree().debug_moves();
}
