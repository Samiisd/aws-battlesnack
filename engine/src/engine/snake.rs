use std::collections::VecDeque;

use serde::Deserialize;

use super::{Movement, point::Point};

const DEFAULT_SNAKE_HEALTH: i32 = 100;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Snake {
    health: i32,
    body: VecDeque<Point>,
    head: Point,
    length: usize,
}

impl Snake {
    pub fn new(head: Point) -> Self {
        Snake {
            health: DEFAULT_SNAKE_HEALTH,
            body: VecDeque::new(),
            length: 3,
            head,
        }
    }

    pub fn apply_move(&mut self, mov: Movement) {
        debug_assert!(!self.is_dead());

        // Last body part (their tail) is removed from the board
        debug_assert!((self.body.len() + 1) <= self.length);
        self.body.push_back(self.head);
        if (self.body.len() + 1) > self.length {
            self.body.pop_front();
        }

        // A new body part is added to the board in the direction they moved.
        self.head = self.head.apply_mov(mov);

        // moves cost one life point
        self.consume_health();
    }

    pub fn consume_health(&mut self) {
        debug_assert!(!self.is_dead());

        self.health -= 1;
    }

	pub fn kill(&mut self) {
        debug_assert!(!self.is_dead());

        self.body.clear();
		self.length = 0;
	}

    #[inline]
    pub fn feed(&mut self) {
        debug_assert!(!self.is_dead());

        // Health reset set maximum.
        self.health = DEFAULT_SNAKE_HEALTH;
        // Additional body part placed on top of current tail (this will extend their visible length by one on the next turn).
        self.length += 1;
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.length == 0
    }
}

impl Snake {
    #[inline]
    pub fn health(&self) -> i32 {
        self.health
    }

    #[inline]
    pub fn head(&self) -> &Point {
        &self.head
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn body(&self) -> &VecDeque<Point> {
        &self.body
    }
}
