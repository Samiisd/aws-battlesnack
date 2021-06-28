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

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}
