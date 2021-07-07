use serde::Deserialize;

use crate::engine::Movement;

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn apply_mov(&self, mov: Movement) -> Point {
        match mov {
            Movement::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Movement::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Movement::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Movement::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}
