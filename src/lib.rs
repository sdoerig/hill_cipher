use std::ops::Mul;
use std::str::MatchIndices;
use std::{error::Error, fmt};

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

#[derive(Debug, Clone)]
pub struct IncompatibleMatrixError {
    error: String,
}

impl fmt::Display for IncompatibleMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Error for IncompatibleMatrixError {}
impl IncompatibleMatrixError {
    fn new(error: String) -> Self {
        IncompatibleMatrixError { error }
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
        if self.rows != rhs.cols {
            return Err(IncompatibleMatrixError::new(format!(
                "Lhs matrix has {} rows - rhs matrix has {} cols - they should be the same.",
                self.cols, rhs.rows
            )));
        }
        let mut result_matrix = Matrix::new(self.rows, rhs.cols);
        let mut sub_value: i128 = 0;

        let mut lhs_x_count = 0;
        for i in 0..self.rows {
            let lhs_sub_value = match self.matrix.get(i) {
                Some(i) => i,
                None => panic!("Ok something went terribly wrong."),
            };
        }

        Ok(result_matrix)
    }
}

impl Matrix {
    fn new(cols: usize, rows: usize) -> Self {
        Matrix {
            cols,
            rows,
            matrix: Vec::new(),
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
}
