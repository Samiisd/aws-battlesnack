pub enum Collision {
    None,
    Wall,
    SelfBody,
    OtherBody,
    HeadToHead {
        src_length: usize,
        dst_length: usize,
    },
}

impl Collision {
    pub fn causes_death(&self) -> bool {
        match *self {
            Collision::None => false,
            Collision::Wall => true,
            Collision::SelfBody => true,
            Collision::OtherBody => true,
            Collision::HeadToHead {
                src_length,
                dst_length,
            } => src_length <= dst_length,
        }
    }
}
