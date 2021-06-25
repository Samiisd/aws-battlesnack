#![feature(hash_drain_filter)]
mod engine;

extern crate ndarray;

use engine::{board::Board, point::Point, snake::Snake};

use crate::engine::Movement;

fn main() {
    let snake = &Snake::new(Point { y: 2, x: 3 });
    dbg!(snake);

    let board = &mut Board::new(10, 12, vec![snake.clone()]);
    dbg!(&board);

    while board.alive_snakes().count() != 0 {
        board.step(vec![Movement::Right]);
        dbg!(&board);
    }
}
