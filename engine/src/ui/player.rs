use crate::engine::Movement;

pub trait Player {
    fn next_move(&self) -> Movement;
    fn get_color(&self) -> [f32; 4];
}
