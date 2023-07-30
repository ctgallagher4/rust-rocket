use std::f32::consts::PI;

use crate::traits::Drawable;
use crate::{AST_POINTS, HEIGHT, SPEED_LIMIT, WIDTH};
use rand::{self, Rng};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Asteroid {
    pub x: f32,
    pub y: f32,
    pub rad: f32,
    vel_x: f32,
    vel_y: f32,
    perturbations: Vec<(i32, i32)>,
}

impl Asteroid {
    pub fn new(perturbations: Vec<(i32, i32)>) -> Self {
        let mut rng = rand::thread_rng();
        let choices = [
            "top".to_string(),
            "bottom".to_string(),
            "left".to_string(),
            "right".to_string(),
        ];
        #[allow(unused_assignments)]
        let (mut x, mut y) = (0.0, 0.0);
        let choice = &choices[rng.gen_range(0..4)];
        if *choice == "top".to_string() {
            (x, y) = (rng.gen_range(0..WIDTH) as f32, 0.0);
        } else if *choice == "bottom".to_string() {
            (x, y) = (rng.gen_range(0..WIDTH) as f32, HEIGHT as f32);
        } else if *choice == "left".to_string() {
            (x, y) = (0.0, rng.gen_range(0..WIDTH) as f32);
        } else {
            (x, y) = (WIDTH as f32, rng.gen_range(0..WIDTH) as f32);
        }
        let rad = rng.gen_range(20..51) as f32;
        let vel_x = rng.gen_range(-1.0 * SPEED_LIMIT / 2.0..SPEED_LIMIT / 2.0) as f32;
        let vel_y = rng.gen_range(-1.0 * SPEED_LIMIT / 2.0..SPEED_LIMIT / 2.0) as f32;
        Self {
            x,
            y,
            rad,
            vel_x,
            vel_y,
            perturbations,
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        if self.x > WIDTH as f32 {
            self.x = 0.0;
        } else if self.x < 0.0 {
            self.x = WIDTH as f32;
        }
        if self.y > HEIGHT as f32 {
            self.y = 0.0;
        } else if self.y < 0.0 {
            self.y = HEIGHT as f32;
        }
        self.draw(canvas)
    }
}

impl Drawable for Asteroid {
    /// A function to draw an asteroid
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let mut point_tuples: Vec<(i32, i32)> = Vec::new();
        for i in 0..36 {
            let i1 = i * 10;
            let angle: f32 = i1 as f32 * (2.0 * PI) / 360.0;
            let mut x = (self.x + self.rad * angle.cos()) as i32;
            let mut y = (self.y + self.rad * angle.sin()) as i32;
            x += self.perturbations[i].0 as i32;
            y += self.perturbations[i].1 as i32;
            point_tuples.push((x, y))
        }
        let mut points: [Point; AST_POINTS + 1] = [Point::new(0, 0); AST_POINTS + 1];

        for index in 0..points.len() - 1 {
            points[index] = Point::new(point_tuples[index].0, point_tuples[index].1)
        }
        points[AST_POINTS] = points[0];
        canvas.draw_lines(&points[..]).unwrap();
    }
}
