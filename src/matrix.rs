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

//change the implementation so that by indexing you get a vector, enabling [][] syntax
impl Index<(usize)> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vec<f64> {
        &mut self.elements[index]
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
                assert!(matrix[i][j].approx_eq(0.0, (0.0, 2)));
            }
        }
    }

    #[test]
    fn matrix_assignment_by_index() {
        let mut matrix = matrix::new(4, 4);
        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                matrix[i][j] = (i + j) as f64;
            }
        }

        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                assert!(matrix[i][j].approx_eq((i + j) as f64, (0.0, 2)));
            }
        }
    }

    #[test]
    fn matrices_of_differing_size() {
        let mut matrix_small = matrix::new(2, 2);
        let mut matrix_medium = matrix::new(3, 3);

        matrix_small[0][0] = -3.0;
        matrix_small[0][1] = 5.0;
        matrix_small[1][0] =  1.0;
        matrix_small[1][1] = -2.0;

        matrix_medium[0][0] = -3.0;
        matrix_medium[0][1] = 5.0;
        matrix_medium[0][2] = 0.0;
        matrix_medium[1][0] = 1.0;
        matrix_medium[1][1] = -2.0;
        matrix_medium[1][2] = -7.0;
        matrix_medium[2][0] = 0.0;
        matrix_medium[2][1] = 1.0;
        matrix_medium[2][2] = 1.0;
        
        assert!(matrix_small[0][0].approx_eq(-3.0, (0.0, 2)));
        assert!(matrix_small[0][1].approx_eq(5.0, (0.0, 2)));
        assert!(matrix_small[1][0].approx_eq(1.0, (0.0, 2)));
        assert!(matrix_small[1][1].approx_eq(-2.0, (0.0, 2)));

        assert!(matrix_medium[0][0].approx_eq(-3.0, (0.0, 2)));
        assert!(matrix_medium[1][1].approx_eq(-2.0, (0.0, 2)));
        assert!(matrix_medium[2][2].approx_eq(1.0, (0.0, 2)));
    }
}