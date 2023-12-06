

// use crate::widgets::display::Display;
// use embedded_graphics::draw_target::DrawTarget;
// use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::display::DisplayDevice;


pub trait Drawable <T>{

    fn draw(&self, display: &mut dyn DisplayDevice<Target=T>, pos: (u16, u16));
    fn get_size(&self) -> (u16, u16);
}