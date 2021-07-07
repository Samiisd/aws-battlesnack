use crate::engine::matrice::Matrice;
use std::collections::HashSet;
use std::hash::Hash;

use crate::engine::Movement;

use super::{matrice::Displacement, Collision};
use crate::{Point, Snake};
use itertools::Itertools;

pub type SnakeId = u8;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    height: i32,
    width: i32,
    food: HashSet<Point>,
    snakes: Vec<Snake>,
    matrice: Matrice,
}

impl Hash for Board {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.matrice.hash(state);
    }
}

// Game
impl Board {
    pub fn new(width: i32, height: i32, snakes: Vec<Snake>) -> Self {
        Self {
            matrice: Matrice::new(&snakes, height as usize, width as usize),
            height,
            width,
            snakes,
            food: HashSet::new(),
        }
    }

    pub fn step(&mut self, movs: Vec<Movement>) {
        // dbg!(self.alive_snakes().map(|(_,s)| s.health()).collect_vec(), &movs);
        // dbg!(self.nb_snakes_alive(), &movs);

        debug_assert_eq!(self.snakes.len(), movs.len());
        // dbg!(self.alive_snakes().count());
        // dbg!(&movs);
        // dbg!(self.alive_snakes().map(|(_, s)| *s.head()).collect_vec());

        // Move all alive snakes
        let displacements = self.update_snakes_positions(movs);

        // Kill collided snakes
        self.kill_collided_snakes();

        // Feed snakes
        self.feed_snakes();

        // kill hungry snakes
        self.kill_hungry_snakes();

        // update matrice
        self.update_matrice(displacements.clone()); // FIXME: remove clone

        // dbg!(&displacements, self.matrice.array());
    }
}

// Helpers
impl Board {
    pub fn matrice(&self) -> &Matrice {
        &self.matrice
    }

    pub fn snakes(&self) -> &Vec<Snake> {
        &self.snakes
    }

    pub fn alive_snakes(&self) -> impl Iterator<Item = (usize, &Snake)> {
        self.snakes.iter().enumerate().filter(|(_, s)| !s.is_dead())
    }

    pub fn alive_snakes_mut(&mut self) -> impl Iterator<Item = (usize, &mut Snake)> {
        self.snakes
            .iter_mut()
            .enumerate()
            .filter(|(_, s)| !s.is_dead())
    }

    pub fn update_matrice(&mut self, displacements: Vec<Displacement>) {
        self.matrice.update(
            displacements
                .into_iter()
                .filter(|(id, ..)| !self.snakes[*id as usize].is_dead())
                .collect(),
        );
    }

    #[inline]
    pub fn nb_snakes(&self) -> usize {
        self.snakes.len()
    }

    #[inline]
    pub fn nb_snakes_alive(&self) -> usize {
        self.alive_snakes().count()
    }

    #[inline]
    pub fn is_outside(&self, p: Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }
}

// engine logic
impl Board {
    fn update_snakes_positions(&mut self, movs: Vec<Movement>) -> Vec<Displacement> {
        self.alive_snakes_mut()
            .map(|(i, s)| (i as SnakeId, s.apply_move(movs[i])))
            .collect()
    }

    fn kill_hungry_snakes(&mut self) {
        let hungry_snakes: Vec<usize> = self
            .alive_snakes()
            .filter(|(_, s)| s.health() == 0)
            .map(|(i, _)| i)
            .collect();

        self.kill_snakes(hungry_snakes.into_iter());
    }

    fn feed_snakes(&mut self) {
        let snake_heads: HashSet<Point> = self.alive_snakes().map(|(_, s)| *s.head()).collect();

        let food_available: HashSet<Point> = self
            .food
            .drain_filter(|f| snake_heads.contains(f))
            .collect();

        self.alive_snakes_mut()
            .filter(|(_, s)| food_available.contains(s.head()))
            .for_each(|(_, s)| s.feed());
    }

    fn kill_collided_snakes(&mut self) {
        // Compute all collisions
        let collisions: Vec<(usize, Collision)> = self.alive_snakes()
            .map(|(i,_)| (i, self.check_collision(i)))
            .filter(|(_, c)| c.causes_death())
            .collect();

        // if collisions.len() > 0 {
        //     dbg!(&collisions);
        // }

        // kill snakes that got killing collision
        self.kill_snakes( collisions.iter().map(|(id,_)| *id));
    }
}

// collisions
impl Board {
    pub fn kill_snakes(&mut self, iter_dead_snakes: impl Iterator<Item = usize>) {
        iter_dead_snakes.for_each(|i| {
            let s = &mut self.snakes[i];
            self.matrice.remove_points(s.body().iter());
            s.kill();
        });
    }

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
        self.is_outside(*p)
    }

    fn collides_self_body(&self, snake_id: usize) -> bool {
        let snake = &self.snakes[snake_id];
        snake.body_without_head().any(|p| p == snake.head())
    }

    fn collides_other_body(&self, snake_id: usize) -> bool {
        let snake = &self.snakes[snake_id];
        self.alive_snakes()
            .any(|(_, other)| other.body_without_head().any(|p| p == snake.head()))
    }

    fn collides_head_to_head(&self, snake_id: usize) -> Option<Collision> {
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
