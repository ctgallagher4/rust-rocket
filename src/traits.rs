use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Drawable {
    fn draw(&mut self, canvas: &mut Canvas<Window>);
}
