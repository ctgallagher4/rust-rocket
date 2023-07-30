extern crate sdl2;

use rusteroids::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::time::Instant;

pub fn main() {
    // Initialize
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rusteroids", 1200, 1000)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game = Game::new();

    // start instant
    let mut instant = Instant::now();

    'running: loop {
        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // check events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // check key presses
        let keyboard_state = KeyboardState::new(&event_pump);

        // Update Step
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let delta = instant.elapsed();
        instant = Instant::now();
        game.update(&mut canvas, &keyboard_state, delta);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
