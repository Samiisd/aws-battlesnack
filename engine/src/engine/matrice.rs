use crate::engine::board::SnakeId;
use crate::{Point, Snake};
use ndarray::Array2;

pub(crate) type CellValue = u8;
pub type Displacement = (SnakeId, (Option<Point>, Point));

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Matrice {
    height: usize,
    width: usize,
    array: Array2<CellValue>,
}

impl Matrice {
    pub fn new(snakes: &Vec<Snake>, height: usize, width: usize) -> Self {
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
    pub fn update(&mut self, displacements: Vec<Displacement>) {
        displacements.iter().for_each(|&(idx, (tail, to))| {
            if let Some(tail) = tail {
                debug_assert_eq!(
                    idx,
                    (self.get(tail).unwrap_or(255)) as SnakeId,
                    "Probably moving wrong snake"
                );
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
        let v = self.array[[p.y as usize, p.x as usize]];
        if v == 0 {
            None
        } else {
            Some(v - 1)
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
