use crate::engine::Movement;
use piston_window::Button;
use piston_window::Key;

use super::player::Player;

pub struct Human {
    keys: [Key; 4],
    color: [f32; 4],
    last_mov: Movement,
}

impl Human {
    pub(crate) fn new(color: [f32; 4], keys: [Key; 4]) -> Self {
        Self {
            keys,
            color,
            last_mov: Movement::Up,
        }
    }

    pub fn register_key_event(&mut self, press_args: Button) {
        let mov = match press_args {
            Button::Keyboard(k) if k == self.keys[0] => Some(Movement::Left),
            Button::Keyboard(k) if k == self.keys[1] => Some(Movement::Up),
            Button::Keyboard(k) if k == self.keys[2] => Some(Movement::Right),
            Button::Keyboard(k) if k == self.keys[3] => Some(Movement::Down),
            _ => None,
        };

        let mov = mov.unwrap_or(self.last_mov);
        if !mov.is_opposite(self.last_mov) {
            self.last_mov = mov;
        }
    }
}

impl Player for Human {
    fn next_move(&self) -> Movement {
        self.last_mov
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
