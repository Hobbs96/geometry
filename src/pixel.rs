use crate::color;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub color: color::Color
}

pub fn new(r: f64, g: f64, b: f64) -> Pixel {
    Pixel {
        color: color::new(r, g, b)
    }
}