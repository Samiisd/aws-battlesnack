use crate::engine::{Movement, Player};
use crate::SnakeGame;
use piston_window::Button;
use piston_window::Key;


pub struct Human {
    keys: [Key; 4],
    color: [f32; 4],
    last_mov: Movement,
    registered_mov: Option<Movement>,
}

impl Human {
    pub(crate) fn new(color: [f32; 4], keys: [Key; 4]) -> Self {
        Self {
            keys,
            color,
            last_mov: Movement::Up,
            registered_mov: None,
        }
    }
}

impl Player for Human {
    fn next_move(&mut self) -> Movement {
        self.last_mov = self.registered_mov.unwrap_or(self.last_mov);
        self.registered_mov = None;
        self.last_mov
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn register_key_event(&mut self, press_args: Button) {
        let mov = match press_args {
            Button::Keyboard(k) if k == self.keys[0] => Some(Movement::Left),
            Button::Keyboard(k) if k == self.keys[1] => Some(Movement::Up),
            Button::Keyboard(k) if k == self.keys[2] => Some(Movement::Right),
            Button::Keyboard(k) if k == self.keys[3] => Some(Movement::Down),
            _ => None,
        };

        let mov = mov.unwrap_or(self.last_mov);
        if !mov.is_opposite(self.last_mov) {
            self.registered_mov = Some(mov);
        }
    }

    fn think(&mut self, _: &SnakeGame) {}
}
