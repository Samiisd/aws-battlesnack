use crate::engine::{Movement, SnakeGame};
use piston_window::Button;

pub trait Player {
    fn think(&mut self, game: &SnakeGame);
    fn next_move(&mut self) -> Movement;
    fn get_color(&self) -> [f32; 4];
    fn register_key_event(&mut self, press_args: Button);
}
