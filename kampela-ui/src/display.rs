

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

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
    // draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    // pixelcolor::BinaryColor,
};


use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox as ETTextBox,
};

pub trait DisplayDevice {
    type Target: DrawTarget<Color = BinaryColor>;
    fn get_display_target(&mut self) -> &mut Self::Target;
    fn draw(&mut self) {
        let mut disp = self.get_display_target();

        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        let rect = Rectangle::new(
            Point::new(0, 0),
            Size::new(100, 100),
        );
        let _ = ETTextBox::with_textbox_style("TEEEEEST", rect, character_style, textbox_style)
            .draw(disp);
    }
}
