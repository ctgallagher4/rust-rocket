use std::f32::consts::PI;

use crate::{traits::Drawable, ACC, HEIGHT, SHIP_SIZE, SPEED_LIMIT, TURN_SPEED, WIDTH};
use sdl2::keyboard::Scancode;
use sdl2::{keyboard::KeyboardState, rect::Point, render::Canvas, video::Window};

#[derive(PartialEq)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub bear: f32,
}

impl Player {
    /// A constructor function which takes x, y, and baring
    pub fn new(x: f32, y: f32, speed_x: f32, speed_y: f32, bear: f32) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y,
            bear,
        }
    }
    /// A function to move a player forward
    pub fn forward(&mut self) {
        self.speed_x += ACC * self.bear.cos();
        self.speed_y += ACC * self.bear.sin();
        if self.speed_x > SPEED_LIMIT {
            self.speed_x = SPEED_LIMIT;
        } else if self.speed_x < -1.0 * SPEED_LIMIT {
            self.speed_x = -1.0 * SPEED_LIMIT
        }
        if self.speed_y > SPEED_LIMIT {
            self.speed_y = SPEED_LIMIT
        } else if self.speed_y < -1.0 * SPEED_LIMIT {
            self.speed_y = -1.0 * SPEED_LIMIT
        }
    }
    /// A function to turn left
    pub fn turn_left(&mut self) {
        self.bear -= TURN_SPEED * 2.0 * PI / 360.0;
    }
    /// A function to turn right
    pub fn turn_right(&mut self) {
        self.bear += TURN_SPEED * 2.0 * PI / 360.0;
    }
    /// Update the player and missiles
    pub fn update(&mut self, canvas: &mut Canvas<Window>, keyboard_state: &KeyboardState) -> bool {
        let key_w: bool = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::W);
        let key_a: bool = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::A);
        let key_d: bool = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::D);
        let mut thrust: bool = false;
        if key_w {
            self.forward();
            thrust = true;
        }
        if key_a {
            self.turn_left();
        }
        if key_d {
            self.turn_right();
        }
        self.x += self.speed_x;
        self.y += self.speed_y;
        if self.x > WIDTH as f32 {
            self.x = 0.0;
        } else if self.x < 0 as f32 {
            self.x = WIDTH as f32;
        }
        if self.y > HEIGHT as f32 {
            self.y = 0.0;
        } else if self.y < 0 as f32 {
            self.y = HEIGHT as f32;
        }
        self.draw(canvas);
        thrust
    }
}

impl Drawable for Player {
    /// A function to draw the player + fire
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let (topx, topy) = (
            self.x + self.bear.cos() * SHIP_SIZE,
            self.y + self.bear.sin() * SHIP_SIZE,
        );
        let top: Point = Point::new(topx.round() as i32, topy.round() as i32);
        let bot_left: Point = calculate_bottom_left(self.x, self.y, self.bear);
        let bot_right: Point = calculate_bottom_right(self.x, self.y, self.bear);
        let points_array: [Point; 4] = [top, bot_left, bot_right, top];
        canvas.draw_lines(&points_array[..]).unwrap();
    }
}

/// Calculates the bottom left of a ship
fn calculate_bottom_left(x: f32, y: f32, bear: f32) -> Point {
    let (left_x, left_y) = (
        (x + (200.0 * 2.0 * PI / 360.0 + bear).cos() * SHIP_SIZE) as i32,
        (y + (200.0 * 2.0 * PI / 360.0 + bear).sin() * SHIP_SIZE) as i32,
    );
    Point::new(left_x, left_y)
}

/// Calculates the bottom right of a ship
fn calculate_bottom_right(x: f32, y: f32, bear: f32) -> Point {
    let (right_x, right_y) = (
        (x + (160.0 * 2.0 * PI / 360.0 + bear).cos() * SHIP_SIZE) as i32,
        (y + (160.0 * 2.0 * PI / 360.0 + bear).sin() * SHIP_SIZE) as i32,
    );
    Point::new(right_x, right_y)
}
