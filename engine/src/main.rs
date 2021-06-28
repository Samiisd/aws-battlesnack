#![feature(hash_drain_filter)]
mod engine;

extern crate ndarray;

use crate::engine::{Board, Point, Snake, Movement};
use ruscii::{
    app::{App, State},
    drawing::{Pencil},
    gui::FPSCounter,
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, Window},
};

#[inline]
fn point_to_vec2(p: Point) -> Vec2 {
    Vec2::xy(p.x, p.y)
}

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();
    let win_size = app.window().size();

    let snake = &Snake::new(Point { y: 2, x: 3 });
    let board = &mut Board::new(10, 12, vec![snake.clone()]);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        if board.alive_snakes().count() == 0 {
            app_state.stop();
        }

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("{:?}", app_state.keyboard().last_key_events()), Vec2::xy(20, 20));
        if let Some(mov) = match app_state.keyboard().get_keys_down().first() {
            Some(Key::Up) | Some(Key::W) => Some(Movement::Down),
            Some(Key::Down) | Some(Key::S) => Some(Movement::Up),
            Some(Key::Left) | Some(Key::A) => Some(Movement::Left),
            Some(Key::Right) | Some(Key::D) => Some(Movement::Right),
            _ => None,
        } {
            pencil.draw_text(&format!("{:?}", mov), Vec2::xy(20, 15));
            board.step(vec![mov]);
        }

        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0));


        board.alive_snakes().for_each(|(_, s)| {
            pencil.set_foreground(Color::Cyan);
            pencil.draw_text(&format!("{:?}", s.head()), Vec2::xy(20, 10));
            pencil.draw_char('@', point_to_vec2(*s.head()));
            s.body().iter().for_each(|&p| {
                pencil.draw_char('#', point_to_vec2(p));
            })
        });

        pencil.set_origin(win_size / 2.);
        fps_counter.update();
    });
}
