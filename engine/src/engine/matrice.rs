use crate::engine::{board::SnakeId, Point, Snake};
use ndarray::Array2;

use super::Collision;

pub(crate) type CellValue = u8;
pub type Displacement = (SnakeId, (Option<Point>, Point));

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Matrice {
    height: usize,
    width: usize,
    array: Array2<CellValue>,
}

impl Matrice {
    pub fn new(snakes: &[Snake], height: usize, width: usize) -> Self {
        let mut matrice = Matrice {
            height,
            width,
            array: Array2::zeros((height, width)),
        };

        snakes.iter().enumerate().for_each(|(id, snake)| {
            let id = id as u8;
            snake.body().iter().for_each(|&p| {
                matrice.mark_snake(id, p);
            });
        });

        matrice
    }
}

// updaters
impl Matrice {
    pub fn update(&mut self, displacements: Vec<Displacement>, collisions: &[Collision]) {
        // fix collisions overrides
        collisions
            .iter()
            .filter_map(|c| match c {
                Collision::OtherBody { id_2, loc, .. } => Some((id_2, loc)),
                Collision::HeadToHead {
                    src_length,
                    dst_length,
                    id_1,
                    id_2,
                    loc,
                } => match src_length {
                    _ if src_length < dst_length => Some((id_2, loc)),
                    _ if src_length > dst_length => Some((id_1, loc)),
                    _ => None,
                },
                _ => None,
            })
            .for_each(|(&id, &p)| self.mark_snake(id as u8, p));

        // apply displacements
        displacements.iter().for_each(|&(idx, (tail, to))| {
            if let Some(tail) = tail {
                self.mark_empty(tail);
            }

            self.mark_snake(idx, to);
        })
    }

    pub fn remove_points<'a>(&mut self, points: impl Iterator<Item = &'a Point>) {
        let (h, w) = (self.height, self.height);
        points
            .filter(|&p| p.x >= 0 && p.x < w as i32 && p.y >= 0 && p.y < h as i32)
            .for_each(|p| self.mark_empty(*p));
    }
}

// getters
impl Matrice {
    pub fn array(&self) -> &Array2<CellValue> {
        &self.array
    }

    #[inline]
    pub fn get(&self, p: Point) -> Option<CellValue> {
        match self.array[[p.y as usize, p.x as usize]] {
            0 => None,
            v => Some(v - 1),
        }
    }

    #[inline]
    fn set(&mut self, p: Point, v: CellValue) {
        self.array[[p.y as usize, p.x as usize]] = v;
    }
}

// rules
impl Matrice {
    #[inline]
    pub fn mark_snake(&mut self, v: SnakeId, p: Point) {
        self.set(p, (v + 1) as CellValue);
    }

    #[inline]
    fn mark_empty(&mut self, p: Point) {
        self.set(p, 0);
    }
}
