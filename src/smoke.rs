use std::f32::consts::PI;

use crate::{traits::Drawable, SMOKE_SIZE};
use rand::Rng;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A struct for handling smoke
pub struct Smoke {
    x: f32,
    y: f32,
    pub frame: i32,
}

impl Smoke {
    pub fn new(x: f32, y: f32) -> Smoke {
        Smoke { x, y, frame: 1 }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        self.draw(canvas);
        self.frame += 1;
    }
}

impl Drawable for Smoke {
    /// A method to draw an extension
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let mut points: Vec<Point> = vec![];
        let mut rng = rand::thread_rng();
        for angle in 0..=360 {
            let rads = angle as f32 / 360.0 * 2.0 * PI;
            let x = self.x
                + rng.gen_range(0..10) as f32 / 10.0
                    * SMOKE_SIZE
                    * (self.frame as f32)
                    * (rads as f32).cos();
            let y = self.y
                + rng.gen_range(0..10) as f32 / 10.0
                    * SMOKE_SIZE
                    * (self.frame as f32)
                    * (rads as f32).sin();
            points.push(Point::new(x as i32, y as i32))
        }
        canvas.draw_points(&points[..]).unwrap();
    }
}
