

// use crate::widgets::display::Display;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;


pub trait Drawable {

    fn draw<Display: DrawTarget<Color = BinaryColor>>(&self, display: &mut Display, pos: (u16, u16)) -> Result<(), Display::Error>;
    fn get_size(&self) -> (u16, u16);
}