use std::collections::HashSet;

use serde::Deserialize;

use super::{Movement, collision::Collision, point::Point, snake::Snake};

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    height: i32,
    width: i32,
    food: HashSet<Point>,
    snakes: Vec<Snake>,
}

// Game
impl Board {
    pub fn new(width: i32, height: i32, snakes: Vec<Snake>) -> Self {
        Self {
            height,
            width,
            snakes,
            food: HashSet::new(),
        }
    }

    pub fn step(&mut self, movs: Vec<Movement>) {
        debug_assert_eq!(self.snakes.len(), movs.len());

        // Move all alive snakes
        self.alive_snakes_mut()
            .for_each(|(i, s)| s.apply_move(movs[i]));

        // Compute all collisions
        let collisions : Vec<Collision> = (0..self.snakes.len())
            .filter(|&i| !self.snakes[i].is_dead())
            .map(|i| self.check_collision(i))
            .collect();

        // kill snakes that got killing collision
        collisions
            .iter()
            .enumerate()
            .filter(|(_, c)| c.causes_death())
            .for_each(|(i, _)| self.snakes[i].kill());

        // Feed snakes
        let snake_heads : HashSet<Point> = self.alive_snakes()
            .map(|(_, s)| *s.head())
            .collect();

        let food_available : HashSet<Point> = self.food
            .drain_filter(|f| snake_heads.contains(f))
            .collect();

        self.alive_snakes_mut()
            .filter(|(_, s)| food_available.contains(s.head()))
            .for_each(|(_, s)| s.feed());

        // kill hungry snakes
        self.alive_snakes_mut()
            .filter(|(_, s)| s.health() == 0)
            .for_each(|(_, s)| s.kill());
    }
}

// Helpers
impl Board {
    pub fn alive_snakes(&self) -> impl Iterator<Item = (usize, &Snake)> {
        self.snakes.iter().enumerate().filter(|(_, s)| !s.is_dead())
    }

    pub fn alive_snakes_mut(&mut self) -> impl Iterator<Item = (usize, &mut Snake)> {
        self.snakes
            .iter_mut()
            .enumerate()
            .filter(|(_, s)| !s.is_dead())
    }
}

// collisions
impl Board {
    pub fn check_collision(&self, snake_id: usize) -> Collision {
        debug_assert!(!self.snakes[snake_id].is_dead());

        // check wall
        if self.collides_wall(snake_id) {
            Collision::Wall
        } else if self.collides_other_body(snake_id) {
            Collision::OtherBody
        } else if self.collides_self_body(snake_id) {
            Collision::SelfBody
        } else if let Some(collision) = self.collides_head_to_head(snake_id) {
            collision
        } else {
            Collision::None
        }
    }

    #[inline]
    fn collides_wall(&self, snake_id: usize) -> bool {
        let p = self.snakes[snake_id].head();
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn collides_self_body(&self, snake_id: usize) -> bool {
        // FIXME: this can be optimized using Collision Map
        let snake = &self.snakes[snake_id];
        snake.body().contains(snake.head())
    }

    fn collides_other_body(&self, snake_id: usize) -> bool {
        // FIXME: this can be optimized using Collision Map
        let snake = &self.snakes[snake_id];
        self.snakes
            .iter()
            .any(|other| other.body().contains(snake.head()))
    }

    fn collides_head_to_head(&self, snake_id: usize) -> Option<Collision> {
        // FIXME: this can be optimized using Collision Map
        let snake = &self.snakes[snake_id];
        self.alive_snakes()
            .filter(|&(i, other)| i != snake_id && other.head() == snake.head())
            .map(|(_, other)| Collision::HeadToHead {
                src_length: snake.length(),
                dst_length: other.length(),
            })
            .next()
    }
}
