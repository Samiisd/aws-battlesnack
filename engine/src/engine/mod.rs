use serde::{Deserialize, Serialize};

pub mod board;
pub mod collision;
pub mod point;
pub mod reward;
pub mod snake;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}
