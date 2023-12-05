



use embedded_graphics::{pixelcolor::BinaryColor, prelude::{DrawTarget}};




pub trait Display {

}


impl Display for DrawTarget<Color = BinaryColor> {

}