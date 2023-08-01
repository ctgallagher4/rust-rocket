pub mod asteroid;
pub mod explosion;
pub mod game;
pub mod missile;
pub mod player;
pub mod smoke;
pub mod traits;

pub const ACC: f32 = 180.0;
pub const SHIP_SIZE: f32 = 20.0;
pub const TURN_SPEED: f32 = 150.0;
pub const WIDTH: i32 = 1200;
pub const HEIGHT: i32 = 1000;
pub const SPEED_LIMIT: f32 = 200.0;
pub const FLAME_SIZE: f32 = 5.0;
pub const AST_POINTS: usize = 36;
pub const SMOKE_SIZE: f32 = 1.0;
pub const SMOKE_FRAMES: i32 = 35;
pub const NUM_ASTEROIDS: usize = 8;
pub const PERT_SIZE: i32 = 5;
pub const ASTEROID_NUM_FRAMES: i32 = 50;
pub const MISSLE_REFRESH: u64 = 400;
