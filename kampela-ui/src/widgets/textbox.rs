



use crate::widgets::{widget::Widget, drawable::Drawable, interactive::Interactive};


use crate::stdwrap::String;




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

impl Widget for TextBox {}

impl Drawable for TextBox
{
    fn draw<Display: DrawTarget<Color = BinaryColor>>(&self, display: &mut Display, pos: (u16, u16)) -> Result<(), <Display as DrawTarget>::Error> {
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        let rect = Rectangle::new(
            Point::new(pos.0 as i32, pos.1 as i32),
            Size::new(self.size.0 as u32, self.size.1 as u32),
        );
        let _ = ETTextBox::with_textbox_style(self.text.as_str(), rect, character_style, textbox_style)
            .draw(display)?;
        Ok(())
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