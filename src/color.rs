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
}