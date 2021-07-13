#![feature(proc_macro_hygiene, decl_macro)]
#![feature(available_concurrency)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
#[cfg(test)]
mod test;

// External crates
#[macro_use]
extern crate rocket;

use std::time::{Instant};

use engine::Player;

// Uses
use rocket::{http::Status, serde::json::Json, tokio};

fn convert_snake(s: &requests::Snake) -> engine::Snake {
    engine::Snake::new_from(s.health, s.body.clone(), s.length as usize, s.head)
}

fn convert_snakes(snakes: &[requests::Snake]) -> Vec<engine::Snake> {
    snakes.into_iter().map(convert_snake).collect()
}

fn convert_board(board: &requests::Board) -> engine::Board {
    engine::Board::new_from(
        board.width,
        board.height,
        convert_snakes(&board.snakes),
        &board.food,
    )
}

#[get("/")]
fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: Some("sissaad".to_string()),
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("0.1".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

#[post("/move", data = "<req>")]
async fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let since_execution = Instant::now();

    let snake_id = req
        .board
        .snakes
        .iter()
        .enumerate()
        .find(|(_, s)| s.id == req.you.id)
        .map(|(id, _)| id)
        .unwrap();

    let latency_max = req.game.timeout as u64;
    // fixme: replace by avg latency
    let latency = req.you.latency.parse::<u64>().unwrap_or(50);

    let board = convert_board(&req.board);
    let mut game = engine::SnakeGame::new(board);
    game.set_player(snake_id);

    let mut bot = engine::BotA::new(
        snake_id,
        std::thread::available_concurrency()
            .map(|p| p.get())
            .unwrap_or(32),
        [0.0; 4],
    );

    bot.think(&game);

    let sleep_time = latency_max - (latency + since_execution.elapsed().as_millis() as u64 + 10) ;

    // fixme: that should be async + await, but who cares, 2h left lol
    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_time)).await;

    let movement = responses::Move::new(bot.next_move());

    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, start, movement, end])
}
