use crate::pixel;

#[derive(Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<pixel::Pixel>>
}

pub fn new(height: usize, width: usize) -> Canvas {
    let pixels = vec![vec![pixel::new(0.0, 0.0, 0.0); width]; height];
    Canvas {
        width,
        height,
        pixels
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::canvas;
    use crate::color;

    #[test]
    fn init_correctly() {
        let canvas = canvas::new(20, 20);

        assert_eq!(canvas.width, 20);
        assert_eq!(canvas.height, 20);
        for i in 0..canvas.height {
            for j in 0..canvas.width {
                assert_eq!(canvas.pixels[i][j].color, (color::new(0.0, 0.0, 0.0)));
            }
        }
    }
}