#![feature(hash_drain_filter)]
#![feature(deque_range)]
#![feature(option_insert)]
mod engine;
mod ui;


extern crate piston_window;

use crate::engine::{Board, SnakeGame, Point, Snake};
use crate::ui::Player;
use mcts::GameState;
use piston_window::*;

const COLOR_WALL: [f32; 4] = [0.8, 0.8, 0.7, 1.];

const OFFSET: (f64, f64) = (100., 100.);
const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 12;
const TILE_SIZE: f64 = 30.0;
const FREQ_SECONDS: f64 = 0.4;


fn x<T>(v: T) -> f64 
     where T : Into<f64> {
    v.into() * TILE_SIZE + OFFSET.0
}

fn y<T>(v: T) -> f64 
     where T : Into<f64> {
    v.into() * TILE_SIZE + OFFSET.1
}


fn render_walls(board: &Board, t: math::Matrix2d, gfx: &mut G2d) {
    let (w, h) = (board.width(), board.height());

    let wall_size = TILE_SIZE;

    // upper row 
    line(COLOR_WALL, wall_size, [x(-1), y(-1), x(w+1), y(-1)], t, gfx);

    // left column
    line(COLOR_WALL, wall_size, [x(-1), y(-1), x(-1), y(h+1)], t, gfx);

    // right column
    line(COLOR_WALL, wall_size, [x(w+1), y(-1), x(w+1), y(h+1)], t, gfx);

    // bottom row
    line(COLOR_WALL, wall_size, [x(-1), y(h+1), x(board.width()+1), y(h+1)], t, gfx);
}

fn render_players(board: &Board, players: &Vec<Box<dyn Player>>, t: math::Matrix2d, gfx: &mut G2d) {
    board.alive_snakes().for_each(|(id, s)| {
        let head = s.head();
        let color = players[id].get_color();

        // draw body
        s.body_without_head().for_each(|p| {
            rectangle(
                color,
                rectangle::square(x(p.x), y(p.y), TILE_SIZE),
                t,
                gfx,
            )
        });

        // draw head
        rectangle(
            [1., 1., 1., 1.],
            rectangle::square(x(head.x), y(head.y), TILE_SIZE/2.),
            t,
            gfx,
        );
    });
}

fn main() {
    let mut players : Vec<Box<dyn Player>> = vec![
        Box::new(ui::BotA::new(3, color::hex("eeff11"))),
        // Box::new(ui::BotA::new(3, color::hex("00ff11"))),
        // Box::new(ui::Human::new(color::hex("50E4EA"), [Key::Left, Key::Down, Key::Right, Key::Up])),
        Box::new(ui::Human::new(color::hex("57D658"), [Key::A, Key::S, Key::D, Key::W])),
    ];

    let snakes = vec![
        Snake::new(Point { x: 8, y: 8}),
        Snake::new(Point { x: 4, y: 2}),
        // Snake::new(Point { x: 1, y: 5}),
    ];

    let board = Board::new(BOARD_WIDTH as i32, BOARD_HEIGHT as i32, snakes);

    let mut game = SnakeGame::new(board);

    players[0].think(&game);

    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston!",
        [
            (BOARD_WIDTH as f64 * TILE_SIZE) as u32,
            (BOARD_HEIGHT as f64 * TILE_SIZE) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut time = 0.0;

    while let Some(event) = window.next() {
        // Update input for users
        if let Some(press_args) = event.press_args() {
            players
                .iter_mut()
                .for_each(|p| p.as_mut().register_key_event(press_args))
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            render_walls(game.board(), context.transform, graphics);
            render_players(game.board(), &players, context.transform, graphics);
        });

        event.update(|arg| {
            time += arg.dt;

            if time >= FREQ_SECONDS {
                game.make_move(&players.iter_mut().map(|p| p.next_move()).collect());

                players
                    .iter_mut()
                    .for_each(|p| p.think(&game));
                
                time = 0.;
            }
        });

        if game.board().nb_snakes_alive() == 0 {
            break;
        }
    }
}
