#![feature(hash_drain_filter)]
#![feature(deque_range)]
mod engine;
mod ui;

// extern crate ndarray;

// use crate::engine::{Board, Movement, MyEvaluator, MyMCTS, Point, Snake, SnakeGame};
// use mcts::transposition_table::ApproxTable;
// use mcts::tree_policy::UCTPolicy;
// use mcts::MCTSManager;

// fn create_board() -> Board {
//     let snakes = [
//         Point { x: 0, y: 1 },
//         Point { x: 2, y: 3 },
//         Point { x: 3, y: 1 },
//         Point { x: 3, y: 2 },
//         Point { x: 3, y: 3 },
//         // Point { x: 6, y: 2 },
//         // Point { x: 7, y: 2 },
//         // Point { x: 8, y: 1 },
//         // Point { x: 8, y: 3 },
//         // Point { x: 8, y: 4 },
//     ]
//     .iter()
//     .map(|&p| Snake::new(p))
//     .collect();

//     Board::new(4, 4, snakes)
// }

// fn benchmark_n_snakes(snakes: Vec<Snake>) {
//     let n_snakes = snakes.len();

//     let mut board = Board::new(21, 21, snakes.clone());
//     for _ in 0..100 {
//         if board.alive_snakes().count() > 0 {
//             board.step((0..n_snakes).into_iter().map(|_| rand::random()).collect());
//         }
//     }
// }

// pub fn benchmark_engine_10_snakes() {
//     let snakes = [
//         Point { x: 2, y: 4 },
//         Point { x: 5, y: 1 },
//         Point { x: 17, y: 12 },
//         Point { x: 16, y: 12 },
//         Point { x: 15, y: 12 },
//         Point { x: 15, y: 11 },
//         Point { x: 15, y: 10 },
//         Point { x: 10, y: 19 },
//         Point { x: 9, y: 19 },
//         Point { x: 0, y: 19 },
//     ]
//     .iter()
//     .map(|&p| Snake::new(p))
//     .collect();

//     benchmark_n_snakes(snakes);
// }

// pub fn benchmark_engine_4_snakes() {
//     let snakes = [
//         Point { x: 2, y: 4 },
//         Point { x: 5, y: 1 },
//         Point { x: 17, y: 12 },
//         Point { x: 10, y: 19 },
//     ]
//     .iter()
//     .map(|&p| Snake::new(p))
//     .collect();

//     benchmark_n_snakes(snakes);
// }

// pub fn main() {
//     // benchmark_engine_10_snakes();
//     let game = SnakeGame::new(create_board());
//     dbg!(game.available_moves_snake(0));

//     // for _ in 0..100 {
//     //     if game.board().alive_snakes().count() > 0 {
//     //         board.step((0..board.snakes).into_iter().map(|_| rand::random()).collect());
//     //     }
//     // }
//     dbg!(game.board().matrice().array());

//     let mut mcts = MCTSManager::new(
//         game,
//         MyMCTS,
//         MyEvaluator,
//         UCTPolicy::new(5.0),
//         // (),
//         ApproxTable::new(1024),
//     );

//     mcts.playout_n(1_000_000);
//     // mcts.playout_n_parallel(1_000_000, 12);
//     dbg!(mcts.best_move());
//     // let pv: Vec<_> = mcts
//     //     .principal_variation_states(10)
//     //     .into_iter()
//     //     .collect();
//     // println!("Principal variation: {:?}", pv);
//     println!("Evaluation of moves:");
//     mcts.tree().debug_moves();
// }

extern crate piston_window;

use crate::engine::{Board, SnakeGame, Point, Snake};
use crate::ui::Player;
use mcts::GameState;
use piston_window::*;

const COLOR_WALL: [f32; 4] = [0.8, 0.8, 0.7, 1.];

const OFFSET: (f64, f64) = (100., 100.);
const BOARD_WIDTH: usize = 21;
const BOARD_HEIGHT: usize = 21;
const TILE_SIZE: f64 = 30.0;
const FREQ_SECONDS: f64 = 0.5;


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

    // upper row 
    line(COLOR_WALL, TILE_SIZE, [x(-1), y(-1), x(w+1), y(-1)], t, gfx);

    // left column
    line(COLOR_WALL, TILE_SIZE, [x(-1), y(-1), x(-1), y(h+1)], t, gfx);

    // right column
    line(COLOR_WALL, TILE_SIZE, [x(w+1), y(-1), x(w+1), y(h+1)], t, gfx);

    // bottom row
    line(COLOR_WALL, TILE_SIZE, [x(-1), y(h+1), x(board.width()+1), y(h+1)], t, gfx);
}

fn render_players(board: &Board, players: &Vec<impl Player>, t: math::Matrix2d, gfx: &mut G2d) {
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
    let mut players = vec![
        ui::Human::new(color::hex("50E4EA"), [Key::Left, Key::Down, Key::Right, Key::Up]),
        ui::Human::new(color::hex("57D658"), [Key::A, Key::S, Key::D, Key::W]),
    ];

    let snakes = vec![
        Snake::new(Point { x: 2, y: 4}),
        Snake::new(Point { x: 5, y: 2}),
    ];

    let board = Board::new(BOARD_WIDTH as i32, BOARD_HEIGHT as i32, snakes);

    let mut game = SnakeGame::new(board);

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
                .for_each(|p| p.register_key_event(press_args))
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            render_walls(game.board(), context.transform, graphics);
            render_players(game.board(), &players, context.transform, graphics);
        });

        event.update(|arg| {
            time += arg.dt;
            if time >= FREQ_SECONDS {
                game.make_move(&players.iter().map(|p| p.next_move()).collect());
                time = 0.;

            }
        });

        if game.board().nb_snakes_alive() == 0 {
            break;
        }
    }
}
