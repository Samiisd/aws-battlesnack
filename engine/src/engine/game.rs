use crate::engine::{Board, Movement};
use mcts::transposition_table::TranspositionHash;
use mcts::GameState;
use rand::{prelude::SliceRandom, thread_rng};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct SnakeGame {
    current_player: usize,
    board: Board,
}

impl SnakeGame {
    pub fn new(board: Board) -> Self {
        SnakeGame {
            current_player: 0,
            board,
        }
    }

    pub fn set_player(&mut self, id: usize) {
        debug_assert!(id < self.board.snakes().len());
        self.current_player = id
    }

    pub fn available_moves_snake(&self, id: usize) -> Vec<Movement> {
        let s = &self.board.snakes()[id];
        let head = s.head();
        let matrice = self.board.matrice();
        let available_moves = if s.is_dead() {
            vec![]
        } else {
            [
                Movement::Down,
                Movement::Left,
                Movement::Up,
                Movement::Right,
            ]
            .iter()
            .filter_map(|&m| {
                let new_position = head.apply_mov(m);
                if self.board.is_outside(new_position) || matrice.get(new_position).is_some() {
                    None
                } else {
                    Some(m)
                }
            })
            .collect()
        };

        if available_moves.is_empty() {
            vec![]
        } else {
            available_moves
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn step(&mut self, mov: Vec<Movement>) {
        self.board.step(mov, false);
    }
}

impl TranspositionHash for SnakeGame {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.board.hash(&mut hasher);
        hasher.finish()
    }
}

impl GameState for SnakeGame {
    type Player = usize;
    type Move = Movement;
    type MoveList = Vec<Movement>;

    fn current_player(&self) -> Self::Player {
        self.current_player
    }

    fn available_moves(&self) -> Self::MoveList {
        self.available_moves_snake(self.current_player)
    }

    fn make_move(&mut self, mov: &Self::Move) {
        let sample = (0..self.board().snakes().len())
            .map(|id| {
                if id == self.current_player() {
                    *mov
                } else {
                    *self
                        .available_moves_snake(id)
                        .choose(&mut thread_rng())
                        .unwrap_or(&Movement::Up)
                }
            })
            .collect();

        self.board.step(sample, true);
    }
}
