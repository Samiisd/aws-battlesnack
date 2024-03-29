use itertools::Itertools;
use ndarray::Array2;
use rand::prelude::SliceRandom;

use crate::engine::matrice::Matrice;
use std::{collections::HashSet, iter::FromIterator};
use std::hash::Hash;
use std::vec;

use crate::engine::Movement;

use super::{matrice::Displacement, Collision, Point, Snake};

pub type SnakeId = u8;

#[derive(Debug, Clone)]
pub struct Board {
    height: i32,
    width: i32,
    food: HashSet<Point>,
    snakes: Vec<Snake>,
    matrice: Matrice,
    collisions: Vec<Collision>,
    food_spawn_chance: f32,
    food_min_amount: usize,
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
            food_min_amount: snakes.len() - 1,
            matrice: Matrice::new(&snakes, height as usize, width as usize),
            height,
            width,
            snakes,
            collisions: vec![],
            food: HashSet::new(),
            food_spawn_chance: 0.15,
        }
    }

    pub fn new_from(width: i32, height: i32, snakes: Vec<Snake>, food: &[Point]) -> Self {
        Self {
            food_min_amount: snakes.len() - 1,
            matrice: Matrice::new(&snakes, height as usize, width as usize),
            height,
            width,
            snakes,
            collisions: vec![],
            food: HashSet::from_iter(food.iter().map(|p| *p)),
            food_spawn_chance: 0.15,
        }
    }

    pub fn step(&mut self, movs: Vec<Movement>, is_simulation: bool) {
        debug_assert_eq!(self.snakes.len(), movs.len());

        // Move all alive snakes
        let displacements = self.update_snakes_positions(movs);

        // Kill collided snakes
        self.kill_collided_snakes();

        // Feed snakes
        self.feed_snakes();

        // kill hungry snakes
        self.kill_hungry_snakes();

        // spawn food
        if !is_simulation {
            self.spawn_food();
        }

        // update matrice
        self.update_matrice(displacements); // FIXME: remove clone
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
            &self.collisions,
        );
    }

    #[inline]
    pub fn nb_snakes_alive(&self) -> usize {
        self.alive_snakes().count()
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn collisions(&self) -> &Vec<Collision> {
        &self.collisions
    }

    #[inline]
    pub fn food(&self) -> &HashSet<Point> {
        &self.food
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

        self.kill_snakes(hungry_snakes);
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
        self.collisions = self
            .alive_snakes()
            .filter_map(|(i, _)| self.check_collision(i))
            .collect();

        let snakes_to_kill: Vec<usize> = self
            .collisions
            .iter()
            .filter(|&c| c.causes_death())
            .map(|c| match *c {
                Collision::Wall { id } => id,
                Collision::SelfBody { id } => id,
                Collision::OtherBody { id_1, .. } => id_1,
                Collision::HeadToHead { id_1, .. } => id_1,
            })
            .collect();

        // kill snakes that got killing collision
        self.kill_snakes(snakes_to_kill);
    }

    fn spawn_food(&mut self) {
        let nb_curr_food = self.food.len();
        if self.food.len() < self.food_min_amount {
            self.spawn_food_rnd(self.food_min_amount - nb_curr_food);
        } else if rand::random::<f32>() < self.food_spawn_chance {
            self.spawn_food_rnd(1);
        }
    }

    fn spawn_food_rnd(&mut self, n: usize) {
        let a: Vec<Point> = self
            .unoccupied_points()
            .choose_multiple(&mut rand::thread_rng(), n)
            .copied()
            .collect();

        a.iter().for_each(|p| {
            self.food.insert(*p);
        });
    }

    fn unoccupied_points(&self) -> Vec<Point> {
        let mut h_empty: Array2<bool> =
            Array2::from_elem([self.height as usize, self.width as usize], true);
        let mut mark_not_empty = |p: &Point| h_empty[[p.y as usize, p.x as usize]] = false;

        self.alive_snakes()
            .for_each(|(_, s)| s.body().iter().for_each(&mut mark_not_empty));
        self.food.iter().for_each(&mut mark_not_empty);

        (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|&(y, x)| h_empty[[y as usize, x as usize]])
            .map(|(y, x)| Point { y, x })
            .collect()
    }
}

// collisions
impl Board {
    pub fn kill_snakes(&mut self, snakes_dead: Vec<usize>) {
        snakes_dead.into_iter().for_each(|i| {
            let s = &mut self.snakes[i];
            self.matrice.remove_points(s.body().iter());
            s.kill();
        });
    }

    pub fn check_collision(&self, snake_id: usize) -> Option<Collision> {
        debug_assert!(!self.snakes[snake_id].is_dead());

        self.collides_wall(snake_id)
            .or_else(|| self.collides_self_body(snake_id))
            .or_else(|| self.collides_other_body(snake_id))
            .or_else(|| self.collides_head_to_head(snake_id))
    }

    #[inline]
    fn collides_wall(&self, snake_id: usize) -> Option<Collision> {
        let p = self.snakes[snake_id].head();
        if self.is_outside(*p) {
            Some(Collision::Wall { id: snake_id })
        } else {
            None
        }
    }

    fn collides_self_body(&self, snake_id: usize) -> Option<Collision> {
        let snake = &self.snakes[snake_id];
        if snake.body_without_head().any(|&p| p == *snake.head()) {
            Some(Collision::SelfBody { id: snake_id })
        } else {
            None
        }
    }

    fn collides_other_body(&self, snake_id: usize) -> Option<Collision> {
        let snake = &self.snakes[snake_id];
        self.alive_snakes()
            .filter(|(_, other)| other.body_without_head().any(|p| p == snake.head()))
            .map(|(id_other, _)| Collision::OtherBody {
                id_1: snake_id,
                id_2: id_other,
                loc: *snake.head(),
            })
            .next()
    }

    fn collides_head_to_head(&self, snake_id: usize) -> Option<Collision> {
        let snake = &self.snakes[snake_id];
        self.alive_snakes()
            .filter(|&(i, other)| i != snake_id && other.head() == snake.head())
            .map(|(id_other, other)| Collision::HeadToHead {
                src_length: snake.length(),
                dst_length: other.length(),
                id_1: snake_id,
                id_2: id_other,
                loc: *snake.head(),
            })
            .next()
    }
}
