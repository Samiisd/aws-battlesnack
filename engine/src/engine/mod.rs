use serde::{Deserialize, Serialize};

mod board;
mod collision;
mod point;
mod reward;
mod snake;

pub use board::Board;
pub use collision::Collision;
pub use point::Point;
pub use snake::Snake;

use rand::{Rng, distributions::{Distribution, Standard}};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
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
