use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Mul;

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

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Matrix {
        let mut product = new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..self.columns {
               product[row][column] = self[row][0] * other[0][column]  +
                                        self[row][1] * other[1][column]  +
                                        self[row][2] * other[2][column]  +
                                        self[row][3] * other[3][column];
            }
        }
        product
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.columns != other.columns {
            false
        }
        else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    if self[i][j] != other[i][j] {
                        return false
                    }
                }
            }
            true
        }
    }
}

impl Eq for Matrix {}

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

    #[test]
    fn matrix_equality() {
        let mut matrix1 = matrix::new(2, 2);
        let mut matrix2 = matrix::new(2, 2);
        let matrix3 = matrix::new(3, 3);

        matrix1[0][0] = 1.7;
        matrix1[1][1] = -3.1;
        matrix2[0][0] = 1.7;
        matrix2[1][1] = -3.1;
        
        assert_eq!(matrix1, matrix2);

        matrix1[0][1] = 2.0;

        assert_ne!(matrix1, matrix2);
        assert_ne!(matrix1, matrix3);
    }

    #[test]
    fn matrix_multiplication() {
        let mut matrix1 = matrix::new(4, 4);
        let mut matrix2 = matrix::new(4, 4);

        matrix1[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix1[1] = vec![5.0, 6.0, 7.0, 8.0];
        matrix1[2] = vec![9.0, 8.0, 7.0, 6.0];
        matrix1[3] = vec![5.0, 4.0, 3.0, 2.0];

        matrix2[0] = vec![-2.0, 1.0, 2.0, 3.0];
        matrix2[1] = vec![3.0, 2.0, 1.0, -1.0];
        matrix2[2] = vec![4.0, 3.0, 6.0, 5.0];
        matrix2[3] = vec![1.0, 2.0, 7.0, 8.0];

        let matrix3 = matrix1 * matrix2;

        assert_eq!(matrix3[0], vec![20.0, 22.0, 50.0, 48.0]);
        assert_eq!(matrix3[1], vec![44.0, 54.0, 114.0, 108.0]);
        assert_eq!(matrix3[2], vec![40.0, 58.0, 110.0, 102.0]);
        assert_eq!(matrix3[3], vec![16.0, 26.0, 46.0, 42.0]);
    }
}