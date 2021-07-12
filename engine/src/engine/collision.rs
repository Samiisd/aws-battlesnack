use crate::Point;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum Collision {
    Wall {
        id: usize,
    },
    SelfBody {
        id: usize,
    },
    OtherBody {
        id_1: usize,
        id_2: usize,
        loc: Point,
    },
    HeadToHead {
        src_length: usize,
        dst_length: usize,
        id_1: usize,
        id_2: usize,
        loc: Point,
    },
}

impl Collision {
    pub fn causes_death(&self) -> bool {
        match *self {
            Collision::Wall { .. } => true,
            Collision::SelfBody { .. } => true,
            Collision::OtherBody { .. } => true,
            Collision::HeadToHead {
                src_length,
                dst_length,
                ..
            } => src_length <= dst_length,
        }
    }
}
