use core::ops::Add;
use crate::float_cmp::ApproxEq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

pub fn new(r: f64, g: f64, b: f64) -> Color{
    Color {
        r,
        g,
        b
    }
}

impl Color {
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq(other.r, (0.0, 2)) &&
        self.g.approx_eq(other.g, (0.0, 2)) &&
        self.b.approx_eq(other.b, (0.0, 2)) 
    }
}

impl Eq for Color {}

#[cfg(test)]
mod color_tests {
    use crate::color;
    use crate::float_cmp::ApproxEq;

    #[test]
    fn new_color() {
        let red = color::new(1.0, 0.0, 0.0);
        let c1 = color::new(-0.4, 0.5, 1.7);

        assert!(red.r.approx_eq(1.0, (0.0, 2)));
        assert!(c1.r.approx_eq(-0.4, (0.0, 2)));
        assert!(c1.g.approx_eq(0.5, (0.0, 2)));
        assert!(c1.b.approx_eq(1.7, (0.0, 2)));
    }

    #[test]
    fn add_colors() {
        let red = color::new(1.0, 0.0, 0.0);
        let green = color::new(0.0, 1.0, 0.0);
        let blue = color::new(0.0, 0.0, 1.0);
        let c1 = red + blue;
        let c2 = green + blue;
        let c3 = red + green;

        assert_eq!(c1, color::new(1.0, 0.0, 1.0));
        assert_eq!(c2, color::new(0.0, 1.0, 1.0));
        assert_eq!(c3, color::new(1.0, 1.0, 0.0));

    }
}