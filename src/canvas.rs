use crate::pixel;
use crate::color;
use std::ops::IndexMut;
use std::ops::Index;

#[derive(Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<pixel::Pixel>>
}

pub fn new(height: usize, width: usize) -> Canvas {
    let pixels = vec![vec![pixel::new(color::new(0.0, 0.0, 0.0)); width]; height];
    Canvas {
        width,
        height,
        pixels
    }
}

impl Canvas {
    pub fn to_ppm(self) -> String {
        let mut result = String::new();
        result.push_str("P3\n");
        result = result + &format!("{} {}\n",self.width.to_string(), self.height.to_string());
        result = result + "255\n";

        // TODO: a more functional approach to clean up this looping
        let mut pixel_grid = Vec::<String>::new();
        for i in 0..self.height {
            for j in 0..self.width {
                pixel_grid.push(self[(i, j)].color.to_string());
            }
        }
        result + &pixel_grid.join("\n")
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = pixel::Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
       &self.pixels[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut pixel::Pixel{
        &mut self.pixels[index.0][index.1]
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::canvas;
    use crate::color;
    use crate::pixel;

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

    #[test]
    fn writing_pixels() {
        let mut canvas = canvas::new(10, 10);
        let red = color::new(1.0, 0.0, 0.0);
        let c1 = color::new(0.25, 0.5, 0.75);
        canvas[(0, 5)] = pixel::new(red);
        canvas[(0, 0)] = pixel::new(red);
        canvas[(3 ,4)] = pixel::new(c1);
        canvas[(9, 9)] = pixel::new(c1);

        assert_eq!(canvas[(0, 5)].color, red);
        assert_eq!(canvas[(3, 4)].color, c1);
        assert_eq!(canvas[(9, 9)].color, c1);
        assert_eq!(canvas[(0, 0)].color, red);
    }

    #[test]
    fn canvas_to_ppm() {
        let mut canvas = canvas::new(3, 3);
        let c1 = color::new(0.1, 0.7, 0.7);
        let c2 = color::new(0.0, 0.0, 0.7);
        canvas[(0, 1)] = pixel::new(c1);
        canvas[(2, 2)] = pixel::new(c2);

        let ppm = canvas.to_ppm();
        let mut ppm_lines = ppm.lines();

        assert_eq!(ppm_lines.next(), Some("P3"));
        assert_eq!(ppm_lines.next(), Some("3 3"));
        assert_eq!(ppm_lines.next(), Some("255"));
        //TODO: write an elegant test to ensure that all of the pixels are printed correctly in the ppm string
    }
}