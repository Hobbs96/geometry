use crate::float_cmp::ApproxEq;
use crate::lazy_static;
use crate::vector;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Mul;

lazy_static::lazy_static! {
    static ref IDENTITY: Matrix = {
        let identity = from_vectors(vec![vec![1.0, 0.0, 0.0, 0.0],
                                                    vec![0.0, 1.0, 0.0, 0.0],
                                                    vec![0.0, 0.0, 1.0, 0.0],
                                                    vec![0.0, 0.0, 0.0, 1.0]]);
        identity
    };
}


#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    elements: Vec<Vec<f64>>,
}

pub fn new(rows: usize, columns: usize) -> Matrix {
    if columns < 1 || rows < 1 {
        panic!("Rows and columns of a matrix both must be >= 1");
    }
    Matrix {
        rows,
        columns,
        elements: vec![vec![0.0; columns]; rows],
    }
}

pub fn from_vectors(rows: Vec<Vec<f64>>) -> Matrix {
    let n = rows.len();
   if n == 0 {
       panic!("Cannot create an empty matrix. Please supply at least a 1x1 square matrix.");
   } 
   for i in 0..n {
       if rows[i].len() != n {
           panic!("You must provide a square (n x n) matrix as an argument.");
       }
   }
   let mut matrix = new(n, n);
   for i in 0..n {
       for j in 0..n {
           matrix[i][j] = rows[i][j];
       }
   }
   let matrix = matrix;
   matrix
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
        if self.columns != other.rows {
            panic!(
                "Matrix multiplication requires that the number of rows
             in the first Matrix is equal to the number of columns in the second"
            );
        }

        let mut product = new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..other.columns {
                product[row][column] = self[row][0] * other[0][column]
                    + self[row][1] * other[1][column]
                    + self[row][2] * other[2][column]
                    + self[row][3] * other[3][column];
            }
        }
        product
    }
}

impl Mul<vector::Vector> for Matrix {
    type Output = vector::Vector;

    fn mul(self, vec: vector::Vector) -> vector::Vector {
        let mut vector_as_matrix = new(4, 1);
        vector_as_matrix[0] = vec![vec.x];
        vector_as_matrix[1] = vec![vec.y];
        vector_as_matrix[2] = vec![vec.z];
        vector_as_matrix[3] = vec![vec.w];

        let product = self * vector_as_matrix;
        if product[3][0].approx_eq(1.0, (0.0, 2)) {
            vector::build_point(product[0][0], product[1][0], product[2][0])
        } else {
            vector::build_vector(product[0][0], product[1][0], product[2][0])
        }
    }
}

impl Mul<IDENTITY> for Matrix {
    type Output = Matrix;

    fn mul(self, identity: IDENTITY) -> Matrix {
        let mut product = new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..self.columns {
                product[row][column] = self[row][0] * identity[0][column] +
                    self[row][1] * identity[1][column] +
                    self[row][2] * identity[2][column] +
                    self[row][3] * identity[3][column];
            }
        }
        product
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.columns != other.columns {
            false
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    if self[i][j] != other[i][j] {
                        return false;
                    }
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::float_cmp::ApproxEq;
    use crate::matrix;
    use crate::vector;

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
    fn new_matrix_from_vectors() {
        let matrix = matrix::from_vectors(vec![vec![1.0, 2.0, 3.0, 4.0],
                                            vec![5.0, 6.0, 7.0, 8.0],
                                            vec![9.0, 8.0, 7.0, 6.0],
                                            vec![5.0, 4.0, 3.0, 2.0]]);

        assert!(matrix[2][2].approx_eq(7.0, (0.0, 2)));
        assert!(matrix[3][1].approx_eq(4.0, (0.0, 2)));
        assert!(matrix[3][3].approx_eq(2.0, (0.0, 2)));
    }

    #[test]
    #[should_panic]
    fn new_matrix_from_non_square_vectors() {
        let matrix = matrix::from_vectors(vec![vec![1.0, 2.0, 3.0],
                                            vec![4.0, 5.0, 6.0],
                                            vec![7.0, 8.0, 9.0, 10.0]]);
        assert!(matrix.rows == matrix.columns);
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

        matrix_small[0] = vec![-3.0, 5.0];
        matrix_small[1] = vec![1.0, -2.0];

        matrix_medium[0] = vec![-3.0, 5.0, 0.0];
        matrix_medium[1] = vec![1.0, -2.0, -7.0];
        matrix_medium[2] = vec![0.0, 1.0, 1.0];

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

    #[test]
    fn matrix_vector_multiplication() {
        let mut matrix1 = matrix::new(4, 4);
        let p1 = vector::build_point(1.0, 2.0, 3.0);

        matrix1[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix1[1] = vec![2.0, 4.0, 4.0, 2.0];
        matrix1[2] = vec![8.0, 6.0, 4.0, 1.0];
        matrix1[3] = vec![0.0, 0.0, 0.0, 1.0];

        let p2 = matrix1 * p1;

        assert_eq!(p2, vector::build_point(18.0, 24.0, 33.0));
    }

    #[test]
    fn identity_matrix_multiplication() {
        let m1 = matrix::from_vectors(vec![vec![0.0, 1.0, 2.0, 4.0],
                                                vec![1.0, 2.0, 4.0, 8.0],
                                                vec![2.0, 4.0, 8.0, 16.0],
                                                vec![4.0, 8.0, 16.0, 32.0]]);
        let m2 = matrix::from_vectors(vec![vec![0.0, 1.0, 2.0, 4.0],
                                                vec![1.3, 2.1, 4.4, 8.7],
                                                vec![2.0, 7.1, 8.0, 16.0],
                                                vec![4.0, 8.0, 16.0, 32.0]]);

        let m3 = m1.clone() * matrix::IDENTITY.clone();        
        let m4 = m2.clone() * matrix::IDENTITY.clone();        

        assert_eq!(m1, m3);
        assert_eq!(m2, m4);
    }
}
