use core::ops::Index;
use core::ops::IndexMut;

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    elements: Vec<Vec<f64>>    
}

pub fn new(rows: usize, columns: usize) -> Matrix {
    if columns < 1 || rows < 1 {
        panic!("Rows and columns of a matrix both must be >= 1");
    }
    Matrix {
        rows,
        columns,
        elements: vec![vec![0.0; columns]; rows]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64{
        &mut self.elements[index.0][index.1]
    }
}
#[cfg(test)]
mod matrix_tests {
    use crate::matrix;
    use crate::float_cmp::ApproxEq;

    #[test]
    fn new_matrix() {
        let matrix = matrix::new(4, 4);

        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                assert!(matrix[(i, j)].approx_eq(0.0, (0.0, 2)));
            }
        }
    }

    #[test]
    fn matrix_assignment_by_index() {
        let mut matrix = matrix::new(4, 4);
        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                matrix[(i, j)] = (i + j) as f64;
            }
        }

        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                assert!(matrix[(i, j)].approx_eq((i + j) as f64, (0.0, 2)));
            }
        }
    }
}