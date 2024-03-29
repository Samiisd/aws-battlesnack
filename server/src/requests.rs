use engine::Point;
use serde::Deserialize;
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Turn {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    pub id: String,
    pub timeout: i32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Point>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Point>,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Point>,
    pub head: Point,
    pub length: u32,
    pub shout: String,
    pub squad: String,
    pub latency: i32,
}
