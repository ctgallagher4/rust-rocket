use crate::traits::Drawable;
use crate::SHIP_SIZE;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A struct to store a missile
pub struct Missile {
    pub x: f32,
    pub y: f32,
    x_vel: f32,
    y_vel: f32,
    pub bear: f32,
}

impl Missile {
    /// A method to generate a new missile
    pub fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, bear: f32) -> Self {
        Self {
            x,
            y,
            x_vel,
            y_vel,
            bear,
        }
    }
    /// A method to update a missile
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        self.x += self.x_vel;
        self.y += self.y_vel;

        self.draw(canvas)
    }
}

impl Drawable for Missile {
    /// A method to draw a missile
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let x_top: i32 = (self.x + SHIP_SIZE / 2.0 * self.bear.cos()) as i32;
        let y_top: i32 = (self.y + SHIP_SIZE / 2.0 * self.bear.sin()) as i32;
        let top: Point = Point::new(x_top, y_top);
        let bottom: Point = Point::new(self.x as i32, self.y as i32);
        canvas.draw_line(top, bottom).unwrap();
    }
}
