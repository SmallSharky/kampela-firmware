

use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor};

use crate::widgets::{interactive::Interactive, drawable::Drawable};


pub trait Widget: Drawable + Interactive {
    fn draw<Display: DrawTarget<Color = BinaryColor>>(&self, display: &mut Display, pos: (u16, u16)) -> Result<(), Display::Error> {
        Drawable::draw(self, display, pos)
    }
    fn get_size(&self) -> (u16, u16) {
        Drawable::get_size(self)
    }
}