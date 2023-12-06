
// use crate::display::DisplayDevice;
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use crate::{interactive::Interactive, drawable::Drawable, display::DisplayDevice};


pub trait Widget<T: DrawTarget<Color=BinaryColor>>: Drawable<T> + Interactive {
    fn draw(&self, display: &mut dyn DisplayDevice<Target=T>, pos: (u16, u16)) {
        Drawable::draw(self, display, pos)
    }
    fn get_size(&self) -> (u16, u16) {
        Drawable::get_size(self)
    }
}