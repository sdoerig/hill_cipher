//! Allows multiplying matrix - not more
//! as needed for the hill cipher
use std::ops::Mul;
use std::{error::Error, fmt};

/// This error is returned when an matrix is already fully
/// populated and one attempts to add an additional value.
#[derive(Debug, Clone)]
pub struct CapacityExhaustedError {
    error: String,
}

impl fmt::Display for CapacityExhaustedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Error for CapacityExhaustedError {}
impl CapacityExhaustedError {
    fn new(error: String) -> Self {
        CapacityExhaustedError { error }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    RowToColMismatch,
    NotPopulated,
}

#[derive(Debug, Clone)]
pub struct IncompatibleMatrixError {
    error_str: String,
    pub error: MatrixError,
}

impl fmt::Display for IncompatibleMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_str)
    }
}
impl Error for IncompatibleMatrixError {}
impl IncompatibleMatrixError {
    fn new(error_str: String, error: MatrixError) -> Self {
        IncompatibleMatrixError { error_str, error }
    }
}

struct Matrix {
    matrix: Vec<i128>,
    rows: usize,
    cols: usize,
    capacity: usize,
}

impl Mul<Matrix> for Matrix {
    type Output = Result<Matrix, IncompatibleMatrixError>;
    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.cols != rhs.rows {
            return Err(IncompatibleMatrixError::new(
                format!(
                    "Lhs matrix has {} rows - rhs matrix has {} cols - they should be the same.",
                    self.cols, rhs.rows
                ),
                MatrixError::RowToColMismatch,
            ));
        }
        if self.capacity != self.matrix.len() {
            return Err(IncompatibleMatrixError::new(
                "Lhs matrix is not fully populated.".to_owned(),
                MatrixError::NotPopulated,
            ));
        } else if rhs.capacity != rhs.matrix.len() {
            return Err(IncompatibleMatrixError::new(
                "Rhs matrix is not fully populated.".to_owned(),
                MatrixError::NotPopulated,
            ));
        }
        let mut result_matrix = Matrix::new(rhs.cols, self.rows);

        for r in 0..self.rows {
            let lhs_row = self.row(r);
            for c in 0..rhs.cols {
                let mut result_value: i128 = 0;
                let rhs_col = rhs.col(c);
                for lhs_col_idx in 0..lhs_row.len() {
                    result_value +=
                        lhs_row.get(lhs_col_idx).unwrap() * rhs_col.get(lhs_col_idx).unwrap()
                }
                result_matrix.add_value(result_value).unwrap();
            }
        }

        Ok(result_matrix)
    }
}

impl Matrix {
    fn new(cols: usize, rows: usize) -> Self {
        Matrix {
            cols,
            rows,
            matrix: Vec::with_capacity(cols * rows),
            capacity: cols * rows,
        }
    }

    fn add_value(&mut self, i: i128) -> Result<usize, CapacityExhaustedError> {
        if self.matrix.len() < self.capacity {
            self.matrix.push(i);
            Ok(self.capacity - self.matrix.len())
        } else {
            Err(CapacityExhaustedError {
                error: format!("Capacity of {} exhausted.", self.capacity),
            })
        }
    }

    fn row(&self, i: usize) -> Vec<i128> {
        let mut row: Vec<i128> = Vec::new();
        if i >= self.rows {
            return row;
        }
        let range_start: usize = self.cols * i;
        let range_end: usize = range_start + self.cols;

        for idx in range_start..range_end {
            row.push(*self.matrix.get(idx).unwrap());
        }
        row
    }

    fn col(&self, i: usize) -> Vec<i128> {
        let mut col: Vec<i128> = Vec::new();
        if i >= self.cols {
            return col;
        }
        let mut idx = i;
        while idx < self.matrix.len() {
            col.push(*self.matrix.get(idx).unwrap());
            idx += self.cols;
        }
        col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix() {
        let mut matrix = Matrix::new(2, 2);
        for i in 0..4 {
            match matrix.add_value(i) {
                Ok(i) => assert!(i > usize::MIN || i == usize::MIN),
                Err(_) => panic!("Must not run into an error."),
            }
        }
        match matrix.add_value(5) {
            Ok(_) => panic!("Must run into capacity error now."),
            Err(e) => assert!(format!("{e}") == "Capacity of 4 exhausted."),
        }
        //assert_eq!(result, 4);
    }
    #[test]
    fn test_row() {
        // testing a matrix like this:
        //        cols
        //       0  1  2
        // rows  3  4  5
        //       6  7  8
        //       9 10 11

        let mut matrix = Matrix::new(3, 4);
        for i in 0..12 {
            matrix.add_value(i).unwrap();
        }

        assert_eq!(matrix.row(0), vec![0, 1, 2], "Row 0 failed.");
        assert_eq!(matrix.row(1), vec![3, 4, 5], "Row 1 failed.");
        assert_eq!(matrix.row(2), vec![6, 7, 8], "Row 2 failed.");
        assert_eq!(matrix.row(3), vec![9, 10, 11], "Row 3 failed.");
        assert_eq!(matrix.row(4), vec![], "Row 4 failed.");
    }

    #[test]
    fn test_row_vector() {
        // testing a matrix like this:
        //        cols
        //       4
        // rows  5
        //       6

        let mut matrix = Matrix::new(1, 3);
        for i in &[4, 5, 6] {
            matrix.add_value(*i).unwrap();
        }
        assert_eq!(matrix.col(0), vec![4, 5, 6]);
    }

    #[test]
    fn test_col() {
        // testing a matrix like this:
        //        cols
        //       0  1  2
        // rows  3  4  5
        //       6  7  8
        //       9 10 11

        let mut matrix = Matrix::new(3, 4);
        for i in 0..12 {
            matrix.add_value(i).unwrap();
        }

        assert_eq!(matrix.col(0), vec![0, 3, 6, 9], "Col 0 failed.");
        assert_eq!(matrix.col(1), vec![1, 4, 7, 10], "Col 1 failed.");
        assert_eq!(matrix.col(2), vec![2, 5, 8, 11], "Col 2 failed.");
        assert_eq!(matrix.col(3), vec![], "Col 3 failed.");
    }

    #[test]
    fn test_multiply() {
        // Short explanation on the test below
        // Given the matrix
        // | 3 2 1 | * | 1 2 | = | 7 8 |
        // | 1 0 2 |   | 0 1 |   | 9 2 |
        //             | 4 0 |

        let mut lhs = Matrix::new(3, 2);
        for i in &[3, 2, 1, 1, 0, 2] {
            lhs.add_value(*i).unwrap();
        }

        let mut rhs = Matrix::new(2, 3);
        for i in &[1, 2, 0, 1, 4, 0] {
            rhs.add_value(*i).unwrap();
        }
        let result = match lhs * rhs {
            Ok(m) => m,
            Err(_) => Matrix::new(1, 1),
        };
        assert_eq!(result.row(0), vec![7, 8]);
        assert_eq!(result.row(1), vec![9, 2]);
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }
    #[test]
    fn test_multiply_rhs_vector() {
        // Short explanation on the test below
        // Given the matrix
        // | 1,2,3 | * | 4 | = | 32 |
        // | 3,2,1 |   | 5 |   | 28 |
        //             | 6 |

        let mut lhs = Matrix::new(3, 2);
        for i in &[1, 2, 3, 3, 2, 1] {
            lhs.add_value(*i).unwrap();
        }

        let mut rhs = Matrix::new(1, 3);
        for i in &[4, 5, 6] {
            rhs.add_value(*i).unwrap();
        }
        let result = match lhs * rhs {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(result.col(0), vec![32, 28]);
        assert_eq!(result.col(1), vec![]);
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 1);
    }
    #[test]
    fn test_multiply_lhs_square_rhs_vector() {
        // Short explanation on the test below
        // Given the matrix
        // | 1 2 3 | * | 4 | = |  32 |
        // | 3 2 1 |   | 5 |   |  28 |
        // | 6 7 8 |   | 6 |   | 107 |

        let mut lhs = Matrix::new(3, 3);
        for i in &[1, 2, 3, 3, 2, 1, 6, 7, 8] {
            lhs.add_value(*i).unwrap();
        }

        let mut rhs = Matrix::new(1, 3);
        for i in &[4, 5, 6] {
            rhs.add_value(*i).unwrap();
        }
        let result = match lhs * rhs {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(result.col(0), vec![32, 28, 107]);
        assert_eq!(result.col(1), vec![]);
        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 1);
    }

    #[test]
    fn test_multiply_incompatible_matrix() {
        // Short explanation on the test below
        // Given the matrix
        // | 1 2 3 | * | 4 | = Error
        // | 3 2 1 |   | 5 |
        // | 6 7 8 |

        let mut lhs = Matrix::new(3, 3);
        for i in &[1, 2, 3, 3, 2, 1, 6, 7, 8] {
            lhs.add_value(*i).unwrap();
        }

        let mut rhs = Matrix::new(1, 2);
        for i in &[4, 5] {
            rhs.add_value(*i).unwrap();
        }
        let result = match lhs * rhs {
            Ok(_) => panic!("Matrix are incompatible - must not return a result."),
            Err(e) => e,
        };
        assert_eq!(result.error, MatrixError::RowToColMismatch);
    }
}
