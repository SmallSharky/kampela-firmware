
#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
}


#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::vec::Vec;
}


use stdwrap::*;

use crate::{drawable::Drawable, interactive::Interactive, display::DisplayDevice, widget::Widget};

// use crate::display::DisplayDevice;


use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_4X6, FONT_6X10},
        MonoTextStyle,
    },
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, Rectangle,
    },
    Drawable as EGDrawable,
};

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
};


use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox as ETTextBox,
};

pub struct TextBox {
    text: String,
    size: (u16, u16),
}

impl <T: DrawTarget<Color=BinaryColor>> Widget <T> for TextBox {}

impl <T: DrawTarget<Color=BinaryColor>> Drawable <T> for TextBox
{
    fn draw(&self, display: &mut dyn DisplayDevice<Target = T>, pos: (u16, u16)) {
        // let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        // let textbox_style = TextBoxStyleBuilder::new()
        //     .alignment(HorizontalAlignment::Center)
        //     .vertical_alignment(VerticalAlignment::Middle)
        //     .build();

        let rect = Rectangle::new(
            Point::new(pos.0 as i32, pos.1 as i32),
            Size::new(self.size.0 as u32, self.size.1 as u32),
        );
        // display.draw(ETTextBox::with_textbox_style(self.text.as_str(), rect, character_style, textbox_style));
        display.drawText(self.text.as_str(), rect, &FONT_10X20);
    }

    fn get_size(&self) -> (u16, u16) {
        self.size
    }

}

impl Interactive for TextBox {
    fn handle_tap(&mut self, rel_pos: (u16, u16)) {}
}
impl TextBox {
    pub fn new(text: String, bounds: (u16, u16)) -> Self {
        Self {
            text: text,
            size: bounds,
        }
    }
}