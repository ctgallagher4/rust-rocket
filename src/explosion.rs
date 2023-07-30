use std::f32::consts::PI;

use crate::traits::Drawable;
use crate::ASTEROID_NUM_FRAMES;
use rand::Rng;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A struct to hold an explosion
pub struct Explosion {
    x: f32,
    y: f32,
    pub frame: i32,
    size: f32,
}

impl Explosion {
    /// A method to generate a new explosion
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            x,
            y,
            frame: 1,
            size,
        }
    }
    /// A method to update an explosion
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        self.frame += 1;
        self.draw(canvas)
    }
}

impl Drawable for Explosion {
    /// A method to draw an explosion
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let mut rng = rand::thread_rng();
        let mut points: Vec<Point> = Default::default();
        for angle in 0..=360 {
            let rads = angle as f32 / 360.0 * 2.0 * PI;
            let x = self.x
                + self.size * self.frame as f32 / (ASTEROID_NUM_FRAMES as f32 * 0.8)
                    * rng.gen_range(0..10) as f32
                    / 10.0
                    * (rads as f32).cos();
            let y = self.y
                + self.size * self.frame as f32 / (ASTEROID_NUM_FRAMES as f32 * 0.8)
                    * rng.gen_range(0..10) as f32
                    / 10.0
                    * (rads as f32).sin();
            points.push(Point::new(x as i32, y as i32))
        }
        canvas.draw_points(&points[..]).unwrap();
    }
}
