



use crate::widgets::widget::Widget;

use crate::stdwrap::Std;

pub struct TextBox {
    text: Std::String,
    size: (u16, u16),
}


impl Widget for TextBox {
    fn draw(&self, display: &mut Display, pos: (u16, u16)) {
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        let restore = Rectangle::new(
            Point::new(0, 50),
            Size::new(SCREEN_SIZE_X / 2, SCREEN_SIZE_Y - 50),
        );
        let generate = Rectangle::new(
            Point::new(SCREEN_SIZE_X as i32 / 2, 50),
            Size::new(SCREEN_SIZE_X / 2, SCREEN_SIZE_Y - 50),
        );
        let header = Rectangle::new(Point::new(0, 0), Size::new(SCREEN_SIZE_X, 50));
        TextBox::with_textbox_style("restore", restore, character_style, textbox_style)
            .draw(display)?;
        TextBox::with_textbox_style("generate", generate, character_style, textbox_style)
            .draw(display)?;
        TextBox::with_textbox_style(
            "restore or generate?",
            header,
            character_style,
            textbox_style,
        )
        .draw(display)?;
    }

}

impl for TextBox {
    pub fn new(text: String, bounds: (i16, i16)) -> Self {

    }
}