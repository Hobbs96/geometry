use crate::color;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub color: color::Color
}

pub fn new(color: color::Color) -> Pixel {
    Pixel {
        color
    }
}