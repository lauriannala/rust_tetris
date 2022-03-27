extern crate sdl2;
mod config;
mod field;
mod tetromino;
use config::{HEIGHT, WIDTH, WINDOW_MULTIPLIER};
use field::Field;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use tetromino::Tetromino;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust_tetris",
            WIDTH * WINDOW_MULTIPLIER,
            HEIGHT * WINDOW_MULTIPLIER,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut tetromino = Tetromino::new()?;

    let field = Field::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // TODO
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // TODO
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    // TODO
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    // TODO
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (x, y) in &field.0 {
            if tetromino.is_set(*x, *y) {
                let render = Rect::new(
                    (x * WINDOW_MULTIPLIER) as i32,
                    (y * WINDOW_MULTIPLIER) as i32,
                    WINDOW_MULTIPLIER,
                    WINDOW_MULTIPLIER,
                );
                canvas.fill_rect(render).unwrap();
            }
        }
        canvas.present();
        tetromino.move_next();
    }

    Ok(())
}
