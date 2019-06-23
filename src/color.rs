use crate::float_cmp::ApproxEq;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Sub;
use std::string::ToString;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq(other.r, (0.0, 2))
            && self.g.approx_eq(other.g, (0.0, 2))
            && self.b.approx_eq(other.b, (0.0, 2))
    }
}

impl Eq for Color {}

impl ToString for Color {
    fn to_string(&self) -> String {
        let scaled_colors = [self.r, self.g, self.b]
            .iter()
            .map(|color| (color * 255.0) as i32)
            .map(|color| {
                if color < 0 {
                    0
                } else if color > 255 {
                    255
                } else {
                    color
                }
            })
            .collect::<Vec<i32>>();
        format!(
            "{} {} {}",
            scaled_colors[0], scaled_colors[1], scaled_colors[2]
        )
    }
}

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
        let c4 = color::new(3.4, 7.1, 0.2) + color::new(1.2, 0.1, 0.05);

        assert_eq!(c1, color::new(1.0, 0.0, 1.0));
        assert_eq!(c2, color::new(0.0, 1.0, 1.0));
        assert_eq!(c3, color::new(1.0, 1.0, 0.0));
        assert_eq!(c4, color::new(4.6, 7.2, 0.25));
    }

    #[test]
    fn subtract_colors() {
        let red = color::new(1.0, 0.0, 0.0);
        let green = color::new(0.0, 1.0, 0.0);
        let blue = color::new(0.0, 0.0, 1.0);
        let c1 = red - blue;
        let c2 = green - blue;
        let c3 = red - green;
        let c4 = color::new(4.5, 0.21, 0.71) - color::new(0.8, 0.02, 0.17);

        assert_eq!(c1, color::new(1.0, 0.0, -1.0));
        assert_eq!(c2, color::new(0.0, 1.0, -1.0));
        assert_eq!(c3, color::new(1.0, -1.0, 0.0));
        assert_eq!(c4, color::new(3.7, 0.19, 0.54));
    }

    #[test]
    fn scalar_multiplication() {
        let red = color::new(1.0, 0.0, 0.0);
        let green = color::new(0.0, 1.0, 0.0);
        let blue = color::new(0.0, 0.0, 1.0);
        let c1 = red * 0.7;
        let c2 = green * 0.2;
        let c3 = blue * 1.5;
        let c4 = color::new(0.5, 0.8, 0.16) * 2.0;

        assert_eq!(c1, color::new(0.7, 0.0, 0.0));
        assert_eq!(c2, color::new(0.0, 0.2, 0.0));
        assert_eq!(c3, color::new(0.0, 0.0, 1.5));
        assert_eq!(c4, color::new(1.0, 1.6, 0.32));
    }

    #[test]
    fn color_color_multiplication() {
        let red = color::new(1.0, 0.0, 0.0);
        let green = color::new(0.0, 1.0, 0.0);
        let blue = color::new(0.0, 0.0, 1.0);
        let c1 = red * green;
        let c2 = green * blue;
        let c3 = color::new(2.0, 1.0, 0.5) * color::new(0.72, 0.69, 0.17);
        let c4 = color::new(0.46, 0.71, 0.84) * color::new(1.0, 1.0, 1.0);

        assert_eq!(c1, color::new(0.0, 0.0, 0.0));
        assert_eq!(c2, color::new(0.0, 0.0, 0.0));
        assert_eq!(c3, color::new(1.44, 0.69, 0.085));
        assert_eq!(c4, color::new(0.46, 0.71, 0.84));
    }
}
