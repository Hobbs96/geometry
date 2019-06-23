use crate::color;
use crate::pixel;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<pixel::Pixel>>,
}

pub fn new(height: usize, width: usize) -> Canvas {
    let pixels = vec![vec![pixel::new(color::new(0.0, 0.0, 0.0)); width]; height];
    Canvas {
        width,
        height,
        pixels,
    }
}

impl Canvas {
    pub fn to_ppm(self) -> String {
        //TODO: some ppm programs don't allow lines over 70 chars
        let mut ppm = String::new();
        ppm.push_str("P6\n");
        ppm = ppm + &format!("{} {}\n", self.width.to_string(), self.height.to_string());
        ppm = ppm + "255\n";

        let mut pixel_grid = Vec::<String>::new();
        for i in 0..self.height {
            pixel_grid.push(
                self.pixels[i]
                    .iter()
                    .enumerate()
                    .map(|(index, pixel)| {
                        if index < self.width - 1 {
                            pixel.color.to_string() + " "
                        } else {
                            pixel.color.to_string()
                        }
                    })
                    .collect(),
            );
        }
        ppm + &pixel_grid.join("\n") + "\n" //some ppm programs are picky about a trailing newline
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = pixel::Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixels[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut pixel::Pixel {
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
        canvas[(3, 4)] = pixel::new(c1);
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
        let c2 = color::new(-0.5, 0.5, 1.1);
        let c3 = color::new(0.0, 0.0, 0.7);
        canvas[(0, 1)] = pixel::new(c1);
        canvas[(1, 1)] = pixel::new(c2);
        canvas[(2, 2)] = pixel::new(c3);

        let ppm = canvas.to_ppm();
        let correct_ppm = "P6\n3 3\n255\n\
                           0 0 0 25 178 178 0 0 0\n\
                           0 0 0 0 127 255 0 0 0\n\
                           0 0 0 0 0 0 0 0 178\n"
            .to_string();
        let mut ppm_lines = ppm.lines();

        assert_eq!(ppm_lines.next(), Some("P6"));
        assert_eq!(ppm_lines.next(), Some("3 3"));
        assert_eq!(ppm_lines.next(), Some("255"));
        assert_eq!(ppm, correct_ppm);
    }
}
