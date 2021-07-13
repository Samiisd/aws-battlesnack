use crate::engine::{Movement, SnakeGame};

pub trait Player {
    fn think(&mut self, game: &SnakeGame);
    fn next_move(&mut self) -> Movement;
    fn get_color(&self) -> [f32; 4];
}
