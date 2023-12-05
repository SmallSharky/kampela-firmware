




pub trait Drawable {
    pub fn draw(&self, display: &mut Display);
    pub fn get_size(&self) -> (u16, u16);
}