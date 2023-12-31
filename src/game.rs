use crate::{
    asteroid::Asteroid, explosion::Explosion, missile::Missile, player::Player, smoke::Smoke,
    HEIGHT, NUM_ASTEROIDS, SHIP_SIZE, SMOKE_FRAMES, SPEED_LIMIT, WIDTH,
};
use crate::{ASTEROID_NUM_FRAMES, MISSLE_REFRESH, PERT_SIZE};
use rusty_time::Timer;
use sdl2::keyboard::Scancode;
use sdl2::video::Window;
use sdl2::{keyboard::KeyboardState, render::Canvas};
use std::f32::consts::PI;
use std::time::Duration;

/// A struct to hold the game structs
pub struct Game {
    player: Player,
    asteroids: Vec<Asteroid>,
    missiles: Vec<Missile>,
    missile_timer: Timer,
    smoke_trail: Vec<Smoke>,
    explosions: Vec<Explosion>,
    pub score: i32,
}

impl Game {
    /// A struct for storing the game's structs
    pub fn new() -> Self {
        let player: Player = Player::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32, 0.0, 0.0, 0.0);
        let pert: Vec<Vec<(i32, i32)>> = Asteroid::init_perturbations(8, 360, 5);
        let asteroids: Vec<Asteroid> = vec![
            Asteroid::new(pert[0].clone()),
            Asteroid::new(pert[1].clone()),
            Asteroid::new(pert[2].clone()),
            Asteroid::new(pert[3].clone()),
            Asteroid::new(pert[4].clone()),
            Asteroid::new(pert[5].clone()),
            Asteroid::new(pert[6].clone()),
            Asteroid::new(pert[7].clone()),
        ];
        let missiles: Vec<Missile> = vec![];
        let missile_timer = Timer::from_millis(MISSLE_REFRESH);
        Self {
            player,
            asteroids,
            missiles,
            missile_timer,
            smoke_trail: vec![],
            explosions: vec![],
            score: 0,
        }
    }
    /// A method to update the game's structs
    pub fn update(
        &mut self,
        canvas: &mut Canvas<Window>,
        keyboard_state: &KeyboardState,
        delta: Duration,
    ) -> bool {
        self.missile_timer.update(delta);
        let thrust = self.player.update(canvas, keyboard_state, delta);
        if thrust {
            let back_x = self.player.x + SHIP_SIZE * (self.player.bear + PI).cos();
            let back_y = self.player.y + SHIP_SIZE * (self.player.bear + PI).sin();
            self.smoke_trail.push(Smoke::new(back_x, back_y))
        }
        for smoke in self.smoke_trail.iter_mut() {
            smoke.update(canvas)
        }
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update(canvas, delta);
        }
        for missile in self.missiles.iter_mut() {
            missile.update(canvas, delta);
        }
        for explosion in self.explosions.iter_mut() {
            explosion.update(canvas);
        }
        self.check_remove_missiles();
        self.check_add_missiles(keyboard_state);
        self.check_remove_smoke();
        if self.check_collision() {
            return true;
        }
        self.check_refill_asteroids();
        self.check_remove_explosion();
        return false;
    }
    /// A method to check if missiles should be removed
    fn check_remove_missiles(&mut self) {
        let mut rem_indices: Vec<usize> = vec![];
        for (index, missile) in self.missiles.iter().enumerate() {
            if missile.x > WIDTH as f32
                || missile.x < 0.0
                || missile.y > HEIGHT as f32
                || missile.y < 0.0
            {
                rem_indices.push(index)
            }
        }
        let mut count: usize = 0;
        for i in rem_indices {
            self.missiles.remove(i - count);
            count += 1;
        }
    }
    /// A method to check if smoke should be removed
    fn check_remove_smoke(&mut self) {
        let mut rem_indices: Vec<usize> = vec![];
        for (index, smoke) in self.smoke_trail.iter().enumerate() {
            if smoke.frame > SMOKE_FRAMES {
                rem_indices.push(index)
            }
        }
        let mut count: usize = 0;
        for i in rem_indices {
            self.smoke_trail.remove(i - count);
            count += 1;
        }
    }
    /// A method to check if missiles should be added
    fn check_add_missiles(&mut self, keyboard_state: &KeyboardState) {
        let key_space: bool = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::Space);
        if key_space && self.missile_timer.ready {
            let (top_x, top_y) = (
                self.player.x + self.player.bear.cos() * SHIP_SIZE,
                self.player.y + self.player.bear.sin() * SHIP_SIZE,
            );
            let x_vel: f32 = self.player.speed_x + 2.0 * SPEED_LIMIT * self.player.bear.cos();
            let y_vel: f32 = self.player.speed_y + 2.0 * SPEED_LIMIT * self.player.bear.sin();
            self.missiles
                .push(Missile::new(top_x, top_y, x_vel, y_vel, self.player.bear));
            self.missile_timer.reset()
        }
    }
    /// A method to check collisions
    fn check_collision(&mut self) -> bool {
        let mut rem_indices_asteroid: Vec<usize> = vec![];
        let mut rem_indices_missile: Vec<usize> = vec![];
        for (index, asteroid) in self.asteroids.iter_mut().enumerate() {
            if ((self.player.x - asteroid.x).powf(2.0) + (self.player.y - asteroid.y).powf(2.0))
                .powf(0.5)
                < asteroid.rad + SHIP_SIZE
            {
                return true;
            }
            for (index2, missile) in self.missiles.iter().enumerate() {
                if ((missile.x - asteroid.x).powf(2.0) + (missile.y - asteroid.y).powf(2.0))
                    .powf(0.5)
                    < asteroid.rad + SHIP_SIZE / 2.0
                {
                    rem_indices_asteroid.push(index);
                    rem_indices_missile.push(index2);
                    self.explosions
                        .push(Explosion::new(asteroid.x, asteroid.y, asteroid.rad));
                    self.score += 1;
                }
            }
        }
        let mut count = 0;
        for i in rem_indices_asteroid.iter() {
            self.asteroids.remove(i - count);
            count += 1;
        }
        count = 0;
        for i in rem_indices_missile.iter() {
            self.missiles.remove(i - count);
            count += 1;
        }
        return false;
    }
    /// A funtion to refill the number of asteroids in the system
    fn check_refill_asteroids(&mut self) {
        if self.asteroids.len() == 0 {
            let pert = Asteroid::init_perturbations(NUM_ASTEROIDS as i32, 36, PERT_SIZE);
            for i in 0..NUM_ASTEROIDS {
                self.asteroids.push(Asteroid::new(pert[i].clone()));
            }
        }
    }
    /// A function to remove explosions
    fn check_remove_explosion(&mut self) {
        let mut remove_indices: Vec<usize> = vec![];
        for (index, explosion) in self.explosions.iter().enumerate() {
            if explosion.frame == ASTEROID_NUM_FRAMES {
                remove_indices.push(index);
            }
        }
        let mut count = 0;
        for i in remove_indices {
            self.explosions.remove(i - count);
            count += 1;
        }
    }
}
