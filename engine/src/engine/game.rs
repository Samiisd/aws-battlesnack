use crate::engine::{Board, Movement};
use itertools::*;
use mcts::transposition_table::TranspositionHash;
use mcts::GameState;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct SnakeGame {
    current_player: u8,
    movement_queue: Vec<Movement>,
    board: Board,
}

impl SnakeGame {
    pub fn new(board: Board) -> Self {
        SnakeGame {
            current_player: 0,
            movement_queue: Vec::with_capacity(board.alive_snakes().count()),
            board,
        }
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
            .filter_map(|&m| -> Option<Movement> {
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
            vec![Movement::Up]
        } else {
            available_moves
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
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
    type Move = Vec<Movement>;
    type MoveList = Vec<Vec<Movement>>;

    fn current_player(&self) -> Self::Player {
        0
    }

    fn available_moves(&self) -> Self::MoveList {
        (0..self.board().snakes().len())
            .map(|id| self.available_moves_snake(id))
            .multi_cartesian_product()
            .collect()
    }

    fn make_move(&mut self, mov: &Self::Move) {
        self.board.step(mov.clone());
    }
}
