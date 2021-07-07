#![feature(hash_drain_filter)]
#![feature(deque_range)]

mod engine;
pub use engine::{Board, Movement, Point, Snake, SnakeGame};
