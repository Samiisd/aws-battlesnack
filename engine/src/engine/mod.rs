use serde::{Deserialize, Serialize};

mod board;
mod collision;
mod game;
mod matrice;
mod mcts;
mod point;
mod reward;
mod snake;

pub use self::mcts::{MyEvaluator, MyMCTS};
pub use board::Board;
pub use collision::Collision;
pub use game::SnakeGame;
pub use point::Point;
pub use snake::Snake;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl Movement {
    pub fn is_opposite(&self, o: Movement) -> bool {
        match self {
            Movement::Right => o == Movement::Left,
            Movement::Left => o == Movement::Right,
            Movement::Up => o == Movement::Down,
            Movement::Down => o == Movement::Up,
        }
    }
}

impl Distribution<Movement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Movement {
        match rng.gen_range(0..4) {
            0 => Movement::Right,
            1 => Movement::Left,
            2 => Movement::Up,
            _ => Movement::Down,
        }
    }
}
